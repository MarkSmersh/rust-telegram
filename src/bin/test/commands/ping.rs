use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::{Ctx, commands::PingArgs, telegram::models::MessageModel};

pub async fn ping(_: Ctx<PingArgs>, mut m: MessageModel) {
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let unix = i64::try_from(unix).unwrap();

    let i = Instant::now();

    let mut text = "Ping".to_owned();

    m = m.set_text(text.to_owned());

    let _ = m.send_draft(unix).await;

    let i = i.elapsed().as_millis();

    text = format!("{} {}ms", text, i);

    let _ = m.reply(text).await;
}
