use reqwest::redirect::Policy;
use reqwest::Client;
use uuid::Uuid;

#[tokio::test]
async fn when_a_user_registers_they_should_then_be_able_to_login() {
    let id = Uuid::new_v4();
    let email_under_test = format!("{}@test.com", id);

    let api_endpoint = retrieve_api_endpoint().await;

    let http_client = Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .redirect(Policy::none())
        .build()
        .unwrap();
    

    let result = http_client
        .post(format!("{}users", api_endpoint))
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"emailAddress": email_under_test, "password": "Testing!23", "name": "James"}).to_string())
        .send()
        .await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status(), 201);

    let login_response = http_client
        .post(format!("{}login", api_endpoint))
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"emailAddress": email_under_test, "password": "Testing!23"}).to_string())
        .send()
        .await;

    assert_eq!(login_response.unwrap().status(), 200);
}
async fn retrieve_api_endpoint() -> String {
    // You could write code here to dynamically retrieve the API endpoint from your environment or configuration.

    "http://localhost:3000/".to_string()
}
