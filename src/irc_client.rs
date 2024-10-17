use crate::database;
use crate::message_parser;
use crate::utils;
use futures::prelude::*;
use irc::client::prelude::*;
use sqlx::PgPool;
use log::{info, warn, error, debug};

pub async fn run_irc_client(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Configuring IRC client...");
    let config = Config {
        nickname: Some("predb-irc-rust".to_owned()),
        server: Some("irc.zenet.org".to_owned()),
        channels: vec!["#pre".to_owned()],
        ..Config::default()
    };

    info!("Creating and connecting IRC client...");
    let mut client = Client::from_config(config).await?;

    info!("Identifying with the server...");
    client.identify()?;

    info!("Connected to IRC server. Joining channel #pre...");

    let mut stream = client.stream()?;

    info!("Listening for messages...");
    while let Some(message) = stream.next().await.transpose()? {
        match message.command {
            Command::PRIVMSG(channel, msg) => {
                let stripped_msg = utils::strip_irc_formatting(&msg);
                debug!("Stripped message: {}", stripped_msg);
                if let Some((category, release_name)) = message_parser::parse_pre_message(&stripped_msg) {
                    info!("[PRE] {} - {}", category, release_name);
                    let group_name = message_parser::extract_group_name(&release_name);
                    debug!("Extracted group name: {}", group_name);

                    match database::insert_release(&pool, &release_name, &group_name, &category).await {
                        Ok(_) => info!("Release inserted into database successfully."),
                        Err(e) => error!("Failed to insert release into database: {}", e),
                    }
                } else {
                    debug!("Message did not match PRE format.");
                    if let Some(prefix) = message.prefix {
                        info!("[{}] {}: {}", channel, prefix, stripped_msg);
                    } else {
                        info!("[{}] Unknown: {}", channel, stripped_msg);
                    }
                }
            }
            Command::PING(server1, _server2) => {
                debug!("Received PING from server. Responding with PONG...");
                client.send_pong(&server1)?;
            }
            _ => {}
        }
    }

    warn!("IRC client disconnected.");
    Ok(())
}
