use std::str::FromStr;

use actix_web::{HttpResponse, post, Responder, web};
use serde_json::json;
use unicode_segmentation::UnicodeSegmentation;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

#[derive(serde::Deserialize)]
struct Content {
    input: String,
}

#[post("/15/nice")]
async fn part_1(
    content: web::Json<Content>
) -> impl Responder {
    let password = content.input.clone();

    let re = fancy_regex::Regex::new(r"^(?=.*[aeiouy].*[aeiouy].*[aeiouy])(?=.*([a-z])\1)(?!.*(?:ab|cd|pq|xy)).*$").unwrap();

    return match re.is_match(&password).unwrap() {
        true => HttpResponse::Ok().json(json!({"result": "nice"})),
        false => HttpResponse::BadRequest().json(json!({"result": "naughty"})),
    };
}

fn check_rules(password: String) -> Option<i32> {
    // Rule 1: must be at least 8 characters long
    if password.len() < 8 {
        return Some(1);
    }

    // Rule 2: must contain uppercase letters, lowercase letters, and digits
    let re = fancy_regex::Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).+$").unwrap();
    if !re.is_match(&password).unwrap() {
        return Some(2);
    }
    // Rule 3: must contain at least 5 digits
    let re = fancy_regex::Regex::new(r"^(.*\d.*){5,}$").unwrap();
    if !re.is_match(&password).unwrap() {
        return Some(3);
    }
    // Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    let re = fancy_regex::Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for mat in re.find_iter(password.as_str()) {
        sum += i32::from_str(mat.unwrap().as_str()).unwrap();
    }

    if sum != 2023 {
        return Some(4);
    }

    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    // get index of j, o, and y, if not present return 5 don't unwrap
    let j = match password.find("j") {
        Some(i) => i,
        None => return Some(5),
    };
    let o = match password.find("o") {
        Some(i) => i,
        None => return Some(5),
    };
    let y = match password.find("y") {
        Some(i) => i,
        None => return Some(5),
    };
    let re = fancy_regex::Regex::new(r"j.+o.+y").unwrap();
    if !re.is_match(&password).unwrap() {
        return Some(5);
    }

    if j > o || o > y {
        return Some(5);
    }
    // Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    let re = fancy_regex::Regex::new(r"([a-zA-Z]).\1").unwrap();
    if !re.is_match(&password).unwrap() {
        return Some(6);
    }
    // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    if !password.chars().any(|c| ('\u{2980}'..='\u{2BFF}').contains(&c)) {
        return Some(7);
    }
    // Rule 8: must contain at least one emoji

    if !password.graphemes(true).any(|el| emojis::get(el).is_some()) {
        return Some(8);
    }

    // Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an 'a'
    if !sha256::digest(password).ends_with("a") {
        return Some(9);
    }

    None
}

#[post("/15/game")]
async fn part_2(
    content: web::Json<Content>
) -> impl Responder {
    let password = content.input.clone();
    return match check_rules(password) {
        Some(i) => match i {
            1 => HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "8 chars"})),
            2 => HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "more types of chars"})),
            3 => HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "55555"})),
            4 => HttpResponse::BadRequest().json(json!({"result": "naughty", "reason": "math is hard"})),
            5 => HttpResponse::NotAcceptable().json(json!({"result": "naughty", "reason": "not joyful enough"})),
            6 => HttpResponse::UnavailableForLegalReasons().json(json!({"result": "naughty", "reason": "illegal: no sandwich"})),
            7 => HttpResponse::RangeNotSatisfiable().json(json!({"result": "naughty", "reason": "outranged"})),
            8 => HttpResponse::UpgradeRequired().json(json!({"result": "naughty", "reason": "😳"})),
            9 => HttpResponse::ImATeapot().json(json!({"result": "naughty", "reason": "not a coffee brewer"})),
            _ => HttpResponse::InternalServerError().json(json!({"result": "naughty", "reason": "unknown"})),
        }
        None => HttpResponse::Ok().json(json!({"result": "nice", "reason": "that's a nice password"})),
    };
}
