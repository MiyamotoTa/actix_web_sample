use actix_web::{App, HttpServer};

mod routes;
mod v1;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
