use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildId, Member};
use serenity::model::user::User;
use serenity::prelude::*;
use songbird::SerenityInit;

struct Handler {
    user_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        println!("Finding user {}", self.user_id);

        let guilds = context.cache.guilds();

        println!("Searching in {} guilds", guilds.len());

        let member = find_member_in_guilds(&context, self.user_id, guilds).await;

        member
            .expect("User not found")
            .user
            .direct_message(&context, |f| f.content("your mother"))
            .await
            .unwrap();
    }
}

async fn find_member_in_guilds(
    context: &Context,
    user_id: u64,
    guilds: Vec<GuildId>,
) -> Option<Member> {
    for guild in guilds {
        let members = guild.members(context, None, None).await.unwrap();

        for member in members {
            if member.user.id == user_id {
                return Some(member);
            }
        }
    }

    None
}

pub async fn dickcord() {
    let token =
        env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment youi fhfjck");

    let user_id: u64 = env::var("DISCORD_USER")
        .expect("Expected a DISCORD_USER id in the environment. Who am i supposed to follow?")
        .parse()
        .unwrap();

    let handler = Handler { user_id };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .register_songbird()
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
