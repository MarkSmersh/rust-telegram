use rust_telegram::telegram;
use std::{ops::Deref, sync::Arc};
use tokio::sync::RwLock;

type CtxTg = Arc<RwLock<telegram::Client>>;

pub struct Ctx<Args: Sized> {
    tg: CtxTg,
    args: Args,
}

impl<Args> Ctx<Args> {
    pub fn new(tg: CtxTg, args: Args) -> Self {
        Self { args: args, tg: tg }
    }
}

impl<Args> Deref for Ctx<Args> {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}
