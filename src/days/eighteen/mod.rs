use actix_web::{get, HttpResponse, post, Responder, web};
use sqlx::PgPool;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1_reset);
    cfg.service(part_1_orders);
    cfg.service(part_1_regions);
    cfg.service(part_1_regions_total);
    cfg.service(part_1_regions_top_list);
}

#[post("/18/reset")]
async fn part_1_reset(
    pool: web::Data<PgPool>
) -> impl Responder {
    sqlx::query!("DROP TABLE IF EXISTS regions").execute(pool.as_ref()).await.unwrap();
    sqlx::query!("DROP TABLE IF EXISTS orders").execute(pool.as_ref()).await.unwrap();

    sqlx::query!("CREATE TABLE regions (
        id INT PRIMARY KEY,
        name VARCHAR(50)
    )")
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

#[derive(serde::Deserialize)]
struct Region {
    id: i32,
    name: String,
}

#[post("/18/orders")]
async fn part_1_orders(
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

#[post("/18/regions")]
async fn part_1_regions(
    regions: web::Json<Vec<Region>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    for region in regions.iter() {
        sqlx::query!(
            "INSERT INTO regions (id, name) VALUES ($1, $2)",
            region.id,
            region.name
        )
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Ok()
}

#[derive(serde::Serialize)]
struct RegionTotal {
    region: String,
    total: i64,
}

#[get("/18/regions/total")]
async fn part_1_regions_total(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let totals = sqlx::query!(
        "SELECT regions.name region, SUM(orders.quantity) total FROM orders JOIN regions ON orders.region_id = regions.id GROUP BY regions.name ORDER BY regions.name"
    )
        .fetch_all(pool.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|row| RegionTotal {
            region: row.region.unwrap(),
            total: row.total.unwrap(),
        })
        .collect::<Vec<RegionTotal>>();

    HttpResponse::Ok().json(totals)
}

struct TopGift {
    gift_name: String,
}

#[derive(serde::Serialize)]
struct RegionTopList {
    region: String,
    top_gifts: Vec<String>,
}

#[get("/18/regions/top_list/{number}")]
async fn part_1_regions_top_list(
    pool: web::Data<PgPool>,
    number: web::Path<i64>,
) -> impl Responder {
    let number = number.into_inner();

    let region = sqlx::query!("SELECT id, name \"name!\" FROM regions")
        .fetch_all(pool.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|row| Region {
            id: row.id,
            name: row.name,
        })
        .collect::<Vec<Region>>();

    let mut regions_top_list = Vec::new();
    for region in region {
        let top_gifts = sqlx::query!(
            "SELECT
                gift_name \"gift_name!\"
            FROM
                orders
            WHERE
                region_id = $1
            GROUP BY
                gift_name
            ORDER BY
                SUM(quantity) DESC, gift_name
            LIMIT $2",
            region.id,
            number
        )
            .fetch_all(pool.as_ref())
            .await
            .unwrap()
            .into_iter()
            .map(|row| TopGift {
                gift_name: row.gift_name,
            })
            .collect::<Vec<TopGift>>();

        regions_top_list.push(RegionTopList {
            region: region.name,
            top_gifts: top_gifts.into_iter().map(|gift| gift.gift_name).collect(),
        });
    }
    regions_top_list.sort_unstable_by_key(|r| r.region.clone());

    HttpResponse::Ok().json(regions_top_list)
}
