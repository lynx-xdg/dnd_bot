use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    sync::Mutex
};

mod initiative;
mod roll;

pub struct Data {
    initiatives: Mutex<HashMap<u64, initiative::Tracker>>
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("TOKEN").expect("missing TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roll::roll(), roll::groll(), initiative::initiative()],
            prefix_options: poise::PrefixFrameworkOptions{ prefix: Some("!".to_string()), ..Default::default()},
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {initiatives: Mutex::new(HashMap::new())})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
