use sdi::*;
use crate::{Context, Error};

/// Roll a die
#[poise::command(prefix_command, slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Roll"] #[rest] roll_str: String,
) -> Result<(), Error> {
    let mut parser = Parser::from_text(roll_str);
    let ast = parser.parse();
    let result = ast.eval();
    let output = format!("You rolled a {}", result);
    ctx.say(output).await?;
    Ok(())
}

/// Roll a die without telling the rest of chat
#[poise::command(prefix_command, slash_command)]
pub async fn groll(
    ctx: Context<'_>,
    #[description = "Roll"] #[rest] roll_str: String,
) -> Result<(), Error> {
    match ctx {
        poise::Context::Application(ctx) => {
            ctx.defer_ephemeral().await?;
        },
        poise::Context::Prefix(ctx) => {
            ctx.msg.delete(ctx.http()).await?;
            ctx.say("I can't do a GM roll using a prefix command TwT").await?;
            return Ok(())
        }
    };
    let mut parser = Parser::from_text(roll_str);
    let ast = parser.parse();
    let result = ast.eval();
    let output = format!("You rolled a {}", result);
    ctx.say(output).await?;
    Ok(())
}
