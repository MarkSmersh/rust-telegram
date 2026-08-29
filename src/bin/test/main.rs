mod bot;
mod commands;
mod ctx;

use rust_telegram::{
    telegram::{self},
    tools::Env,
};
use std::error::Error;

use crate::{bot::Bot, ctx::Ctx};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let env = Env::new()?;

    let token = env
        .get("TEST_API")
        .expect("TEST_API enviroment variable is not provided.")
        .to_owned();

    Bot::new(token).init().await;

    Ok(())
}
