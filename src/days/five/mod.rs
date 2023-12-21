use actix_web::{HttpResponse, post, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1_and_2);
}

#[derive(serde::Deserialize)]
struct Params {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

#[post("/5")]
async fn part_1_and_2(
    web::Query(Params { offset, limit, split }): web::Query<Params>,
    list: web::Json<Vec<String>>,
) -> impl Responder {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(list.len());
    let split = split.unwrap_or(0);

    let list = list.into_inner();
    let list = list
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<String>>();

    if split > 0 {
        let split_result = list
            .chunks(split)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<String>>>();
        HttpResponse::Ok().json(split_result)
    } else {
        HttpResponse::Ok().json(list)
    }
}


