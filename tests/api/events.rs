use crate::helper::{SESSION_TOKEN, TestApp};

#[tokio::test]
async fn create_event_test() {
    let TestApp { ref address, .. } = TestApp::spawn().await;

    let client = reqwest::Client::new();

    let _ = client
        .post(format!("{address}/events"))
        .header(
            "Authorization",
            format!("Bearer {}", SESSION_TOKEN.get().expect("Missing SESSION_TOKEN")),
        )
        .json(&serde_json::json!({
            "name" : "this is test event",
            "description": "And goodold description",
            "budget": 150,
            "starts_at": "1998-09-05"
        }));
}
