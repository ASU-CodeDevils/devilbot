use reqwest::StatusCode;
use serde_json::Value;
// Opens a conversation. See https://api.slack.com/methods/conversations.open for request json examples
pub async fn open_conversation(json: &Value) -> Value {
    let devil_bot_auth_token: String = crate::get_env_var("SLACK_API_BOT_TOKEN");
    let client = reqwest::Client::new();
    let channel_res = client
        .post("https://slack.com/api/conversations.open")
        .bearer_auth(&devil_bot_auth_token)
        .json(json)
        .send()
        .await
        .unwrap();
    match channel_res.status() {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };
    channel_res.json().await.unwrap()
}
