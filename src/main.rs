use std::{ops::Deref, sync::Arc};

use clap::Parser;
use tokio::sync::RwLock;

use crate::{
    commands::{
        Cli,
        echo::echo,
        ping::{self, ping},
    },
    telegram::{
        models::UserModel,
        types::ParseMode::{HTML, Markdown},
    },
};

mod commands;
mod telegram;

mod ctx;

struct Bot {
    tg: Arc<RwLock<telegram::Client>>,
}

type CtxTg = Arc<RwLock<telegram::Client>>;

struct Ctx<Args: Sized> {
    tg: CtxTg,
    args: Args,
}

impl<Args> Ctx<Args> {
    fn new(tg: CtxTg, args: Args) -> Self {
        Self { args: args, tg: tg }
    }
}

impl<Args> Deref for Ctx<Args> {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

#[tokio::main]
async fn main() {
    Bot::new("".to_string()).init().await;
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

                        if c.is_err() {
                            let _ = m
                                .set_parse_mode(Markdown)
                                .reply(c.as_ref().err().unwrap().to_string())
                                .await;
                            continue;
                        }

                        match c.unwrap().command {
                            commands::Commands::Echo(args) => {
                                echo(Ctx::new(tg.clone(), args), m).await;
                            }
                            commands::Commands::Ping(args) => {
                                ping(Ctx::new(tg.clone(), args), m).await;
                            }
                        }
                    }
                }
            }
        });
    }
}
