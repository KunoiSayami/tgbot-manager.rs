use anyhow::anyhow;
use teloxide::{dispatching::UpdateFilterExt, prelude::*};

type Result<T> = anyhow::Result<T>;

async fn on_message(bot: AutoSend<Bot>, message: Message) -> Result<()> {
    let sender_chat = message
        .sender_chat()
        .ok_or_else(|| anyhow!("No sender chat"))?;

    // anonymous admin message
    if message.chat.id == sender_chat.id {
        return Ok(());
    }

    let channel_id = bot.get_chat(message.chat.id).await?.linked_chat_id();

    // linked channel message
    if channel_id == Some(sender_chat.id.0) {
        return Ok(());
    }

    if let Err(_) = bot.delete_message(message.chat.id, message.id).await {
        let _ = bot
            .send_message(
                message.chat.id,
                "Detected a channel message that is not from the linked channel, but failed to delete it.\n\n\
                 Did you give me permission to delete messages?",
            )
            .await;
        let _ = bot
            .ban_chat_sender_chat(message.chat.id, sender_chat.id)
            .await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    let handler =
        Update::filter_message().endpoint(|msg: Message, bot: AutoSend<Bot>| async move {
            let _ = on_message(msg, bot).await;
            respond(())
        });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
