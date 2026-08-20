use std::{ops::Deref, sync::Arc};

use crate::telegram::{Client, types};

pub struct Model<T> {
    tg: Arc<Client>,
    pub obj: T,
}

impl<T> Model<T> {
    pub fn new(tg: Arc<Client>, obj: T) -> Self {
        Self { tg: tg, obj: obj }
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
    pub async fn reply(&self, text: String) -> Result<MessageModel, reqwest::Error> {
        self.tg
            .send_message(types::SendMessage {
                text,
                chat_id: self.obj.clone().from.unwrap().id,
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
