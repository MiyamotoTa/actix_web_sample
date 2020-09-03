use actix_web::web;

use crate::app::handlers::articles::articles_handler;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api")
            // articles
            .service(web::resource("articles").route(web::get().to(articles_handler::index))),
    );
}
