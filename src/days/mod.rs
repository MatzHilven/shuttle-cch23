pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(zero::configure);
    cfg.configure(one::configure);
    cfg.configure(four::configure);
    cfg.configure(six::configure);
}

pub mod zero;
pub mod one;
pub mod four;

pub mod six;
