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

    // If the first word is the command `|price`
    if words[0] == "|price" {
        // Craft the URL to fetch the price
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=USD",
            words[1]
        );
        // Send a GET request to the url and parse as string
        let res_string = get(&url).ok()?.text().ok()?;
        // Convert the String to JsonValue
        let res_json = json::parse(&res_string).ok()?;
        // Get the price from the json
        let price = res_json[words[1].clone()]["usd"].clone();
        // Check if no price was returned, meaning crypto wasn't found in coingecko api
        if price.is_null() {
            // Return error message
            return Some(Message::new().add_text("No price data found for requested crypto."));
        }
        // Else price acquired and is to be returned
        else {
            // Return the price Message
            return Some(Message::new().add_text(&format!("USD ${}", price)));
        }
    }

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~mocrux-nomdep", "test-93");
    chat_bot.run();
}
