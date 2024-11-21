use poise::serenity_prelude::UserId;
use crate::{Error, Context};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Player {
    DiscordUser(UserId),
    CustomName(String)
}

#[derive(Default, Debug)]
pub struct Tracker {
    pub players: Vec<(Player, u8)>,
    pub current: usize,
}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker {
            players: Vec::new(),
            current: 0,
        }
    }
    pub fn set(&mut self, player: Player, initiative: u8) {
        self.leave(player.clone());
        let mut index = 0;
        for (_, pinitiative) in &self.players {
            if *pinitiative < initiative {
                break;
            }
            index += 1;
        }
        self.players.insert(index, (player, initiative));
    }
    pub fn leave(&mut self, player: Player) {
        for i in 0..self.players.len() {
            if self.players[i].0 == player {
                self.players.remove(i);
                if i < self.current {
                    if self.current == 0 {
                        self.current = self.players.len();
                    }
                    self.current -= 1;
                }
                break;
            }
        };
    }
    pub fn next(&mut self) -> Player {
        self.current = (self.current + 1) % self.players.len();
        let (player, _) = &self.players[self.current];
        player.clone()
    }
}

#[poise::command(slash_command, aliases("i"), prefix_command, subcommands("reset", "set", "kick", "leave", "view", "current", "next"))]
pub async fn initiative(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("I").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn reset(
    ctx: Context<'_>,
) -> Result<(), Error> {
    {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        initiatives.insert(ctx.channel_id().into(), Tracker::new());
        println!("{:?}", initiatives);
    }
    ctx.say("Reset initiatives").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    user: Option<UserId>,
    value: u8,
) -> Result<(), Error> {
    let user = if let Some(id) = user {
        id
    } else {
        ctx.author().id
    };
    {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get_mut(&ctx.channel_id().into()).unwrap();
        tracker.set(Player::DiscordUser(user), value);
        println!("{:?}", tracker);
    }
    let guild = ctx.guild_id().unwrap();
    let user = guild.member(ctx.http(), user).await.unwrap();
    ctx.say(format!("Added {} to the initiatives", user.display_name())).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get_mut(&ctx.channel_id().into()).unwrap();
        tracker.leave(Player::DiscordUser(ctx.author().id));
    }
    let guild = ctx.guild_id().unwrap();
    let user = guild.member(ctx.http(), ctx.author().id).await.unwrap();
    ctx.say(format!("{} left the initiatives", user.display_name())).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn kick(
    ctx: Context<'_>,
    user: UserId,
) -> Result<(), Error> {
    {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get_mut(&ctx.channel_id().into()).unwrap();
        tracker.leave(Player::DiscordUser(user));
    }
    let guild = ctx.guild_id().unwrap();
    let member = guild.member(ctx.http(), user).await.unwrap();
    ctx.say(format!("Kicked {} from the initiatives", member.display_name())).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn view(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let (players, current) = {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get_mut(&ctx.channel_id().into()).unwrap();
        (tracker.players.clone(), tracker.current)
    };
    let guild = ctx.guild_id().unwrap();
    let mut pad_size = 0;
    for (player, _) in &players {
        let len = match player {
            Player::DiscordUser(id) => {
                let member = guild.member(ctx.http(), id).await.unwrap();
                member.display_name().len()
            },
            Player::CustomName(name) => {
                name.len()
            }
        };
        if pad_size < len {
            pad_size = len;
        }
    }
    let mut out = "```".to_string();
    for i in 0..players.len() {
        let prefix = if i == current {
            "x"
        } else {
            " "
        };
        let (player, ini) = &players[i];
        let mut name = match player {
            Player::DiscordUser(id) => {
                let member = guild.member(ctx.http(), id).await.unwrap();
                member.display_name().to_owned()
            },
            Player::CustomName(name) => {
                name.to_string()
            }
        };
        while name.len() < pad_size {
            name.push(' ');
        }
        out.push_str(&format!("{} | {} | {}\n", prefix, name, ini));
    }
    out.push_str(&"```");
    ctx.say(out).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn current(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let player = {
        let initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get(&ctx.channel_id().into()).unwrap();
        tracker.players[tracker.current].0.clone()
    };
    let name = match player {
        Player::CustomName(name) => name,
        Player::DiscordUser(id) => {
            let guild = ctx.guild_id().unwrap();
            let member = guild.member(ctx.http(), id).await.unwrap();
            member.display_name().to_owned()
        }
    };
    ctx.say(format!("Current player: {}", name)).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn next(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let next = {
        let mut initiatives = ctx.data().initiatives.lock().unwrap();
        let tracker = initiatives.get_mut(&ctx.channel_id().into()).unwrap();
        tracker.next()
    };
    let name = match next {
        Player::DiscordUser(id) => {
            let guild = ctx.guild_id().unwrap();
            let user = guild.member(ctx.http(), id).await.unwrap();
            user.display_name().to_owned()
        },
        Player::CustomName(name) => {
            name
        }
    };
    ctx.say(format!("It's {}'s turn", name)).await?;
    Ok(())
}
