use rand::prelude::*;
use urbit_chatbot_framework::{AuthoredMessage, ChatBot, Message};

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // If the message is written by a specific @p
    if authored_message.author == "mocrux-nomdep" {
        // Return a message that mentions ~mocrux-nomdep
        return Some(
            Message::new()
                .add_text("How's it going ")
                .add_mention(&format!("~{}", authored_message.author))
                .add_text(&format!(
                    "? Here's a random character: {}",
                    rand::random::<char>()
                )),
        );
    }

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = ChatBot::new_with_local_config(respond_to_message, "~mocrux-nomdep", "test-93");
    chat_bot.run();
}
