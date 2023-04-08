use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn greet_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    let body = match response.text().await {
        Ok(body) => body,
        _ => panic!(),
    };
    assert_eq!("Hello World", body);
}

#[tokio::test]
async fn custom_greet_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/maiku", address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    let body = match response.text().await {
        Ok(body) => body,
        _ => panic!(),
    };
    assert_eq!("Hello maiku", body);
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the random port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = newsletter::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
