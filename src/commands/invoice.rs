use serenity::prelude::*;
use serenity::model::{
    prelude::*,
    channel::Attachment
};
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

#[command]
pub fn explain(ctx: &mut Context, msg: &Message) -> CommandResult {
    let reply = "Hey!\n\nThis it the mYi receipt processing system. For more info ask Domi!";
    match msg.channel_id.say(&ctx.http, reply) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Could not explain invoice command to user. Error: {}", e);
            Ok(())
        }
    }
}

#[command]
pub fn add(ctx: &mut Context, msg: &Message) -> CommandResult {

    fn handle_attachment(content: &[u8], attachment: &Attachment) -> Result<(), std::io::Error> {
        let mut handle = File::create(&attachment.filename)?;
        handle.write_all(content)?; // TODO: fix the unwraps
        Ok(())
     };

    let fs = &msg.attachments;

    match (fs.len(), fs.get(0)) {
        (0, _ ) => {
            match msg.channel_id.say(&ctx.http, "You haven't provided any attachment. Aborting...") {
                Ok(_) => (),
                Err(e) => println!("Could not reply to user: {}", e)
            }
        },
        (1, Some(attachment)) => {
            if let Ok(content) = attachment.download() {
                match handle_attachment(&content, &attachment) {
                    Ok(_) => {
                        println!("Written file {} sucessfully", &attachment.filename);
                    },
                    Err(e) => println!("Cannot save file the file provided by the user. Error: {}", e)
                }
            } else {
                match msg.channel_id.say(&ctx.http, format!("Your file ({}) is being processed.", &attachment.filename)) {
                Ok(_) => (),
                Err(e) => println!("Could not reply to user: {}", e)
            }

            }
        }
        (_, _) => {
            match msg.channel_id.say(&ctx.http, "You haven't provided any attachment. Aborting...") {
                Ok(_) => (),
                Err(e) => println!("Could not reply to user: {}", e)
            }
        }
    }
    Ok(())
}

