use actix_web::web;

use crate::app::v1::articles::handler::articles_handler;

pub(crate) fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api")
            // articles
            .service(web::resource("articles").route(web::get().to(articles_handler::index))),
    );
}
