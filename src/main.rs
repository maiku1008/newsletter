use newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    // Return the error if we failed to bind the address
    // otherwise call .await on the server
    run(listener)?.await
}
