use serenity::async_trait; //Provides support for asynch traits
use serenity::model::gateway::Ready; //Event for when bot is ready
use serenity::model::channel::Message; //Message sent in channel
use serenity::model::gateway::GatewayIntents; //Specify which events bot should listen to
use serenity::prelude::*; //Commonly used traits and types from serenity
use std::env; //Interact with environment
use dotenv::dotenv; //Loads environment variables from `.env`

struct Handler; //Implement EventHandler to handle discord events

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) { //Handle incoming messages
        if msg.content == "/ping" { //If message if "!ping"
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await { //If sending msg has error
                println!("Error sending message: {:?}", why); //Print error
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) { //When discord triggers ready event, serenity calls ready function
        println!("{} is connected!", ready.user.name); //Notifies that bot is ready, prints bots name
    }
}

#[tokio::main]
async fn main() {
    println!("\n"); //Creates space
    dotenv().ok(); //Loads environmental variables from `.env` (if any)

    // Check if the token is properly retrieved
    let token = match env::var("DISCORD_TOKEN") { //Retrieves token from environmental variables
        Ok(token) => token, //If works then set token
        Err(_) => { //If error
            println!("Error: DISCORD_TOKEN environment variable not set."); //Print error
            return; //Close main method
        }
    };

    // Log the token (optional, for debugging purposes only)
    // println!("Token: {}", token); // Be careful with printing sensitive information

    // Enable necessary intents
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT; //Specifies events the bot will respond to
    let mut client = match Client::builder(&token, intents) //Creates a discord client based on the token and intents
        .event_handler(Handler)
        .await {
            Ok(client) => client, //If works then set client
            Err(e) => { //If error then give error
                println!("Error creating client: {:?}", e);
                return;
            }
        };

    if let Err(e) = client.start().await { //Starts client, if error
        println!("Client error: {:?}", e); //Then give error
    }
}