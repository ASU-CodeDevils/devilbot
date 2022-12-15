use slack_morphism::prelude::*;

use crate::get_env_var;

pub async fn build_token() -> SlackApiToken {
    const SLACK_API_BOT_TOKEN: &str = "SLACK_API_BOT_TOKEN";
    let slack_token_string: String = get_env_var(SLACK_API_BOT_TOKEN);
    let token_value: SlackApiTokenValue = slack_token_string.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);

    token
}
