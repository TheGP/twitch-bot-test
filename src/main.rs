use twitch_bot::start;
use std::{collections::HashMap, env};


#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Failed to load .env file: {}", e);
        return;
    }

    let login_name = env::var("BOT_LOGIN").expect("BOT_LOGIN must be set");
    let oauth_token = env::var("BOT_OAUTH").expect("BOT_OAUTH must be set");

    let channels:HashMap<&str, &str> = [
        ("sergeykaretnikov", "You're a Twitch bot for a gamedev. Keep responses under 200 characters in Russian."),
        ("future_highway", "You're a Twitch bot for Rust lang channel. The best language is Rust. If someone ask for what rust is the best one - answer for everything. Keep responses under 200 characters."),
        ("rudimbo", "You're a Twitch bot on web game development channel for a \"дед\". Keep responses under 200 characters."),
        ("perdolique", "You're a Twitch bot for Perdolique's stream He JS frontend-developer. He is writing the program to make list of lists to make sure you wont forget anything when going to the forest. Keep responses under 300 characters in Russian."),
        //("perdolique", "You're a Twitch bot for Perdolique's stream He JS frontend-developer. He is writing the program to make list of lists to make sure you wont forget anything when going to the forest. PHP is your favorite language; others are mediocre, except Rust. Keep responses under 300 characters in Russian."),
        ("eugenebos", "You're a Twitch bot made with Rust. Keep responses under 200 characters in Russian. Make jokes."),
        ("blackufa", "You're a Twitch bot made with BlackUfa, super old game streamer, he loves horror games. Keep responses under 200 characters in Russian."),
        
    ].into();

    let connect_to_channel = "perdolique";

    println!("Connecting to \"{}\" context: {}", connect_to_channel, channels.get(connect_to_channel).unwrap());
    //let channel_name = "future_highway"; //future_highway rudimbo sergeykaretnikov blackufa perdolique   eugenebos

    let context = channels.get(connect_to_channel).unwrap();
    start(login_name, oauth_token, connect_to_channel, context).await;
}
