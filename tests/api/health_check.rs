use crate::helper::TestApp;

#[tokio::test]
async fn health_check_test() {
    let TestApp { ref address, .. } = TestApp::spawn().await;

    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to GET health_check");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}
