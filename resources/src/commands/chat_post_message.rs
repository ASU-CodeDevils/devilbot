use reqwest::StatusCode;
use serde_json::Value;
pub async fn post_message(json: &Value) {
    // // TODO Update post_message to use slack_morphism - example below.
    // let client = SlackClient::new(SlackClientHyperConnector::new());
    // let slack_token: SlackApiToken = build_token().await;
    // let session = client.open_session(&slack_token);

    // let post_chat_request = SlackApiChatPostMessageRequest::new(
    //     channel.into(),
    //     SlackMessageContent::new().with_text("You are :buns:".into()),
    // );

    // let post_chat_response = session.chat_post_message(&post_chat_request).await?;

    log::info!("JSON Value for onboard message {} ", json);
    let devil_bot_auth_token: String = crate::get_env_var("SLACK_API_BOT_TOKEN");
    let client = reqwest::Client::new();
    let message_res = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(&devil_bot_auth_token)
        .json(json)
        .send()
        .await
        .unwrap();
    log::info!("Chat Post Message Response: {:?}", message_res);
    match message_res.status() {
        StatusCode::OK => println!("Message sent success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    }
    message_res.status();
}
