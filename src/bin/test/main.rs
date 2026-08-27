mod commands;
mod ctx;

use clap::Parser;
use rust_telegram::{
    telegram::{self, Client, models::UserModel, types::ParseMode::Markdown},
    tools::Env,
};
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    commands::{
        Cli,
        Commands::{Echo, Ping},
        echo::echo,
        ping::ping,
    },
    ctx::Ctx,
};

struct Bot {
    tg: Arc<RwLock<Client>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let env = Env::new()?;

    let token = env
        .get("TEST_API")
        .expect("TEST_API enviroment variable is not provided.")
        .to_owned();

    Bot::new(token).init().await;

    Ok(())
}

fn on_start(u: UserModel) {
    println!("{} has been started!", u.first_name);
}

impl Bot {
    fn new(token: String) -> Self {
        let c = telegram::Client::new(token);

        Self {
            tg: Arc::new(RwLock::new(c)),
        }
    }

    async fn init(&self) {
        self.add_handler().await;

        self.tg.read().await.start(on_start).await;
    }

    async fn add_handler(&self) {
        let mut rx = self.tg.write().await.message_channel();
        let tg = self.tg.clone();

        tokio::spawn(async move {
            while let Some(m) = rx.recv().await {
                match m.text.clone().unwrap().as_str() {
                    "/start" => {
                        let _ = m.reply("dobry poczatek".to_string()).await;
                    }
                    &_ => {
                        let c = Cli::try_parse_from(m.text.clone().unwrap().split_whitespace());

                        match c {
                            Ok(c) => match c.command {
                                Echo(args) => {
                                    echo(Ctx::new(tg.clone(), args), m).await;
                                }
                                Ping(args) => {
                                    ping(Ctx::new(tg.clone(), args), m).await;
                                }
                            },
                            Err(e) => {
                                let _ = m.set_parse_mode(Markdown).reply(e.to_string()).await;
                            }
                        }
                    }
                }
            }
        });
    }
}
