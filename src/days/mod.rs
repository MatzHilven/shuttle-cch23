pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(zero::configure);
    cfg.configure(one::configure);
}

pub mod one;
pub mod zero;
