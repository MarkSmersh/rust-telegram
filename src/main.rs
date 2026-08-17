use std::{sync::Arc, time::Instant};

use tokio::sync::RwLock;

use crate::telegram::types;

mod telegram;

struct Bot {
    tg: Arc<RwLock<telegram::Client<Ctx>>>,
}

struct Ctx {
    tg: Arc<RwLock<telegram::Client<Ctx>>>,
}

#[tokio::main]
async fn main() {
    Bot::new("".to_string()).init().await;
}

fn on_start(u: telegram::types::User) {
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
        self.tg
            .write()
            .await
            .add_listener(self.ctx(), |ctx: Arc<Ctx>, m: types::Message| {
                Box::pin(async move {
                    dbg!(m.text.clone());

                    let start = Instant::now();

                    let _ = ctx
                        .tg
                        .read()
                        .await
                        .send_message(types::SendMessage {
                            chat_id: m.from.clone().unwrap().id,
                            text: m.text.clone().unwrap(),
                            ..Default::default()
                        })
                        .await;

                    let end = start.elapsed().as_millis();

                    let _ = ctx
                        .tg
                        .read()
                        .await
                        .send_message(types::SendMessage {
                            chat_id: m.from.unwrap().id,
                            text: end.to_string(),
                            ..Default::default()
                        })
                        .await;
                })
            });

        // self.handlers.
    }

    fn ctx(&self) -> Ctx {
        Ctx {
            tg: self.tg.clone(),
        }
    }
}
