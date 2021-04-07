use json;
use reqwest::blocking::get;
use urbit_chatbot_framework::{AuthoredMessage, Chatbot, Message};

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Split the message up into words (split on whitespace)
    let words = authored_message.contents.to_formatted_words();
    // Error check to ensure sufficient number of words to check for command
    if words.len() <= 2 {
        return None;
    }

    // If the first word is the command `!bible`
    if words[0] == "!bible" {
        // Craft the URL to fetch the verses
        let url = format!(
            "https://bible-api.com/{}%20{}?translation=kjv",
            words[1], words[2]
        );
        // Send a GET request to the url and parse as string
        let res_string = get(&url).ok()?.text().ok()?;
        // Convert the String to JsonValue
        let res_json = json::parse(&res_string).ok()?;
        // Get the verses reference from the json
        let reference = res_json["reference"].clone();
        if reference.is_null() {
            // Return error message
            return Some(Message::new().add_text("Unable to find verses."));
        }

        // Append all verses into a single string.
        let mut verse_text = "".to_string();
        for verse in res_json["verses"].members() {
            verse_text += &format!("{}", verse["text"]);
        }
        // Return the verses
        return Some(Message::new().add_text(&format!("== {} ==\n{}", reference, verse_text)));
    }

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~mocrux-nomdep", "test-93");
    chat_bot.run();
}
