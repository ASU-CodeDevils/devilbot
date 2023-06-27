use std::vec;

use slack_morphism::blocks::{
    SlackBlock, SlackBlockMarkDownText, SlackBlockPlainText, SlackBlockText, SlackDividerBlock,
    SlackHeaderBlock, SlackSectionBlock,
};
use slack_morphism::SlackMessageContent;

use crate::slack;

// TODO: write docs
pub async fn run(
    username: &str,
    first_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let users_vec = vec![username];

    // Open the channel between the bot and the user
    let bot_conversations_open_response =
        slack::conversations::open(users_vec.clone(), true).await?;
    let bot_channel_id = bot_conversations_open_response.channel.id.to_string();
    log::info!("Bot Channel Id {:?}", bot_channel_id);

    // Build the message to send to the new user
    let content_vector: SlackMessageContent =
        SlackMessageContent::new().opt_blocks(build_bot_slack_block_message(first_name).into());
    slack::chat::post_message_with_content(&content_vector, &bot_channel_id, true)
        .await
        .unwrap_or_else(|err| log::info!("Chat post error: {}", err));

    // Open the channel between the user (Rhett) and the new user
    let user_conversations_open_response = slack::conversations::open(users_vec, false).await?;
    let user_channel_id = user_conversations_open_response.channel.id.to_string();
    log::info!("User Channel Id {:?}", user_channel_id);

    // Message to send to the new user
    let message_to_new_user_from_user = format!(
        "Hey {} welcome to CodeDevils! I am Rhett, the \
    President of the organization. Go ahead and introduce yourself over in <#CMGU8033K> and \
    I look forward to seeing you in <#C2N5P84BD> soon! :partywizard:",
        first_name
    );
    slack::chat::post_message(&message_to_new_user_from_user, &user_channel_id, false)
        .await
        .unwrap_or_else(|err| log::info!("Chat post error: {}", err));

    Ok(())
}
pub fn build_bot_slack_block_message(first_name: &str) -> Vec<SlackBlock> {
    let mut block_vector: Vec<SlackBlock> = vec![];

    // Create the header
    let header_block_text: SlackBlockPlainText =
        format!(":alert: Hey! Welcome to CodeDevils {} :alert:", first_name).into();
    let header_block_slack_block_text: SlackBlockText = header_block_text.into();
    let header_block: SlackHeaderBlock = SlackHeaderBlock::new(header_block_slack_block_text);
    block_vector.push(header_block.into());

    // Create the divider
    let slack_divider = SlackDividerBlock::new();
    let slack_divider_block: SlackBlock = slack_divider.into();
    block_vector.push(slack_divider_block.clone());

    let paragraph1: SlackBlockPlainText = "I am DevilBot, the official SlackBot for CodeDevils. I am here to welcome you, and to give you some information about who we are.".into();
    let paragraph1_block_text: SlackBlockText = paragraph1.into();
    let paragraph1_block: SlackBlock = SlackSectionBlock::new()
        .with_text(paragraph1_block_text)
        .into();
    block_vector.push(paragraph1_block);
    block_vector.push(slack_divider_block.clone());

    let paragraph2: SlackBlockPlainText = "First, if you are reading this in your email, get the Slack App on mobile and/or desktop. You don't want to miss anything!".into();
    let paragraph2_block_text: SlackBlockText = paragraph2.into();
    let paragraph2_block: SlackBlock = SlackSectionBlock::new()
        .with_text(paragraph2_block_text)
        .into();
    block_vector.push(paragraph2_block);
    block_vector.push(slack_divider_block.clone());

    let markdown1: SlackBlockMarkDownText = "We have meetings every other Wednesday at 6:30 PM MST over Zoom during the Fall and Spring semesters. Links for these meetings get posted to the <#C30L07P18> channel along with other announcements.".into();
    let markdown1_block_text: SlackBlockText = markdown1.into();
    let markdown1_block: SlackBlock = SlackSectionBlock::new()
        .with_text(markdown1_block_text)
        .into();
    block_vector.push(markdown1_block);

    let markdown2: SlackBlockMarkDownText = "Our President is Rhett Harrison <@WGJRL9NJD>. He should be messaging you right now as well. If you have any questions, he is a great person to start with. He may not know the answer, but he can connect you with the person who does.".into();
    let markdown2_block_text: SlackBlockText = markdown2.into();
    let markdown2_block: SlackBlock = SlackSectionBlock::new()
        .with_text(markdown2_block_text)
        .into();
    block_vector.push(markdown2_block);
    block_vector.push(slack_divider_block);

    let markdown3: SlackBlockMarkDownText =
        "Lastly, we have a discord server for you to join. Join here! https://discord.gg/xG4VnS2hfZ".into();
    let markdown3_block_text: SlackBlockText = markdown3.into();
    let markdown3_block: SlackBlock = SlackSectionBlock::new()
        .with_text(markdown3_block_text)
        .into();
    block_vector.push(markdown3_block);

    block_vector
}
