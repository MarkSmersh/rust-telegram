use std::{collections::HashMap, ops::Deref, str::FromStr, sync::Arc};

use crate::telegram::{
    Client, TgResult,
    types::{self, ParseMode},
};

pub struct Model<T> {
    tg: Arc<Client>,
    obj: T,
    pub fields: HashMap<&'static str, BoxToString>,
}

type BoxToString = Box<dyn ToString + Send + Sync>;

impl<T> Model<T> {
    pub fn new(tg: Arc<Client>, obj: T) -> Self {
        Self {
            tg: tg,
            obj: obj,
            fields: HashMap::new(),
        }
    }

    pub fn set_parse_mode(self, parse_mode: ParseMode) -> Self {
        self.set("parse_mode", parse_mode)
    }

    pub fn set_message_thread_id(self, thread_id: i64) -> Self {
        self.set("thread_id", thread_id)
    }

    pub fn get_message_thread_id(&self) -> Option<i64> {
        match self.fields.get("thread_id") {
            Some(id) => match i64::from_str(id.to_string().as_str()) {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn set<V>(mut self, key: &'static str, value: V) -> Self
    where
        V: ToString + Send + Sync + 'static,
    {
        self.fields.insert(key, Box::new(value));
        self
    }

    pub fn set_text(self, text: String) -> Self {
        self.set("text", text)
    }

    pub fn get(&self, key: &'static str) -> Option<String> {
        match self.fields.get(key) {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    pub fn get_text(&self) -> Option<String> {
        self.get("text")
    }

    pub fn get_parse_mode(&self) -> ParseMode {
        match self.fields.get("parse_mode") {
            Some(pm) => match ParseMode::from_str(pm.to_string().as_str()) {
                Ok(pm) => pm,
                Err(_) => self.tg.parse_mode.clone(),
            },
            None => self.tg.parse_mode.clone(),
        }
    }
}

impl<T> Deref for Model<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

pub type MessageModel = Model<types::Message>;

impl MessageModel {
    pub async fn reply(&self, text: String) -> TgResult<MessageModel> {
        self.tg
            .send_message(types::SendMessage {
                text,
                parse_mode: Some(self.get_parse_mode()),
                chat_id: self.obj.clone().from.unwrap().id,
                message_thread_id: self.get_message_thread_id(),
                ..Default::default()
            })
            .await
    }

    pub async fn send_draft(&self, draft_id: i64) -> TgResult<BoolModel> {
        self.tg
            .send_message_draft(types::SendMessageDraft {
                text: self.get_text(),
                draft_id: draft_id,
                chat_id: self.from.as_ref().unwrap().id,
                parse_mode: Some(self.get_parse_mode()),
                message_thread_id: self.get_message_thread_id(),
                ..Default::default()
            })
            .await
    }
}

pub type UserModel = Model<types::User>;

impl UserModel {}

pub type UpdatesModel = Model<Vec<types::Update>>;

impl Into<Vec<Model<types::Update>>> for UpdatesModel {
    fn into(self) -> Vec<Model<types::Update>> {
        self.obj
            .into_iter()
            .map(|u| Model::new(self.tg.clone(), u))
            .collect()
    }
}

pub type UpdateModel = Model<types::Update>;

// this trait is used to implement a method for types.rs's structs,
// existing as an alternative option instead of [#derive()]
pub trait ToModel {
    fn to_model(self, tg: Arc<Client>) -> Model<Self>
    where
        Self: Sized,
    {
        Model::new(tg, self)
    }
}

pub type BoolModel = Model<bool>;
