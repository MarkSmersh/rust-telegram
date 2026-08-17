// longpoll, receive updates, do echo with ms

pub mod types;

use std::{pin::Pin, sync::Arc};

use reqwest::Error;
use serde::{Serialize, de::DeserializeOwned};

type Handler<Ctx, Args> = dyn Fn(Arc<Ctx>, Args) -> BoxFuture + Sync + Send;
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

struct CtxFn<Ctx, Args> {
    ctx: Arc<Ctx>,
    f: Arc<Handler<Ctx, Args>>,
}

impl<Ctx, Args> Clone for CtxFn<Ctx, Args> {
    fn clone(&self) -> Self {
        Self {
            ctx: Arc::clone(&self.ctx),
            f: Arc::clone(&self.f),
        }
    }
}

impl<Ctx, Args> CtxFn<Ctx, Args> {
    fn new<F>(ctx: Ctx, f: F) -> Self
    where
        F: Fn(Arc<Ctx>, Args) -> BoxFuture + 'static + Sync + Send,
    {
        Self {
            ctx: Arc::new(ctx),
            f: Arc::new(f),
        }
    }

    async fn exec(&mut self, args: Args) {
        let fut = (self.f)(self.ctx.clone(), args);
        tokio::spawn(fut);
    }
}

pub struct Client<Ctx> {
    token: String,
    http: reqwest::Client,
    messages_listeners: Vec<CtxFn<Ctx, types::Message>>,
}

impl<Ctx> Client<Ctx> {
    pub fn new(token: String) -> Self {
        Self {
            token,
            http: reqwest::Client::new(),
            messages_listeners: Vec::new(),
        }
    }

    pub async fn start(&self, fun: fn(u: types::User)) {
        let res = self.get_me().await.expect("Unable to start the bot");

        fun(res);

        self.longpoll().await;
    }

    pub async fn request<P, R>(&self, method: &str, params: P) -> Result<R, Error>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, method);

        println!("{}", url);

        let res = self.http.post(url).json(&params).send().await?;

        let data = res.json::<types::Response<R>>().await?;

        if !data.ok {
            println!(
                "Telegram API error. Code: {}. {}",
                data.error_code.unwrap(),
                data.description.unwrap()
            )
        }

        Result::Ok(data.result.unwrap())
    }

    pub async fn get_me(&self) -> Result<types::User, Error> {
        self.request("getMe", types::GetMe {}).await
    }

    pub async fn send_message(&self, params: types::SendMessage) -> Result<types::User, Error> {
        self.request("sendMessage", params).await
    }

    pub async fn get_updates(
        &self,
        params: types::GetUpdates,
    ) -> Result<Vec<types::Update>, Error> {
        let res = self.request("getUpdates", params).await;

        res
    }

    pub fn add_listener<F>(&mut self, ctx: Ctx, callback: F)
    where
        Ctx: Sized,
        F: Fn(Arc<Ctx>, types::Message) -> BoxFuture + Sync + Send + 'static,
    {
        self.messages_listeners.push(CtxFn::new::<F>(ctx, callback));
    }

    pub async fn longpoll(&self) {
        let mut offst = 0;

        loop {
            let updates = self
                .get_updates(types::GetUpdates {
                    offset: Some(offst),
                    timeout: Some(69),
                    ..types::GetUpdates::default()
                })
                .await
                .expect("Unable to receive an update!");

            for ctx_fn in self.messages_listeners.clone().iter_mut() {
                for u in updates.iter().clone() {
                    if u.message.is_some() {
                        // fun.clone()(u.message.clone().unwrap());
                        // let mut f: Box<dyn Fn(types::Message)> = *fun;
                        // (*fun)(u.message.clone().unwrap());
                        //
                        ctx_fn.exec(u.message.clone().unwrap()).await;
                    }
                }
            }

            match updates.last() {
                None => continue,
                Some(u) => offst = u.update_id + 1,
            }
        }
    }
}
