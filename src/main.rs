use actix_web::{web, App, HttpServer, HttpResponse};

async fn handle_post(data: web::Json<serde_json::Value>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/post", web::post().to(handle_post))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
