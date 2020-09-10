use actix_web::web;

use crate::app::v1::articles::handler::articles_handler;
use crate::app::v1::users::handler::users_handler;

pub(crate) fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("api").service(
            web::scope("v1")
                .service(
                    web::scope("users")
                        .service(users_handler::create)
                        .service(users_handler::find_by_id),
                )
                .service(
                    web::scope("articles")
                        .service(web::resource("/").route(web::get().to(articles_handler::index)))
                        .service(
                            web::resource("{author}/{title}/")
                                .route(web::get().to(articles_handler::get_article)),
                        ),
                ),
        ),
    );
}
