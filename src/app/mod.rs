use std::env;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use slog::{o, Drain, Logger};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

mod config;
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
    pool: Pool<MySql>,
    log: Logger,
}

pub(crate) async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::app::config::Config::from_env().unwrap();

    // Access Log
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
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
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
