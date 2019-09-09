extern crate dotenv;
extern crate env_logger; // needed to load variables from a .env file

// discord related imports
use serenity::{
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    },
    model::{channel::Message, event::ResumedEvent, gateway::Ready, id::UserId},
    prelude::*,
};

// logging, env vars and hashset
use log::info;
use std::collections::HashSet;
use std::env;

// load in the commands from the sub-module
use invoices::commands::invoice::*;

// event handler
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

group!({
    name: "invoice",
    options: {
        description: "Invoice submission related commands.",
        prefix: "invoice",
        default_command: add
    },
    commands: [explain, add, get],
});

// help setup
#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

// the brains
fn main() {
    match dotenv::dotenv() {
        Ok(_) => println!("Environment loaded!"),
        Err(e) => println!("Environment variables could not be loaded: {}", e),
    }
    env_logger::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create client
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    println!("Client created given the token.");

    // configure client
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.with_whitespace(true).prefix("!"))
            .unrecognised_command(|_, _, unknown_command_name| {
                println!("Could not find command: '{}'", unknown_command_name);
            })
            .help(&MY_HELP)
            .group(&INVOICE_GROUP),
    );
    println!("Configured client successfully. Now running...");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
