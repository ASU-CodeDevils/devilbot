use slack_morphism::{SlackApiToken, SlackApiTokenValue};

use crate::get_env_var;

/**
 * Gets your Slack API token string from the environmental variables set in
 * devil-bot-rust-cdk-stack.ts and creates a SlackApiToken object for use in
 * a slack_morphism client.
 */
pub async fn build_bot_token() -> SlackApiToken {
    const SLACK_API_BOT_TOKEN: &str = "SLACK_API_BOT_TOKEN";
    let slack_token_string: String = get_env_var(SLACK_API_BOT_TOKEN);
    let token_value: SlackApiTokenValue = slack_token_string.into();

    SlackApiToken::new(token_value)
}
/**
 * Gets your Slack API token string for the user who installed the app
 * from the environmental variables set in
 * devil-bot-rust-cdk-stack.ts and creates a SlackApiToken object for use in
 * a slack_morphism client.
*/
pub async fn build_user_token() -> SlackApiToken {
    const SLACK_API_USER_TOKEN: &str = "SLACK_API_USER_TOKEN";
    let slack_token_string: String = get_env_var(SLACK_API_USER_TOKEN);
    let token_value: SlackApiTokenValue = slack_token_string.into();

    SlackApiToken::new(token_value)
}
