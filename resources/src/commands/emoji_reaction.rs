use reqwest::StatusCode;
use serde_json::Value;
// Sends an emoji reaction to the passed in json
pub async fn reactions_add(json: &Value) -> Value {
    let devil_bot_auth_token: String = crate::get_env_var("SLACK_API_BOT_TOKEN");
    let client = reqwest::Client::new();
    let emoji_response = client
        .post("https://slack.com/api/reactions.add")
        .bearer_auth(devil_bot_auth_token)
        .json(json)
        .send()
        .await
        .unwrap();
    match emoji_response.status() {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };
    emoji_response.json().await.unwrap()
}
