use json;
use rand::random;
use std::fs;
use urbit_chatbot_framework::{AuthoredMessage, Chatbot, Message};

// poll file structure is json object {opts:{"opt1":0, "opt2":0, ...}, owner:"shipname", voters:{"voter1":1, "voter2":1, ...}}

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Split the message up into words (split on whitespace)
    let words = authored_message.message.to_formatted_words();
    // let voter = authored_message.author; use later....

    // If the first word is the command `!poll`, we initialize a new poll.
    if words[0] == "!poll" {
        let opts = &words[1..];
        let mut msg = "Poll started with options: ".to_string();
        let pollid = random::<u8>();
        let mut pollfile = json::JsonValue::new_object();
        pollfile["opts"] = json::JsonValue::new_object();
        // this is so only the person who starts the poll can end it
        pollfile["owner"] = authored_message.author.into();
        // write specified poll options into json object 'opts'
        for i in 0..opts.len() {
            pollfile["opts"][&opts[i]] = 0.into();
            if i == 0 {
                msg.push_str(&opts[i]);
            } else {
                msg.push_str(&(" ".to_owned() + &opts[i]));
            }
        }

        msg.push_str(&format!(
            ". Type \"!vote {} [option]\" to participate.",
            pollid.to_string()
        ));
        // write to file named by poll id
        fs::create_dir("polls").ok();
        fs::write(format!("polls/{}.json", pollid), json::stringify(pollfile))
            .expect("error writing poll file");

        return Some(Message::new().add_text(&msg));
    } else if words[0] == "!vote" {
        if words.len() != 3 {
            return Some(
                Message::new().add_text("Invalid vote. Format: \"!vote [poll id] [option]\""),
            );
        }

        // read poll file of chosen poll
        let polldata = fs::read_to_string(format!("polls/{}.json", words[1]))
            .expect("error reading poll file");
        // parse vote data and find option to increment
        let mut parsed = json::parse(&polldata).unwrap();
        // make sure voter hasn't voted already
        let already_voted = &parsed["voters"][&authored_message.author];
        if already_voted == 1 {
            return Some(Message::new().add_text("You've already voted in this poll."));
        }
        // modify json object to contain new vote
        let curr_count = json::stringify(parsed["opts"][&words[2]].clone());
        let new_count = curr_count.parse::<u32>().unwrap() + 1;
        parsed["opts"][&words[2]] = new_count.into();
        // add voter to voted list
        parsed["voters"][&authored_message.author] = 1.into();
        // write new result to file
        let result = json::stringify(parsed.clone());
        fs::write(format!("polls/{}.json", words[1]), &result).expect("error writing poll file");
        let votes = json::stringify(parsed["opts"].clone());
        let text = format!("vote for {} counted. current results: {}", words[2], votes);
        return Some(Message::new().add_text(&text));
    } else if words[0] == "!endpoll" {
        if words.len() != 2 {
            return Some(Message::new().add_text("Invalid poll command"));
        }

        let result = fs::read_to_string(format!("polls/{}.json", words[1]))
            .expect("error reading poll file");
        // check if person making command is poll owner
        let parsed = json::parse(&result).unwrap();
        if parsed["owner"].to_string() != authored_message.author {
            return Some(
                Message::new().add_text("Only the person who started the poll can end it."),
            );
        }
        let votes = json::stringify(parsed["opts"].clone());
        let text = format!("Poll ID {} has ended. Results: {}", words[1], votes);
        return Some(Message::new().add_text(&text));
    }

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~ship-name", "chat-name");

    chat_bot.run();
}
