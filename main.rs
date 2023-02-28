use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}



