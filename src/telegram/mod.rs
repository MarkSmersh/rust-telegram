// longpoll, receive updates, do echo with ms

pub mod models;
pub mod types;

use std::{error::Error, ops::Deref, sync::Arc};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc;

use crate::telegram::{
    models::{BoolModel, MessageModel, Model, UpdateModel, UpdatesModel, UserModel},
    types::{ParseMode, SendMessage, Update},
};

type TgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone)]
pub struct Client {
    token: String,
    http: reqwest::Client,

    message_senders: Vec<mpsc::Sender<MessageModel>>,

    parse_mode: ParseMode,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
            http: reqwest::Client::new(),
            message_senders: Vec::new(),
            parse_mode: ParseMode::HTML,
        }
    }

    pub fn set_parse_mode(&mut self, mode: ParseMode) {
        self.parse_mode = mode;
    }

    pub async fn start(&self, fun: fn(u: UserModel)) {
        let res = self.get_me().await.expect("Unable to start the bot");

        fun(res);

        self.longpoll().await;
    }

    pub async fn request<P, R>(&self, method: &str, params: P) -> TgResult<Model<R>>
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
                match data.error_code {
                    Some(v) => v,
                    None => 0,
                },
                match data.description {
                    Some(d) => d,
                    None => "No description".to_owned(),
                }
            );
        }

        let m = Model::new(
            Arc::new((*self).clone()),
            match data.result {
                Some(r) => r,
                None => return Result::Err("No result?")?,
            },
        );

        Ok(m)
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
            let updates: Vec<UpdateModel> = match self
                .get_updates(types::GetUpdates {
                    offset: Some(offst),
                    timeout: Some(69),
                    ..types::GetUpdates::default()
                })
                .await
            {
                Ok(u) => u.into(),
                Err(e) => {
                    println!("{}", e.to_string());
                    continue;
                }
            };

            for sender in self.message_senders.clone() {
                for u in updates.iter() {
                    match u.deref() {
                        Update {
                            message: Some(message),
                            ..
                        } => {
                            let c = Arc::new((*self).clone());
                            let _ = sender.send(Model::new(c, message.to_owned())).await;
                        }
                        _ => todo!("Other telegram updates (callback querym etc)"),
                    }
                }
            }

            match updates.last() {
                None => continue,
                Some(u) => offst = u.update_id + 1,
            }
        }
    }

    pub async fn get_me(&self) -> TgResult<UserModel> {
        self.request("getMe", types::GetMe {}).await
    }

    pub async fn send_message(&self, params: SendMessage) -> TgResult<MessageModel> {
        self.request("sendMessage", params).await
    }

    pub async fn get_updates(&self, params: types::GetUpdates) -> TgResult<UpdatesModel> {
        self.request("getUpdates", params).await
    }

    pub async fn send_message_draft(&self, params: types::SendMessageDraft) -> TgResult<BoolModel> {
        self.request("sendMessageDraft", params).await
    }
}
