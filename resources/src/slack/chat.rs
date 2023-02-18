use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::prelude::{
    SlackApiChatPostEphemeralRequest, SlackApiChatPostEphemeralResponse,
    SlackApiChatPostMessageRequest, SlackApiChatPostMessageResponse,
};
use slack_morphism::{
    SlackApiToken, SlackChannelId, SlackClient, SlackMessageContent, SlackUserId,
};

use crate::slack::client::{build_bot_token, build_user_token};

/**
 * Posts a message to the specified Slack channel.
 * See https://api.slack.com/methods/chat.postMessage for request JSON examples.
 * Parameters:
 * - text: The text of the message to post.
 * - channel: The channel to post the message to.
 * - is_bot_token: Whether to use the bot token or the user token.
 */
pub async fn post_message(
    text: &str,
    channel: &str,
    is_bot_token: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token = match is_bot_token {
        true => build_bot_token().await,
        false => build_user_token().await,
    };
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
 * Posts a reply to a specified message to the specified Slack channel.
 * See https://api.slack.com/methods/chat.postMessage for request JSON examples.
 * Parameters:
 * - text: The text of the message to post.
 * - channel: The channel to post the message to.
 * - thread_timestamp: the timestamp of the message to reply to in the specified channel.
 */
pub async fn reply_message(
    text: &str,
    channel: &str,
    thread_timestamp: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_bot_token().await;
    let session = client.open_session(&slack_token);

    let post_chat_request = SlackApiChatPostMessageRequest::new(
        channel.into(),
        SlackMessageContent::new().with_text(text.into()),
    )
    .with_thread_ts(thread_timestamp.into());

    let _post_chat_response: SlackApiChatPostMessageResponse =
        session.chat_post_message(&post_chat_request).await?;
    Ok(())
}
/**
 * Posts a message to the specified Slack channel.
 * See https://api.slack.com/methods/chat.postMessage for request JSON examples.
 * This specifically uses SlackMessageContent which can use blocks. See https://app.slack.com/block-kit-builder/T2N76FZ3Q
 * Parameters:
 * - text: The text of the message to post.
 * - channel: The channel to post the message to.
 * - is_bot_token: Whether to use the bot token or the user token.
 */
pub async fn post_message_with_content(
    content: &SlackMessageContent,
    channel: &str,
    is_bot_token: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token = match is_bot_token {
        true => build_bot_token().await,
        false => build_user_token().await,
    };
    let session = client.open_session(&slack_token);

    let post_chat_request = SlackApiChatPostMessageRequest::new(channel.into(), content.clone());

    let _post_chat_response: SlackApiChatPostMessageResponse =
        session.chat_post_message(&post_chat_request).await?;
    Ok(())
}

pub async fn _post_ephemeral_message(
    channel: &str,
    user: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_bot_token().await;
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
