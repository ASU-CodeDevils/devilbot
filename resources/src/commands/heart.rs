// This function will add a heart reaction to the passed in message
// Send a Heart emoji to the channel with the specified timestamp from the message
use slack_hook::{PayloadBuilder, Slack};

// TODO Use reactions.add
pub async fn run(channel: &str) {
    let slack_webhook_url: String = crate::get_env_var(crate::DEVIL_BOT_TEST_CHANNEL_URL);
    let slack = Slack::new(&*slack_webhook_url).unwrap();
    let p = PayloadBuilder::new()
        .text("This message should have a heart? :heart:")
        .icon_emoji(":heart:")
        .channel(channel)
        .username("DevilBot")
        .build()
        .unwrap();

    let res: Result<(), slack_hook::Error> = slack.send(&p);
    match res {
        Ok(()) => log::info!("Slack message sent successfully."),
        Err(x) => log::info!("ERR: {}", x),
    }
}
