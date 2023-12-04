pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(zero::configure);
    cfg.configure(one::configure);
    cfg.configure(four::configure);
}

pub mod four;
pub mod one;
pub mod zero;
