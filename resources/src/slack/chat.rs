use slack_morphism::api::{
    SlackApiChatPostEphemeralRequest, SlackApiChatPostEphemeralResponse,
    SlackApiChatPostMessageRequest, SlackApiChatPostMessageResponse,
};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{
    SlackApiToken, SlackChannelId, SlackClient, SlackMessageContent, SlackUserId,
};

use crate::slack::client::build_token;

/**
 * Posts a message to the specified Slack channel.
 * See https://api.slack.com/methods/chat.postMessage for request JSON examples.
 */
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

/**
 * Posts an ephemeral message to the specified Slack channel.
 * See https://api.slack.com/methods/chat.postEphemeral for request JSON examples.
 */
pub async fn post_ephemeral_message(
    channel: &str,
    user: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    // Create params
    let message_content = SlackMessageContent::new().with_text(text.into());
    let slack_channel_id = SlackChannelId(channel.to_string());
    let slack_user_id = SlackUserId(user.to_string());

    log::info!(
        "Chat post ephemeral message params: message_content {:?},\
         slack_channel_id {:?} slack_user_id {:?}",
        message_content,
        slack_channel_id,
        slack_user_id
    );

    let chat_post_ephemeral_message_request: SlackApiChatPostEphemeralRequest =
        SlackApiChatPostEphemeralRequest::new(slack_channel_id, slack_user_id, message_content);

    let _chat_post_ephemeral_message_response: SlackApiChatPostEphemeralResponse = session
        .chat_post_ephemeral(&chat_post_ephemeral_message_request)
        .await?;

    Ok(())
}
