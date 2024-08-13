use reqwest::Error;

pub async fn get_public_ip() -> Result<String, Error> {
    let res = reqwest::get("https://checkip.amazonaws.com/")
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    Ok(res)
}
