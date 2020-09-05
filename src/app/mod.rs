mod error;
mod routes;
mod v1;

use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

pub(crate) async fn run() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::routes)
            .wrap(middleware::NormalizePath)
            .default_service(web::to(error::not_found::handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
