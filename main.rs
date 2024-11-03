use std::fs::File;
use std::path::Path;
use std::{path, str, thread};
use std::time::Duration;
use serenity::futures::io::BufReader;
use serenity::model::guild;
use serenity::{async_trait, futures::stream::Count};
use serenity::model::channel::{self, Message};
use serenity::prelude::*;
use std::io::{self, Write};
use serenity::framework::standard::StandardFramework;
use rand::Rng;
use dotenv::dotenv;
use rpassword::read_password;   
use colored::Colorize;

const OWNERLINE: &str = "Made by RetroWeb (; Rust IS PEAK";
const LOGO: &str = "╔═╗╔╗╔╔═╗╔╗╔\n╠═╣║║║║ ║║║║\n╩ ╩╝╚╝╚═╝╝╚╝";
const DESCRIPTION: &str = "A selfbot made 100% in RUST!";

struct UniFunc;

impl UniFunc {
    fn print_logo() {
        println!("{}", LOGO.purple())
    }
}

#[async_trait]
impl EventHandler for UniFunc {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!help" {
            msg.delete(&ctx.http).await.ok();
            msg.channel_id.say(&ctx.http, "# ANON - Made by RetroWeb").await.ok();
            msg.channel_id.say(&ctx.http, "```Welcome to Anon selfbot made in RUST. Here are some tools to use in discord:\n- !help: displays this message in discord\n- !spam: Spam a message\n- !RNG: Generate a random number 1-100\n- !ping: Responds PONG with debug logs\n- !explain: Explains what this is!\n- !avatar: Gets a users avatar```").await.ok();
        }
        if msg.content == "!ping" {
            msg.delete(&ctx.http).await.ok();
            if let Err(err) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Failed to ping message debugged: {}", err)
            }
            else {
                println!("Succesfully sent message");
            }
            thread::sleep(Duration::from_secs(3));
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}\n{} {}", 
                LOGO.purple().bold(), 
                DESCRIPTION.purple(), 
                OWNERLINE.purple()
            );
        }
        if msg.content == "!spam" {
            msg.delete(&ctx.http).await.ok();
            let mut message = String::new();
            let mut count_input = String::new();
            print!("Enter the message to spam: ");
            io::stdout().flush().unwrap(); 
            io::stdin().read_line(&mut message).expect("Failed to read line");
            print!("Enter the number of times to spam: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut count_input).expect("Failed to read line");
            let count: usize = count_input.trim().parse().expect("Invalid number!");
            let mut counter = 0;
            let messagetospam = message.trim();
            msg.channel_id.say(&ctx.http, "# ANON - Made by RetroWeb").await.ok();
            while counter < count {
                counter += 1;
                msg.channel_id.say(&ctx.http, format!("{}", messagetospam)).await.ok();
            }
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}\n{} {}", 
                LOGO.purple().bold(), 
                DESCRIPTION.purple(), 
                OWNERLINE.purple()
            );
        }
        if msg.content == "!RNG" {
            msg.delete(&ctx.http).await.ok();
            let randnum = rand::thread_rng().gen_range(0..100);
            msg.channel_id.say(&ctx.http, "# ANON - Made by RetroWeb").await.ok();
            msg.channel_id.say(&ctx.http, format!("Random number generated: {}", randnum)).await.ok();
        }
        if msg.content == "!explain" {
            msg.delete(&ctx.http).await.ok();
            msg.channel_id.say(&ctx.http, "# ANON - Made by RetroWeb").await.ok();
            msg.channel_id.say(&ctx.http, "This is ANON a custom built selfbot made in RUST. \nRust is fing peak for the following reasons \n- Imagine Python that can make Drivers, Operating systems, Bootloaders, Discord bots, OSINTS, and DDOS Tools \n- Its super secure \n- All C++ code in the US Government is getting replaced by RUST \n- It can be compiled to windows on linux, and to linux on windows with one command. \n- Rust is the 3rd highest paying programming language").await.ok();
        }
        if msg.content.contains("!avatar") {
            msg.delete(&ctx.http).await.ok();
            let user = if let Some(mentioned_user) = msg.mentions.first() {
                mentioned_user
            } else {
                &msg.author
            };
            let avatar_url = user.avatar_url().unwrap();
            msg.channel_id.say(&ctx.http, "# ANON - Made by RetroWeb").await.ok();
            msg.channel_id.say(&ctx.http, format!("Avatar: {}", avatar_url)).await.ok();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    if OWNERLINE == "Made by RetroWeb (; Rust IS PEAK"{
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "{}\n{} {}", 
            LOGO.purple().bold(), 
            DESCRIPTION.purple(), 
            OWNERLINE.purple()
        );
        println!("{}", "Enter your discord token: ".purple());
        let token = read_password().expect("Failed to read password");
        print!("\x1B[2J\x1B[1;1H");
        UniFunc::print_logo();
        let framework = StandardFramework::new();
        let mut client = Client::builder(&token)
            .event_handler(UniFunc)
            .framework(framework)
            .await
            .expect("Err creating client");
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "{}\n{} {}", 
            LOGO.purple().bold(), 
            DESCRIPTION.purple(), 
            OWNERLINE.purple()
        );
        print!("{}", "\nWelcome to Anon selfbot made in RUST. Here are some tools to use in discord:\n- !help: displays this message in discord\n- !spam: Spam a message\n- !RNG: Generate a random number 1-100\n- !ping: Responds PONG with debug logs\n- !explain: Explains what this is!\n- !avatar: Gets a users avatar\n".purple());
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }else {
        println!{
            "\x1B[2J\x1B[1;1HROSES ARE {} VIOLETS ARE {} YOU ARE A {}",
            "RED".red().bold(),
            "BLUE".blue().bold(),
            "SKID NO ONE LIKES YOU!!!".red().bold(),
        }
    }
}