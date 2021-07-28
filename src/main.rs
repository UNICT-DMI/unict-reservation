use std::error::Error;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text")]
    Help,
}

async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, "unict-reservation", handler).await;

    Ok(())
}
