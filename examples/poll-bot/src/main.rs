use json;
use rand::random;
use std::fs;
use urbit_chatbot_framework::{AuthoredMessage, Chatbot, Message};

// poll file structure is json object:
// {opts:{"opt1":0,"opt2":0, ..}, owner:"shipname", active:true, voters:{"voter1":1,"voter2":1, ..}}

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Split the message up into words (split on whitespace)
    let words = authored_message.message.to_formatted_words();
    // If the first word is the command `!poll`, we initialize a new poll.
    // A name for the poll can be added with title flag '-t' or it will be an auto-generated number.
    if words[0] == "!poll" {
        let options = &words[1..];
        let mut msg = "Poll started with options: ".to_string();
        let mut poll_id = random::<u8>().to_string();
        let mut poll_json = json::JsonValue::new_object();
        poll_json["opts"] = json::JsonValue::new_object();
        // this is so only the person who starts the poll can end it
        poll_json["owner"] = authored_message.author.into();
        // set poll to active
        poll_json["active"] = true.into();
        // write specified poll options into json object 'opts'
        let mut i = 0;
        while i < options.len() {
            // check for '-t' flag and set title of poll if you find it
            if options[i] == "-t" {
                poll_id = options[i+1].clone();
                i += 1;
            } else {
                poll_json["opts"][&options[i]] = 0.into();
                msg.push_str(&(" ".to_owned() + &options[i]));
            }
            i += 1;
        }

        msg.push_str(&format!(
            ". Type \"!vote {} [option]\" to participate.",
            poll_id.to_string()
        ));
        // write to file named by poll id
        fs::create_dir("polls").ok();
        fs::write(format!("polls/{}.json", poll_id), json::stringify(poll_json))
            .expect("error writing poll file");

        return Some(Message::new().add_text(&msg));
    } else if words[0] == "!vote" {
        if words.len() != 3 {
            return Some(
                Message::new().add_text("Invalid vote. Format: \"!vote [poll id] [option]\""),
            );
        }

        // read poll file of chosen poll
        let poll_str = fs::read_to_string(format!("polls/{}.json", words[1]))
            .expect("error reading poll file");
        let mut poll_json = json::parse(&poll_str).unwrap();
        // make sure voter hasn't voted already
        let already_voted = &poll_json["voters"][&authored_message.author];
        if already_voted == 1 {
            return Some(Message::new().add_text("You've already voted in this poll."));
        }
        // make sure poll is still active
        if poll_json["active"] == false {
            return Some(Message::new().add_text("This poll is no longer active."));
        }
        // make sure chosen option is legitimate for this poll
        if poll_json["opts"][&words[2]].as_i32().is_none() {
            return Some(Message::new().add_text("Chosen option is not valid for this poll."));
        }

        // modify json object to contain new vote
        let curr_count = poll_json["opts"][&words[2]].clone();
        let new_count = curr_count.as_i32().unwrap() + 1;
        poll_json["opts"][&words[2]] = new_count.into();
        // add voter to voted list
        poll_json["voters"][&authored_message.author] = 1.into();

        // write new result to file
        fs::write(format!("polls/{}.json", words[1]), json::stringify(poll_json))
            .expect("error writing poll file");
        let text = format!("Vote for {} counted.", words[2]);
        return Some(Message::new().add_text(&text));
    } else if words[0] == "!results" {
        // Return result of poll if given a poll id. 
        // If user says 'all' then results of all active polls are returned.
        if words.len() != 2 {
            return Some(
                Message::new().add_text("Invalid poll command, make sure to specify a poll ID")
            );
        } else if words[1] == "all" {
            // return result of all active polls
            fs::create_dir("polls").ok();
            let files = fs::read_dir("polls/").unwrap();
            let mut text = "".to_string();
            for file in files {
                let poll_id = file.unwrap().path();
                let poll_str = fs::read_to_string(&poll_id)
                               .expect("error reading poll file");
                let poll_json = json::parse(&poll_str).unwrap();
                if poll_json["active"] == true {
                    // TODO: make vote output AND poll id output string look nicer
                    let votes = json::stringify(poll_json["opts"].clone());
                    text.push_str(&format!("Current results for poll '{}': {}. \n", 
                                   poll_id.display(), 
                                   votes));
                } 
            }
            // TODO: currently returns blank message if no active polls. add text in this case.
            return Some(Message::new().add_text(&text)); 
        }
        
        let poll_str = fs::read_to_string(format!("polls/{}.json", words[1]))
            .expect("error reading poll file");
        let poll_json = json::parse(&poll_str).unwrap();
        // TODO: make vote output string look nicer
        let votes = json::stringify(poll_json["opts"].clone());
        let text = format!("Current results for poll '{}': {}", 
                           words[1], 
                           votes);
        return Some(Message::new().add_text(&text));
    } else if words[0] == "!endpoll" {
        if words.len() != 2 {
            return Some(
                Message::new().add_text("Invalid poll command, make sure to specify a poll ID")
            );
        }

        let poll_str = fs::read_to_string(format!("polls/{}.json", words[1]))
            .expect("error reading poll file");
        // check if person making command is poll owner
        let mut poll_json = json::parse(&poll_str).unwrap();
        if poll_json["owner"].to_string() != authored_message.author {
            return Some(
                Message::new().add_text("Only the person who started the poll can end it."),
            );
        }
        // set poll status to not-active, json file remains but can no longer be edited by bot
        poll_json["active"] = false.into();
        let result = json::stringify(poll_json.clone());
        fs::write(format!("polls/{}.json", words[1]), &result)
            .expect("error writing poll file");
        let votes = json::stringify(poll_json["opts"].clone());
        let text = format!("Poll ID {} has ended. Results: {}", words[1], votes);
        return Some(Message::new().add_text(&text));
    }

    // Otherwise do not respond to message
    None
}

fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~bacrys", "testchat-30");
    chat_bot.run();
}