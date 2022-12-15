use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::prelude::{SlackApiChatPostMessageRequest, SlackApiChatPostMessageResponse};
use slack_morphism::{SlackApiToken, SlackClient, SlackMessageContent};

use crate::slack::client::build_token;

pub async fn post_message(
    text: &str,
    channel: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    let post_chat_request = SlackApiChatPostMessageRequest::new(
        channel.into(),
        SlackMessageContent::new().with_text(text.into()),
    );

    let _post_chat_response: SlackApiChatPostMessageResponse =
        session.chat_post_message(&post_chat_request).await?;
    Ok(())
}
