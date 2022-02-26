use slack_hook::{PayloadBuilder, Slack};

// This function parses the text event from the slack event subscription
// if the text contains any form of "ping", it will respond with "pong".
// This can be used as an example command when creating new commands for
// the Slack bot.
pub async fn run(channel: &str) {
    let slack = Slack::new(crate::DEVIL_BOT_TEST_CHANNEL_URL).unwrap();
    let p = PayloadBuilder::new()
        .text("pong")
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
