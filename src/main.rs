#[tokio::main]
async fn main() {
    noise_server::start_server().await;
}
