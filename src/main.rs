use actix_web::{App, HttpRequest, HttpServer, web};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(async |req: HttpRequest| {
                    let name = req.match_info().get("name").unwrap_or("World");
                    format!("Hello {}!", &name)
                }),
            )
            .route(
                "/{name}",
                web::get().to(async |req: HttpRequest| {
                    let name = req.match_info().get("name").unwrap_or("World");
                    format!("Hello {}!", &name)
                }),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
