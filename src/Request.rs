use anyhow::Result;
use reqwest;

pub async fn get_request(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User_Agen", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36")
        .send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        return Ok(body);
    } else {
        return Ok("Request Error".to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn it_work_Request() {
        let url = "www.google.com";
        let response = get_request(url).await;
        println!("{:?}", response);
        assert!(response.is_ok());
    }
}
