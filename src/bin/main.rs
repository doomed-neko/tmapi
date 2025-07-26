use tmapi::Client;
#[tokio::main]
async fn main() {
    let cl = Client::new("y@iusearch.lol").unwrap();
    let status = cl.server_health().await;
    let _ = dbg!(status);
}
