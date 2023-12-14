pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(zero::configure);
    cfg.configure(one::configure);
    cfg.configure(four::configure);
    cfg.configure(six::configure);
    cfg.configure(seven::configure);
    cfg.configure(eight::configure);
    cfg.configure(twelve::configure);
    cfg.configure(eleven::configure);
    cfg.configure(thirteen::configure);
    cfg.configure(fourteen::configure);
}

pub mod four;
pub mod one;
pub mod zero;

pub mod eight;
pub mod seven;
pub mod six;
pub mod eleven;
pub mod twelve;
pub mod thirteen;
pub mod fourteen;
