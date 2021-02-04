#![feature(map_first_last, binary_heap_into_iter_sorted)]

mod cron;
mod matrix;
mod picker;

use argh::FromArgs;
use matrix::Bot;
use tokio::select;

use crate::cron::BruTimeJob;

#[derive(FromArgs)]
#[argh(description = "bru bot")]
struct BruBotArgs {
    #[argh(
        option,
        description = "homeserver url",
        default = "String::from(\"http://localhost:8448\")"
    )]
    homeserver_url: String,

    #[argh(
        option,
        description = "username",
        default = "String::from(\"random_bru_bot\")"
    )]
    username: String,

    #[argh(
        option,
        description = "password",
        default = "String::from(\"wordpass\")"
    )]
    password: String,

    #[argh(
        option,
        description = "room name",
        default = "String::from(\"Random Bru\")"
    )]
    room_name: String,

    #[argh(
        option,
        description = "cron",
        default = "String::from(\"0 0 9 * * Mon *\")"
    )]
    cron: String,

    #[argh(
        option,
        description = "ignored members",
        default = "String::from(\"-bot$\")"
    )]
    ignored_members: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: BruBotArgs = argh::from_env();

    let bot = Bot::new(
        args.homeserver_url,
        args.username,
        args.password,
        args.room_name,
        args.ignored_members,
    )
    .await?;

    let cron_job = BruTimeJob::new(bot.clone(), args.cron);

    if let Err(e) = select! {
        result = cron_job.start() => result,
        result = bot.start() => result,
    } {
        eprintln!("error: {}", e);
    }

    Ok(())
}
