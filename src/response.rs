#[derive(serde::Serialize)]
pub struct ResponseBody<T, E> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<E>,
}
