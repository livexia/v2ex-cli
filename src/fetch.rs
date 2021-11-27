use reqwest::Response;

pub async fn fetch(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7890")?)
        .build()?;
    let resp = client.get(url)
        .send()
        .await?;
    Ok(resp)
}