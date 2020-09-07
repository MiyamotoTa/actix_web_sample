#[macro_use]
extern crate diesel;

mod app;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
