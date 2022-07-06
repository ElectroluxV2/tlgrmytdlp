use std::future::Future;
use std::process::Command;
use std::time::Duration;
use teloxide::prelude::*;
use tokio::task;
use tokio::time::sleep;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let text = message.text().unwrap();
        if !text.starts_with("http") {
            return Ok(());
        }

        task::spawn(async move { on_url(bot, message).await });

        respond(())
    }).await;
}

async fn on_url(bot: AutoSend<Bot>, message: Message) -> Result<(), ()> {

    spawn_ytdlp(message.text().unwrap());

    bot.send_message(message.chat.id, "aaaaaaaaaaaaa").await.expect("Failed to send message");

    Ok(())
}

fn spawn_ytdlp(url: &str) {
    let output = Command::new("yt-dlp")
        .arg("-q")
        .arg("-S")
        .arg("+res:480,codec,br")
        .arg("-J")
        .arg("--no-simulate")
        .arg("--recode-video")
        .arg("mp4")
        .arg("-o")
        .arg("/var/www/html/v/%(id)s.%(ext)s")
        .arg(url)
        .output()
        .expect("Failed to run yt-dlp");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}