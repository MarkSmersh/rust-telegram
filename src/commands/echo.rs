use crate::{Ctx, commands::EchoArgs, telegram::models::MessageModel};

pub async fn echo(ctx: Ctx<EchoArgs>, m: MessageModel) {
    let mut string = ctx.string.clone();

    if ctx.uppercase {
        string = string.to_uppercase();
    }

    if ctx.italic {
        string = format!("<i>{}</i>", string);
    }

    if ctx.bold {
        string = format!("<b>{}</b>", string);
    }

    let _ = m.reply(string).await;
}
