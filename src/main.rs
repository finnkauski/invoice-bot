extern crate env_logger;
extern crate dotenv; // needed to load variables from a .env file

// discord related imports
use serenity::{
    framework::standard::{
        StandardFramework,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions, help_commands},
    model::{event::ResumedEvent, gateway::Ready, channel::{Message}, id::UserId},
    prelude::*,
};

// logging, env vars and hashset
use std::env;
use log::info;
use std::{collections::HashSet};

// load in the commands from the sub-module
mod commands;
use commands::{invoice::*};

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
        default_command: explain
    },
    commands: [explain, add],
});

// help setup
#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

// the brains
fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create client
    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    // configure client
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c
                       .with_whitespace(true)
                       .prefix("!"))
            .unrecognised_command(|_, _, unknown_command_name| {
                println!("Could not find command: '{}'", unknown_command_name);
            })
            .help(&MY_HELP)
            .group(&INVOICE_GROUP));


    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
