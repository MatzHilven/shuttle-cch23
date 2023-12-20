use std::collections::HashMap;
use std::sync::atomic::AtomicI64;

use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use actix_ws::Message;
use futures::StreamExt as _;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(AppData {
        views: Default::default(),
        rooms: Mutex::new(HashMap::new()),
    }));
    cfg.service(ws);
    cfg.service(room_ws);
    cfg.service(reset);
    cfg.service(views);
}

#[get("/19/ws/ping")]
async fn ws(
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_rt::spawn(async move {
        let mut started = false;
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(s) => {
                    if s == "serve" {
                        started = true;
                    } else if s == "ping" && started {
                        let _ = session.text("pong").await;
                    }
                }
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}


#[post("/19/reset")]
async fn reset(app_data: web::Data<AppData>) -> HttpResponse {
    app_data.views.store(0, std::sync::atomic::Ordering::SeqCst);
    HttpResponse::Ok().finish()
}

#[get("/19/views")]
async fn views(app_data: web::Data<AppData>) -> HttpResponse {
    HttpResponse::Ok().json(app_data.views.load(std::sync::atomic::Ordering::SeqCst))
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct TMessage {
    user: String,
    message: String,
}

#[derive(serde::Deserialize)]
struct SentMessage {
    message: String,
}

#[derive(Debug)]
pub struct AppData {
    pub views: AtomicI64,
    pub rooms: Mutex<HashMap<i64, Sender<TMessage>>>,
}

#[get("/19/ws/room/{number}/user/{user}")]
async fn room_ws(
    req: HttpRequest,
    body: web::Payload,
    path: web::Path<(i64, String)>,
    app_data: web::Data<AppData>,
)
    -> Result<HttpResponse, Error> {
    let (room_id, username) = path.into_inner();
    let mut rooms = app_data.rooms.lock().await;
    let sender = match rooms.get(&room_id) {
        Some(s) => s.clone(),
        None => {
            let sender = Sender::new(1024);
            let sender_clone = sender.clone();
            rooms.insert(room_id, sender);
            sender_clone
        }
    };
    drop(rooms);
    let mut rx = sender.subscribe();

    let (response, og_session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let mut session = og_session.clone();
    actix_web::rt::spawn(async move {
        let mut rx_task = actix_web::rt::spawn(async move {
            while let Some(Ok(msg)) = msg_stream.recv().await {
                match msg {
                    Message::Ping(bytes) => {
                        if session.pong(&bytes).await.is_err() {
                            return;
                        }
                    }
                    Message::Text(msg) => {
                        if let Ok(tweet) = serde_json::from_str::<SentMessage>(&msg) {
                            if tweet.message.len() <= 128 {
                                let _ = sender.send(TMessage { user: username.clone(), message: tweet.message });
                            }
                        }
                    }
                    Message::Close(_) => {
                        break;
                    }
                    _ => {}
                }
            }

            let _ = session.close(None).await;
        });

        let mut session = og_session.clone();
        let mut tx_task = actix_web::rt::spawn(async move {
            while let Ok(tweet) = rx.recv().await {
                if session.text(serde_json::to_string(&tweet).unwrap()).await.is_err() {
                    return;
                } else {
                    app_data.views.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });

        tokio::select! {
            _ = (&mut tx_task) => rx_task.abort(),
            _ = (&mut rx_task) => tx_task.abort()
        }
    });

    Ok(response)
}
