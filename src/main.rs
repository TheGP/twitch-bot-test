use twitch_bot::start;
use std::env;


#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Failed to load .env file: {}", e);
        return;
    }
    // if let Ok(text) = get_response("Какой язык лучше, Go или PHP?").await {
    //     println!("Response: {}", text);
    // }

    //let runtime = tokio::runtime::Runtime::new().unwrap();
    //runtime.block_on(twitch_reply()).unwrap();
   
    let login_name = env::var("BOT_LOGIN").expect("BOT_LOGIN must be set");
    let oauth_token = env::var("BOT_OAUTH").expect("BOT_OAUTH must be set");
    
    let channel_name = "eugenebos"; // rudimbo sergeykaretnikov blackufa perdolique   eugenebos

    start(login_name, oauth_token, channel_name).await;
}
