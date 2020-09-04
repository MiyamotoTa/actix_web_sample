use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

mod routes;
mod v1;

pub(crate) async fn run() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes::routes))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
