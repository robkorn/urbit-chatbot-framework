use urbit_chatbot_framework::{AuthoredMessage, ChatBot, Message};

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Any time a message is posted in the chat, respond in chat with a static message.
    Some(Message::new().add_text("Calm Computing ~"))
}

fn main() {
    let chat_bot = ChatBot::new_with_local_config(respond_to_message, "~mocrux-nomdep", "test-93");
    chat_bot.run();
}
