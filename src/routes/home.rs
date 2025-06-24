use actix_web::{HttpResponse, Responder, http::header::ContentType};

pub async fn home() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        r#"
{
    "name": "Invitation Server",
    "description": "This is a server for Invitation App."    
}
    "#,
    )
}
