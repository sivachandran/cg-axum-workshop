mod greeting;

use greeting::greet;

#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("{}", greet("Chennai Geeks"));
}
