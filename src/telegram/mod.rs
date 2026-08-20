// longpoll, receive updates, do echo with ms

pub mod models;
pub mod types;

use std::sync::Arc;

use reqwest::Error;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc;

use crate::telegram::{
    models::{MessageModel, Model, UpdateModel, UpdatesModel, UserModel},
    types::SendMessage,
};

type TgError<T> = Result<T, Error>;

#[derive(Clone)]
pub struct Client {
    token: String,
    http: reqwest::Client,

    message_senders: Vec<mpsc::Sender<MessageModel>>,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
            http: reqwest::Client::new(),
            message_senders: Vec::new(),
        }
    }

    pub async fn start(&self, fun: fn(u: UserModel)) {
        let res = self.get_me().await.expect("Unable to start the bot");

        fun(res);

        self.longpoll().await;
    }

    pub async fn request<P, R>(&self, method: &str, params: P) -> TgError<Model<R>>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, method);

        let res = self.http.post(url).json(&params).send().await?;

        let data = res.json::<types::Response<R>>().await?;

        if !data.ok {
            println!(
                "Telegram API error. Code: {}. {}",
                data.error_code.unwrap(),
                data.description.unwrap()
            )
        }

        let m = Model::new(Arc::new((*self).clone()), data.result.unwrap());

        Result::Ok(m)
    }

    pub async fn get_me(&self) -> Result<UserModel, Error> {
        self.request("getMe", types::GetMe {}).await
    }

    pub async fn send_message(&self, params: SendMessage) -> TgError<MessageModel> {
        self.request("sendMessage", params).await
    }

    pub async fn get_updates(&self, params: types::GetUpdates) -> TgError<UpdatesModel> {
        self.request("getUpdates", params).await
    }

    fn channel<T>(&self) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
        mpsc::channel::<T>(100)
    }

    pub fn message_channel(&mut self) -> mpsc::Receiver<MessageModel> {
        let (tx, rx) = self.channel();
        self.message_senders.push(tx);
        rx
    }

    async fn longpoll(&self) {
        let mut offst = 0;

        loop {
            let updates: Vec<UpdateModel> = self
                .get_updates(types::GetUpdates {
                    offset: Some(offst),
                    timeout: Some(69),
                    ..types::GetUpdates::default()
                })
                .await
                .expect("Unable to receive an update!")
                .into();

            for sender in self.message_senders.clone() {
                for u in updates.iter() {
                    if u.message.is_some() {
                        // ahh billion copies of the client...
                        let c = Arc::new((*self).clone());
                        let _ = sender.send(Model::new(c, u.message.clone().unwrap())).await;
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
