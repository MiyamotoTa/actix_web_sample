mod error;
mod routes;
mod v1;

use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use dotenv::dotenv;
use env_logger::Env;
use std::env;

pub(crate) struct AppState {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl AppState {
    fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set...");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool...");
        AppState { pool }
    }
}

pub(crate) async fn run() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .data(AppState::new())
            .wrap(Logger::default())
            .configure(routes::routes)
            .wrap(middleware::NormalizePath)
            .default_service(web::to(error::not_found::handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
