use actix_web::http::header::ContentType;

#[derive(serde::Serialize)]
pub struct ResponseBody<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

pub fn e500(e: anyhow::Error) -> actix_web::HttpResponse {
    let response_body: ResponseBody<()> = ResponseBody {
        success: false,
        message: e.to_string(),
        data: None,
    };
    actix_web::HttpResponse::InternalServerError()
        .content_type(ContentType::json())
        .json(response_body)
}
