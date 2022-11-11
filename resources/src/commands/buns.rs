use crate::aws;
use crate::get_env_var;
use crate::slack;

const BUNS_TABLE_NAME: &str = "BUNS_TABLE_NAME";

/**  
 * Increments the buns counter for a specified user in the buns table.
 */
pub async fn increment_buns(enterprise_user_id: &str) {
    let buns_table_name: String = get_env_var(BUNS_TABLE_NAME);
    aws::dynamo::increment_item(&*buns_table_name, "user_id", enterprise_user_id, "buns")
        .await
        .unwrap_or_else(|err| log::info!("DynamoDB increment buns error: {}", err));
}

/**  
 * This function parses the text event from the slack event subscription
 * if the text contains any form of "buns", it will respond with a buns reaction.
 * This can be used as an example command when creating new commands for
 * the Slack bot.
 */
pub async fn run(
    channel: &str,
    enterprise_user_id: &str,
    timestamp: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    increment_buns(enterprise_user_id).await;

    slack::reactions::add(channel, timestamp, "buns")
        .await
        .unwrap_or_else(|err| log::info!("Reaction add error: {}", err));
    Ok(())
}
