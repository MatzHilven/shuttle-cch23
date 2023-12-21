use actix_web::{post, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {}


#[post("/6")]
async fn part_1(string: String) -> impl Responder {
    string
}

#[post("/6")]
async fn part_2(string: String) -> impl Responder {
    string
}
