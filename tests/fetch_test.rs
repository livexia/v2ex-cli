

#[cfg(test)]
mod tests {
    use v2ex_cli::fetch::fetch;

    #[tokio::test]
    async fn reqwest_baidu() -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://www.baidu.com";
        let resp = fetch(url).await?;
        assert_eq!(200, resp.status());
        Ok(())
    }
}