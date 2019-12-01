use env_logger;
use log::{error, info};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context) {
                Ok(channel) => channel,
                Err(why) => {
                    error!("Error getting channel: {:?}", why);
                    return;
                }
            };
            info!("User {}",&msg.author.name);
            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
