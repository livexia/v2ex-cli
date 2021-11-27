use v2ex_cli::fetch::fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let test_url = "https://baidu.com";
    fetch(test_url).await?;
    println!("Hello, world!");
    Ok(())
}
