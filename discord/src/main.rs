use std::env;

use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::GatewayIntents;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[group]
#[commands(ping, triangulate)]
struct General;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = StandardFramework::new()
    .configure(|c| c.prefix("!"))
    .group(&GENERAL_GROUP);

  let mut client = Client::builder(&token, GatewayIntents::all())
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

  if let Err(e) = client.start().await {
    println!("Client error: {:?}", e);
  }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  msg.reply(ctx, "Pong!").await?;

  Ok(())
}

#[command]
async fn triangulate(ctx: &Context, msg: &Message) -> CommandResult {
  msg.reply(ctx, "Where are you?").await?;

  Ok(())
}