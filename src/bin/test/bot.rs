use std::sync::Arc;

use clap::Parser;
use rust_telegram::telegram::{Client, models::UserModel, types::ParseMode::Markdown};
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

pub struct Bot {
    tg: Arc<RwLock<Client>>,
}

impl Bot {
    pub fn new(token: String) -> Self {
        let c = Client::new(token);

        Self {
            tg: Arc::new(RwLock::new(c)),
        }
    }

    pub async fn init(&self) {
        self.add_handler().await;

        self.tg.read().await.start(Bot::on_start).await;
    }

    fn on_start(u: UserModel) {
        println!("{} has been started!", u.first_name);
    }

    async fn add_handler(&self) {
        let mut rx = self.tg.write().await.message_channel();
        let tg = self.tg.clone();

        tokio::spawn(async move {
            while let Some(m) = rx.recv().await {
                match &m.text {
                    Some(t) => match t.as_str() {
                        "/start" => {
                            let _ = m.reply("dobry poczatek".to_string()).await;
                        }
                        &_ => {
                            let c = Cli::try_parse_from(t.split_whitespace());

                            match c {
                                Ok(c) => match c.command {
                                    Echo(args) => echo(Ctx::new(tg.to_owned(), args), m).await,
                                    Ping(args) => ping(Ctx::new(tg.to_owned(), args), m).await,
                                },

                                Err(e) => {
                                    let _ = m.set_parse_mode(Markdown).reply(e.to_string()).await;
                                }
                            }
                        }
                    },
                    None => println!("Non-text value passed."),
                }
            }
        });
    }
}
