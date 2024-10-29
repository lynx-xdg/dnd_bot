use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod die;

struct Initiative {
    values: Vec<(String, u8)>,
    current: usize
}

impl Initiative {
    fn new() -> Initiative {
        Initiative {values: Vec::new(), current: 0}
    }
}

struct Data {
    initiatives: Arc<Mutex<HashMap<u64, Initiative>>>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Create new initiative 'session'
#[poise::command(slash_command, prefix_command)]
async fn inew(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut initiatives = ctx.data().initiatives.lock().unwrap();
    let key = u64::from(ctx.channel_id());
    if initiatives.contains_key(&key) {
        ctx.say("Warning: deleting old initiatives").await?;
    }
    initiatives.insert(key, Initiative::new());
    Ok(())
}

/// Roll a die
#[poise::command(slash_command, prefix_command)]
async fn roll(
    ctx: Context<'_>,
    #[description = "Roll"] #[rest] roll_str: String,
) -> Result<(), Error> {
    let response = if let Some(roll) = die::Roll::parse(&roll_str) {
        let mut roll_txt = String::new();
        let mut sum = 0;
        let mut stc = 0;
        for (roll, max) in roll.eval_vec() {
            sum += roll;
            if max == 1 {
                stc += 1;
                continue;
            }
            roll_txt.push_str(&format!("`{}/{}` + ", roll, max).to_string());
        }
        let roll_txt = if stc == 0 {
            roll_txt.trim_end_matches("+ ")
        } else {
            &format!("{}`{}/1`", roll_txt, stc).to_string()
        };
        format!("You rolled a {} = `{}`", roll_txt, sum)
    } else {
        format!("Welp, I didn't understand your roll TwT")
    };
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    //let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let token = "MTI5NTg0NzE1NzcwNTg3MTM2MA.G0b0wa.6PM-wyzXFej_UH7HAfjpmir4FxCDmBpBgKGaOI";
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), roll()],
            prefix_options: poise::PrefixFrameworkOptions{ prefix: Some("/".to_string()), ..Default::default()},
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
