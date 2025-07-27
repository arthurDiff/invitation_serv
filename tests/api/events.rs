use invite_server::{models::Event, response::ResponseBody};

use crate::helper::{SESSION_TOKEN, TestApp};

#[tokio::test]
async fn create_event_successfully_test() {
    let TestApp { ref address, .. } = TestApp::spawn().await;

    let client = reqwest::Client::new();

    let res = client
        .post(format!("{address}/api/events"))
        .header(
            "Authorization",
            format!("Bearer {}", SESSION_TOKEN.get().expect("Missing SESSION_TOKEN")),
        )
        .json(&serde_json::json!({
            "name" : "this is test event",
            "description": "And goodold description",
            "budget": 150,
            "starts_at": "1998-09-05"
        }))
        .send()
        .await
        .expect("Failed to send requet to post | events endpoint");

    assert_eq!(res.status().as_u16(), 200);
    let body: ResponseBody<Event> = res.json().await.expect("Failed to deserialize event body");
    assert_eq!(body.data.name, "this is test event");
    assert_eq!(body.data.description, Some("And goodold description".into()));
}
