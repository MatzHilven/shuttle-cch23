pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(zero::configure);
    cfg.configure(one::configure);
    cfg.configure(four::configure);
    cfg.configure(five::configure);
    cfg.configure(six::configure);
    cfg.configure(seven::configure);
    cfg.configure(eight::configure);
    cfg.configure(twelve::configure);
    cfg.configure(eleven::configure);
    cfg.configure(thirteen::configure);
    cfg.configure(fourteen::configure);
    cfg.configure(fifteen::configure);
    cfg.configure(eighteen::configure);
    cfg.configure(nineteen::configure);
    cfg.configure(twenty::configure);
    cfg.configure(twentyone::configure);
    cfg.configure(twentytwo::configure);
}

pub mod zero;
pub mod one;
pub mod four;

pub mod five;
pub mod six;
pub mod eight;
pub mod seven;
pub mod eleven;
pub mod twelve;
pub mod thirteen;
pub mod fourteen;
pub mod fifteen;
pub mod eighteen;
pub mod nineteen;
pub mod twenty;
pub mod twentyone;
pub mod twentytwo;
