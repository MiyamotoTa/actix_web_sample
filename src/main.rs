mod app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
