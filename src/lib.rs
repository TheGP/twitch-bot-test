use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role};

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient; // message,
use twitch_irc::{ClientConfig, SecureTCPTransport};
use rand::seq::SliceRandom;
use std::collections::HashSet;

use std::env;

fn random_char() -> char {
    let characters = ".*/,~+-#$%^@&";
    let mut rng = rand::thread_rng();

    // Create a HashSet from the characters to ensure uniqueness
    let char_set: HashSet<char> = characters.chars().collect();
    // Convert the HashSet to a Vec to use the choose method
    let char_vec: Vec<_> = char_set.iter().cloned().collect();

    // Get a random character from the Vec and return it
    let random_char: &char = char_vec.choose(&mut rng).unwrap();
    *random_char
}

async fn get_response(text: &str, context: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    let api_key = env::var("CHATGPT_KEY").expect("CHATGPT_KEY must be set");
    let base_url = "https://api.openai.com";
    let client = ChatGPTClient::new(&api_key, base_url);

    let chat_input = ChatInput {
        model: Model::Gpt_4o,
        messages: vec![
            Message {
                role: Role::System,
                content: context.to_owned(),
            },
            Message {
                role: Role::User,
                content: text.to_string(),
            },
        ],
        ..Default::default()
    };

    let response = client.chat(chat_input).await?;

    if let Some(choice) = response.choices.into_iter().nth(0) {
        //println!("Response: {:?}", message.message.content); // .message.content
        Ok(choice.message.content)
    } else {
        Ok("Ошибка".to_string())
    }
}


pub async fn start(login_name: String, oauth_token: String, channel_name_: &str, context: &str) {
    let channel_name = channel_name_.to_owned();
    let context = context.to_owned();

    // default configuration is to join chat as anonymous.
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name, Some(oauth_token))
    );
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join(channel_name.clone()).unwrap();

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            //println!("Received message: {:?}\n\n", message);

            if let twitch_irc::message::ServerMessage::Privmsg(msg) = message {

                if msg.message_text.contains("!почему") || msg.message_text.contains("!апочему")  {
                    println!("!message: {:?}", msg);

                    let message = twitch_irc::message::IRCMessage::new_simple(
                        format!("@reply-parent-msg-id={} PRIVMSG", msg.message_id).to_string(),
                        vec![format!("#{}", channel_name).to_string(), format!(". {} {}", "Иди нахрен", random_char())]
                    );
                    client.send_message(message).await.unwrap();
                }

                if msg.message_text.contains("!ai") {
                    //client.say(channel_name.to_owned(), "@eugenebos1 Я тут".to_owned()).await.unwrap();
                    println!("!ai message detected");
                    println!("!message: {:?}", msg);

                    let tags = msg.source.tags;

                    if tags.0.contains_key("reply-parent-msg-body") && tags.0.contains_key("reply-thread-parent-msg-id") {
                        let parent_body = tags.0.get("reply-parent-msg-body").unwrap();
                        let reply_parent_msg_body = parent_body.clone().unwrap();
                        let reply_parent_msg_id = tags.0.get("reply-thread-parent-msg-id").unwrap().clone().unwrap();
                        println!("!BODY: {:?} {:?}", reply_parent_msg_body, reply_parent_msg_id);
                        
                        if let Ok(text) = get_response(&reply_parent_msg_body, &context).await {
                            println!("Response: {} {:?}", text, reply_parent_msg_id);
                            let message = twitch_irc::message::IRCMessage::new_simple(
                                format!("@reply-parent-msg-id={} PRIVMSG", reply_parent_msg_id).to_string(),
                                vec![format!("#{}", channel_name).to_string(), format!(". {}", text.replace("\n", " "))]
                            );
                            client.send_message(message).await.unwrap();
                        }
                    } else {
                        let message = msg.message_text[4..].to_string();
                        println!("!no parent detected: {}", message);

                        if let Ok(text) = get_response(&message, &context).await {
                            let message = twitch_irc::message::IRCMessage::new_simple(
                                format!("@reply-parent-msg-id={} PRIVMSG", msg.message_id).to_string(),
                                vec![format!("#{}", channel_name).to_string(), format!(". {}", text.replace("\n", " "))]
                            );
                            client.send_message(message).await.unwrap();
                            println!("Response: {}", text);
                        }
                    }
                }
            }
        }
    });


    join_handle.await.unwrap();
}
