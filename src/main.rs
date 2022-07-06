use std::process::Command;
use teloxide::prelude::*;
use tokio::task;
use serde::Deserialize;
use teloxide::types::InputFile;
use url::Url;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().set_api_url(Url::parse("http://s2.budziszm.pl:8081").unwrap()).auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let text = message.text().unwrap_or_else(|| "");
        if !text.starts_with("http") {
            return Ok(());
        }

        task::spawn(async move { on_url(bot, message).await });

        respond(())
    }).await;
}

async fn on_url(bot: AutoSend<Bot>, message: Message) -> Result<(), ()> {

    let video_url = spawn_ytdlp(message.text().unwrap());
    bot.send_message(message.chat.id, format!("Id: {video_url}"))
        .await
        .expect("Failed to send message");

    bot.send_video(message.chat.id, InputFile::file(video_url))
        .await
        .expect("Failed to send video");

    Ok(())
}

fn spawn_ytdlp(url: &str) -> String {
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

    let json = String::from_utf8_lossy(&output.stdout);

    #[derive(Deserialize)]
    struct Response {
        id: String
    }

    let response: Response = serde_json::from_str(&json).unwrap();
    format!("/var/www/html/v/{0}.mp4", response.id)
}

// enum Base {
//     Number(f32),
//     Boolean(bool)
// }
//
// struct Params {
//     arg: String,
//     base: Base
// }
//
// enum Filter {
//     Bassboost(Params)
// }