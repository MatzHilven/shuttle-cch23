use actix_web::{get, Responder, web};
use google_maps::{GoogleMapsClient, LatLng, PlaceType};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

fn get_coords(cell_id: u64) -> (f64, f64) {
    let cell_id = s2::cellid::CellID(cell_id);
    let cell = s2::cell::Cell::from(cell_id);

    let center = cell.center();
    let latitude = center.latitude().deg();
    let longitude = center.longitude().deg();
    (latitude, longitude)
}

#[get("/21/coords/{binary}")]
async fn part_1(
    binary: web::Path<String>,
) -> impl Responder {
    let cell_id = u64::from_str_radix(&binary, 2).unwrap();
    let (latitude, longitude) = get_coords(cell_id);
    let latitude = format!("{}°{}'{:.3}''{}",
                           latitude.abs() as u8,
                           (latitude.fract() * 60.0).abs() as u8,
                           ((latitude.fract() * 60.0).fract() * 60.0).abs(),
                           if latitude > 0.0 { "N" } else { "S" }
    );
    let longitude = format!("{}°{}'{:.3}''{}",
                            longitude.abs() as u8,
                            (longitude.fract() * 60.0).abs() as u8,
                            ((longitude.fract() * 60.0).fract() * 60.0).abs(),
                            if longitude > 0.0 { "E" } else { "W" });

    format!("{} {}", latitude, longitude)
}

#[get("/21/country/{binary}")]
async fn part_2(
    binary: web::Path<String>,
) -> impl Responder {
    dotenv::dotenv().ok();
    let api_key = match std::env::var("GOOGLE_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("No API key found")
    };
    let google_maps_client = GoogleMapsClient::new(api_key.as_str());

    let cell_id = u64::from_str_radix(&binary, 2).unwrap();
    let (latitude, longitude) = get_coords(cell_id);

    let location = google_maps_client.reverse_geocoding(
        LatLng::try_from_f64(latitude, longitude).unwrap()
    )
        .with_result_type(PlaceType::Country)
        .execute()
        .await
        .unwrap();
    location.results[0].address_components[0].long_name.clone()
}
