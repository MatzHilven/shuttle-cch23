use actix_web::{get, HttpResponse, post, Responder, web};
use sqlx::PgPool;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2_reset);
    cfg.service(part_2_orders);
    cfg.service(part_2_orders_total);
    cfg.service(part_3);
}

#[get("/13/sql")]
async fn part_1(
    pool: web::Data<PgPool>
) -> impl Responder {
    sqlx::query!("SELECT 20231213 number")
        .fetch_one(pool.as_ref())
        .await
        .unwrap()
        .number
        .unwrap()
        .to_string()
}

#[post("/13/reset")]
async fn part_2_reset(
    pool: web::Data<PgPool>
) -> impl Responder {
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(pool.as_ref())
        .await
        .unwrap();

    sqlx::query!("CREATE TABLE orders (
        id INT PRIMARY KEY,
        region_id INT,
        gift_name VARCHAR(50),
        quantity INT
    )")
        .execute(pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[post("/13/orders")]
async fn part_2_orders(
    orders: web::Json<Vec<Order>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    for order in orders.iter() {
        sqlx::query!(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity
        )
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Ok()
}

#[get("/13/orders/total")]
async fn part_2_orders_total(
    pool: web::Data<PgPool>
) -> impl Responder {
    let total = sqlx::query!("SELECT SUM(quantity) total FROM orders")
        .fetch_one(pool.as_ref())
        .await
        .unwrap()
        .total
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({ "total": total }))
}

#[get("/13/orders/popular")]
async fn part_3(
    pool: web::Data<PgPool>
) -> impl Responder {
    let popular = sqlx::query!("SELECT gift_name, SUM(quantity) total FROM orders GROUP BY gift_name ORDER BY total DESC LIMIT 1")
        .fetch_one(pool.as_ref())
        .await;


    return HttpResponse::Ok().json(match popular {
        Err(_) => serde_json::json!({ "popular" : null }),
        Ok(popular) =>serde_json::json!( { "popular": popular.gift_name })
    })
}
