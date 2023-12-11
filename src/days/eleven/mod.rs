use std::io::BufReader;
use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, HttpResponse, post, Responder, web};
use image::GenericImageView;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

#[get("/11/assets/decoration.png")]
async fn part_1() -> impl Responder {
    NamedFile::open("assets/decoration.png")
}

#[derive(MultipartForm)]
struct File {
    image: TempFile,
}

#[post("/11/red_pixels")]
async fn part_2(MultipartForm(file): MultipartForm<File>) -> impl Responder {
    let reader = BufReader::new(&file.image.file);
    let image = image::load(reader, image::ImageFormat::Png).unwrap();
    let mut count = 0;

    for pixel in image.pixels() {
        let pixel = pixel.2;
        if pixel[0] as u16 > pixel[1] as u16 + pixel[2] as u16 {
            count += 1;
        }
    }

    HttpResponse::Ok().body(format!("{}", count))
}

#[cfg(test)]
mod tests {
    use actix_web::{body, test, App};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::get().uri("/11/assets/decoration.png").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::post().uri("/11/red_pixels").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        assert_eq!(bytes, "73034");
    }
}
