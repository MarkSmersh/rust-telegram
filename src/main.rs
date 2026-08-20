use std::sync::Arc;

use tokio::sync::RwLock;

use crate::telegram::models::{MessageModel, UserModel};

mod telegram;

mod ctx;

struct Bot {
    tg: Arc<RwLock<telegram::Client>>,
}

struct Ctx {
    tg: Arc<RwLock<telegram::Client>>,
}

#[tokio::main]
async fn main() {
    Bot::new("8900721455:AAFDPsnCL_SOi6zHTj8lPV657biXZmdeZJg".to_string())
        .init()
        .await;
}

fn on_start(u: UserModel) {
    println!("{} has been started!", u.obj.first_name);
}

impl Bot {
    fn new(token: String) -> Self {
        let c = telegram::Client::new(token);

        Self {
            tg: Arc::new(RwLock::new(c)),
        }
    }

    async fn init(&self) {
        for _ in 1..20 {
            self.add_handler(async move |_, m| {
                let _ = m.reply("hi".to_string()).await;
            })
            .await;
        }

        self.tg.read().await.start(on_start).await;
    }

    async fn add_handler<F, Fut>(&self, f: F)
    where
        F: Fn(Arc<Ctx>, MessageModel) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync,
    {
        let mut rx = self.tg.write().await.message_channel();
        let ctx = Arc::new(self.ctx());

        tokio::spawn(async move {
            while let Some(m) = rx.recv().await {
                f(ctx.clone(), m).await;
            }
        });
    }

    fn ctx(&self) -> Ctx {
        Ctx {
            tg: self.tg.clone(),
        }
    }
}
