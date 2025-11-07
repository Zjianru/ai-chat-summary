#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cli boot");
    monitoring::serve("127.0.0.1:4188").await
}
