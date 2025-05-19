use std::time::Duration;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
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
        .body(
            serde_json::json!({"emailAddress": email_under_test, "password": "Testing!23"})
                .to_string(),
        )
        .send()
        .await;

    assert_eq!(login_response.unwrap().status(), 200);
}

#[tokio::test]
async fn inject_kafka_message() {
    produce_event()
        .await;
}

async fn retrieve_api_endpoint() -> String {
    // You could write code here to dynamically retrieve the API endpoint from your environment or configuration.

    "http://localhost:3000/".to_string()
}

async fn produce_event() {
    let broker = "localhost:9092";

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .create()
        .expect("Producer creation failed");

    let res = producer
        .send(
            FutureRecord::to("order-completed")
                .payload("hello ")
                .key("mykey"),
            Duration::from_secs(0),
        )
        .await;

    match res {
        Ok(_) => println!("Publish succeeded"),
        Err((e, _)) => println!("Kafka publish failed: {}", e),
    }
}
