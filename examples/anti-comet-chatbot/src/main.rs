use urbit_chatbot_framework::{AuthoredMessage, ChatBot, Message};

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // If the message author has a name with more than 28 characters (therefore is a comet)
    if authored_message.author.len() > 28 {
        // Return a message that calls out the comet
        return Some(
            Message::new()
                .add_mention(&format!("~{}", authored_message.author))
                .add_text(" You have been noticed by the Anti Comet Defense League."),
        );
    }
    if authored_message.author == "mocrux-nomdep" {}

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = ChatBot::new_with_local_config(respond_to_message, "~mocrux-nomdep", "test-93");
    chat_bot.run();
}
