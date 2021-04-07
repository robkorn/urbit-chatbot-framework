# Urbit Chatbot Framework

A framework that allows anyone to create an Urbit Chatbot with only a few lines of code.

The Urbit Chatbot Framework takes care of all the complexities of connecting to a ship, subscribing to a chat, parsing messages, and sending messages automatically for you. All you have to do is simply write the message handling/response logic and everything else will just work.

This crate is [available on crates.io](https://crates.io/crates/urbit-chatbot-framework).

[![awesome urbit badge](https://img.shields.io/badge/~-awesome%20urbit-lightgrey)](https://github.com/urbit/awesome-urbit)

## Basic Design

At its core, the framework revolves around the `Chatbot` struct. It is defined as:

```rust
/// This struct represents a chatbot that is connected to a given `ship`,
/// is watching/posting to a specific `chat_ship`/`chat_name`
/// and is using the function `respond_to_message` to process any messages
/// which are posted in said chat.
pub struct Chatbot {
    /// `respond_to_message` is a function defined by the user of this framework.
    /// This function receives any messages that get posted to the connected chat,
    /// and if the function returns `Some(message)`, then `message` is posted to the
    /// chat as a response. If it returns `None`, then no message is posted.
    respond_to_message: fn(AuthoredMessage) -> Option<Message>,
    ship: ShipInterface,
    chat_ship: String,
    chat_name: String,
}
```

To create a chatbot, one simply must instantiate a `Chatbot` struct with:

- A `ShipInterface` to connect to to interact with the Urbit network
- The `chat_ship` that the chat is running on
- The `chat_name`
- A function (optionally named respond_to_message) which takes an `AuthoredMessage` as input, and returns an `Option<Message>` as output

## Creating A Chatbot

A `Chatbot` is most easily created using the `new_with_local_config` method:

```rust
/// Create a new `Chatbot` with a `ShipInterface` derived automatically
/// from a local config file. If the config file does not exist, the
/// `Chatbot` will create the config file, exit, and prompt the user to
/// fill it out.
pub fn new_with_local_config(
    respond_to_message: fn(AuthoredMessage) -> Option<Message>,
    chat_ship: &str,
    chat_name: &str,
) -> Self {
```

This automatically deals with creating a local ship config file for the user to edit if none available, and then on rerun, reading said config to connect to a ship.

In order to use this method, we need to define a `respond_to_message` function which we can supply as an argument. Here is an example of one of the simplest possible implementations:

```rust
fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Any time a message is posted in the chat, respond in chat with a static message.
    Some(Message::new().add_text("Calm Computing ~"))
}
```

Do note, that if this function returns `None`, then the `Chatbot` will not reply to the message, and simply continue forward processing the next one.

With this function defined, you can now easily call `Chatbot::new_with_local_config()` and acquire an instantiated `Chatbot`.

## Running The Chatbot

Once the `Chatbot` struct is defined, all one needs to do is:

```
chat_bot.run();
```

This will automatically perform all messaging, parsing, and interfacing with the connected Urbit ship without any further code required.

And just like that you have an Urbit chatbot ready to go.

# Example Projects

The projects linked below are example Urbit Chatbot projects. They link directly to the `main.rs` so you can immediately see the implementation of said chatbot.

You can also easily run any of these examples by:

1. Cloning this repo.
2. Changing directory into the example project folder in your terminal.
3. Editing `main.rs` with the `chat_ship` and `chat_name` which you wish to run the bot in.
4. Running `cargo run` the first time to create the ship config file.
5. Editing the `ship_config.yaml` with your ship information (Moons or comets typically work best. Do not use your daily-driver ship because messages from the ship the chatbot is connected to are all ignored.)
6. Running `cargo run` to run the chatbot with the ship config info provided.

### Simple Chatbot

[The Simple Chatbot](examples/simple-chatbot/src/main.rs) is the simplest chatbot possible which replies to all messages in a chat with a static message. Approximately 4 lines of real code, showing off how simple it is to create a chatbot.

The point of this project is to display the bare minimum requirements for setting up a chatbot.

### Anti Comet Chatbot

[The Anti Comet Chatbot](examples/anti-comet-chatbot/src/main.rs) is a slightly more advanced chatbot which takes a look at the ship that authored the latest message. If the ship has a name long enough to be classified a comet, then it responds with a message. Otherwise, it returns `None`, meaning no message is sent by the chatbot to the chat in reply (if they aren't a comet).

### Crypto Prices Chatbot

[The Crypto Prices Chatbot](examples/crypto-prices-chatbot/src/main.rs) is a real world example of a useful chatbot that implements a command for everyone in a chat to use.

In effect, if anyone types:

```
|price {crypto_name_here}
```

such as

```
|price bitcoin
```

Then the bot will fetch the bitcoin price via coingecko API, and return it:

```
USD $37167
```

This is the first chatbot implemented via the Urbit Chatbot Framework which has real utility, and is a great example of how to go about building chatbots for use cases which need to reply based off of commands and make calls to external APIs.

### Bible Chatbot

[The Bible Chatbot](examples/bible-chatbot/src/main.rs) is another real world example of a useful chatbot that implements a command for everyone in a chat to use.

This chatbot allows anyone to use the `!bible` command and request one or more verses from the bible (KJV):

```
!bible John 1:1-5
```

### Poll Chatbot

[The Poll Chatbot](examples/poll-bot) is the most complex chatbot example enabling anyone to run votes/polls inside of their chats. Every poll is saved into a `.json` file and ensured that every ship only gets a single vote.

To start a poll with any given amount of options:

```
!poll [option1] [option2] ...
```
or
```
!poll -t [title] [option1] [option2] ...
```
The title of a poll acts as its ID, so don't use the same one multiple times yet.

Once a poll is running any ship can vote (precisely once):

```
!vote [pollid] [option]
```

Results of a poll can be accessed via:

```
!results [pollid]
```
or
```
!results all
```
to view all active polls.

The creator of a poll can end the poll via:

```
!endpoll [pollid]
```

Polls without titles have a numerical ID generated at creation and are stored in a json file in the directory that the bot is run from.

(Credits to [~hodzod-walrus](https://github.com/benmcc100) for creating the Poll Chatbot)
