use actix_cors::Cors;
use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};

pub mod chat_connectivity;
pub mod chat_users;
pub mod models;
pub mod schema;
#[get("/test")]
pub async fn testing_if_it_works() -> impl Responder {
    HttpResponse::Ok().body("This is just a test to see if the api works")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(testing_if_it_works)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
