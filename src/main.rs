#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate serde_json;
extern crate env_logger;
extern crate kankyo;
extern crate url;

mod commands;

use std::{collections::HashSet, collections::HashMap, env};
use std::sync::Mutex;
use std::fs::File;

use serenity::{
    framework::StandardFramework,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
    http,
};


use std::sync::RwLock;


lazy_static! {
    static ref STEAMIDS: Mutex<HashMap<String,u64>> = Mutex::new({
    	match File::open("steamids.json") {
    		Ok(file) => {
    			match serde_json::from_reader(file)
    			{
    				Ok(result) => result,
    				_ => HashMap::new()
    			}
    		},
    		_ => HashMap::new()
    	}
    });
    static ref STEAMKEY: RwLock<String> = RwLock::new(String::new());
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}


fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init().expect("Failed to initialize env_logger");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a Discord token in the environment");

    {
        let mut _steamkey = STEAMKEY.write().unwrap();
        *_steamkey = env::var("STEAM_KEY").expect("Expected a Steam Web API key in the environment");
    }

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("."))
        .command("steamid", |c| c.cmd(commands::steam::steamid))
        .command("lobby", |c| c.cmd(commands::lobby::lobby))
        .command("autochess", |c| c.cmd(commands::autochess::autochess))
        );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}