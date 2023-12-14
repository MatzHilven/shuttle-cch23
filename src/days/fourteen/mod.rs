use actix_web::{post, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

#[derive(serde::Deserialize)]
struct Content {
    content: String,
}

fn template(content: &str) -> String {
    format!(
        r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {content}
  </body>
</html>"
    )
}

#[post("/14/unsafe")]
async fn part_1(
    content: web::Json<Content>
) -> impl Responder {
    let content = content.into_inner().content;

    template(&content)
}

#[post("/14/safe")]
async fn part_2(
    content: web::Json<Content>
) -> impl Responder {
    let content = content.into_inner().content
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;");

    template(&content)
}
