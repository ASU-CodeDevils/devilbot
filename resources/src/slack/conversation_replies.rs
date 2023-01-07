use slack_morphism::api::{
    SlackApiConversationsRepliesRequest, SlackApiConversationsRepliesResponse,
};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{SlackApiToken, SlackClient, SlackHistoryMessage};

use crate::slack::client::build_token;

pub async fn get_conversation_replies(
    channel: &str,
    timestamp: &str,
) -> Result<Vec<SlackHistoryMessage>, Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    let conversations_replies_request =
        SlackApiConversationsRepliesRequest::new(channel.into(), timestamp.into());

    let conversations_replies_response: SlackApiConversationsRepliesResponse = session
        .conversations_replies(&conversations_replies_request)
        .await?;
    Ok(conversations_replies_response.messages)
}
