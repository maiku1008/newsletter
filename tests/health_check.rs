use newsletter::{
    configuration::{self, DatabaseSettings},
    startup,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn greet_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", app.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    let body = response.text().await;
    assert!(body.is_ok());
    assert_eq!("Hello World", body.unwrap());
}

#[tokio::test]
async fn custom_greet_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/maiku", app.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    let body = response.text().await;
    assert!(body.is_ok());
    assert_eq!("Hello maiku", body.unwrap());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch save subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub db_name: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the random port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let db_name = Uuid::new_v4().to_string();

    let mut config = configuration::get_configuration().expect("Failed to get configuration");
    config.database.database_name = db_name.clone();

    let connection_pool = configure_database(&config.database).await;

    let server = startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
        db_name,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect(&config.conn_string_no_db())
        .await
        .expect("failed to connect to postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database");

    // Migrate database
    let connection_pool = PgPool::connect(&config.conn_string())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate the database");
    connection_pool
}

// async fn teardown(app: &TestApp) -> PgQueryResult {
//     let mut connection = PgConnection::connect(&app.database.conn_string_no_db())
//         .await
//         .expect("failed to connect to postgres.");
//     connection
//         .execute(format!(r#"DROP DATABASE "{}";"#, app.database_name).as_str())
//         .await
//         .expect("failed to drop database")
// }
