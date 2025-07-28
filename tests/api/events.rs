use std::str::FromStr;

use invite_server::{models::Event, response::ResponseBody};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::helper::{SESSION_TOKEN, TestApp};

#[tokio::test]
async fn create_event_successfully_test() {
    let TestApp {
        ref address, db_pool, ..
    } = TestApp::spawn().await;

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

    // verify res body containing event obj
    assert_eq!(res.status().as_u16(), 200);
    let body: ResponseBody<Event> = res.json().await.expect("Failed to deserialize event body");
    assert_eq!(body.data.name, "this is test event");
    assert_eq!(body.data.description, Some("And goodold description".into()));

    // verify db has one entry
    let query: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM event"#)
        .fetch_one(&db_pool)
        .await
        .expect("Failed executing query to get event count");

    assert_eq!(query, 1);
}

#[tokio::test]
async fn should_return_cached_result_for_same_idem_key() {
    let TestApp {
        ref address, db_pool, ..
    } = TestApp::spawn().await;

    let client = reqwest::Client::new();

    let idem_key = "idem-key-here";

    let res_1 = client
        .post(format!("{address}/api/events"))
        .headers(create_req_header(idem_key))
        .json(&serde_json::json!({
            "name" : "this is test event",
            "description": "And goodold description",
            "budget": 150,
        }))
        .send()
        .await
        .expect("Failed to send requet to post | events endpoint");

    let res_2 = client
        .post(format!("{address}/api/events"))
        .headers(create_req_header(idem_key))
        .json(&serde_json::json!({
            "name" : "this is test event",
            "description": "And goodold description",
            "budget": 150,
        }))
        .send()
        .await
        .expect("Failed to send requet to post | events endpoint");

    assert_eq!(res_1.status().as_u16(), 200);
    assert_eq!(res_2.status().as_u16(), 200);

    // verify db has one entry
    let query: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM event"#)
        .fetch_one(&db_pool)
        .await
        .expect("Failed executing query to get event count");

    assert_eq!(query, 1);
}

fn create_req_header(idem_key: &str) -> HeaderMap {
    let mut hm = HeaderMap::with_capacity(2);

    hm.insert(
        HeaderName::from_str("Authorization").expect("Failed creating header name"),
        HeaderValue::from_str(&format!(
            "Bearer {}",
            SESSION_TOKEN.get().expect("Missing SESSION_TOKEN")
        ))
        .expect("Failed creating header value"),
    );

    hm.insert(
        HeaderName::from_str("idempotency-key").expect("Failed creating header name"),
        HeaderValue::from_str(idem_key).expect("Failed creating header value"),
    );

    hm
}
