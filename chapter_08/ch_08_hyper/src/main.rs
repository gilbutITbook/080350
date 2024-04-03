use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let mut res = client.get("http://www.google.com".parse::<hyper::Uri>().unwrap()).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    while let Some(next) = res.data().await {
    let chunk = next?;
    io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
