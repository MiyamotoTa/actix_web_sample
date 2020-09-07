use std::env;

use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use dotenv::dotenv;
use env_logger::Env;
use slog::{o, Drain, Logger};
use slog_term;

mod error;
mod routes;
mod v1;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v"=>env!("CARGO_PKG_VERSION")))
}

pub(crate) struct AppState {
    pool: Pool<ConnectionManager<MysqlConnection>>,
    log: Logger,
}

pub(crate) async fn run() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set...");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool...");

    let log = configure_log();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
            .wrap(middleware::Logger::default())
            .configure(routes::routes)
            .wrap(middleware::NormalizePath)
            .default_service(web::to(error::not_found::handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
