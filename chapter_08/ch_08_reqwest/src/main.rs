use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "*****")
        .body("a list with shit words")
        .send()
        .await?;

    let status_code = res.status();
    let message = res.text().await?;

    let response = json!({
    "StatusCode": status_code.as_str(),
    "Message": message
    });

    println!("{:#?}", response);

    Ok(())
}
