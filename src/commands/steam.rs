extern crate reqwest;
use std::collections::HashMap;
use ::STEAMIDS;
use ::STEAMKEY;
use url::Url;
use std::sync::MutexGuard;
use std::fs::File;
use std::io::Write;

command!(steamid(_ctx, msg, args) {
	if let Ok(_newid) = args.single_n::<u64>() {
		add_steamid(msg.author.id.to_string(), _newid);
   		let _ = msg.channel_id.say(format!("Steamid of {} set to {}", msg.author.id, _newid));
	} else {
		if let Ok(url) = args.single_n::<Url>() {
			let mut path_segments = url.path_segments().ok_or_else(|| "cannot be base")?;
			match path_segments.next() {
				Some("id") => {
					if let Ok(_newid) = steamid_from_vanityurl(path_segments.next().unwrap().to_string()) {
						add_steamid(msg.author.id.to_string(), _newid);
						let _ = msg.channel_id.say(format!("Steamid of {} set to {:#?}", msg.author.id, _newid));
					} else {
						let _ = msg.channel_id.say(format!("Couldn't get a SteamId from {}", url));
					}
					
				},
				Some("profiles") => {
					if let Ok(_newid) = path_segments.next().unwrap().parse::<u64>()
					{
						add_steamid(msg.author.id.to_string(), _newid);
						let _ = msg.channel_id.say(format!("Steamid of {} set to {:#?}", msg.author.id, _newid));
					}
				}
                _ => {
                    let _ = msg.channel_id.say("Invalid URL");
                }

			}
		}
	}
});

fn write_steamids(steamids: MutexGuard<HashMap<String,u64>>) {
    if let Ok(mut f) = File::create("steamids.json") {
        f.write_all(serde_json::to_string_pretty(&*steamids).unwrap().as_bytes());
        f.sync_all();
    }
}

fn add_steamid(userid: String, _steamid: u64) {
    let mut steamids = STEAMIDS.lock().unwrap();
    steamids.insert(userid, _steamid);
    write_steamids(steamids);
}

fn steamid_from_vanityurl(vanityurl: String) -> Result<u64, String> {
    let resp_result: Result<SteamResponse, String> = match reqwest::Client::new()
    	.get(format!("https://api.steampowered.com/ISteamUser/ResolveVanityURL/v1/?key={}&vanityurl={}", STEAMKEY.read().unwrap(), vanityurl).as_str())
        .send() {
            Ok(mut response) => {
                match response.json() {
                    Ok(json) => Ok(json),
                    Err(e) => Err(format!("{}", e))
                }
            }
            Err(e) => Err(format!("{}", e))
        };

    match resp_result {
       	Ok(response) => {
    		println!("{:#?}", response);
        	if (response.response.success == 1) {
        		if let Ok(usteamid) = response.response.steamid.parse::<u64>() {
        			Ok(usteamid)
        		} else {
        			Err("Steam API response did not contain steamid for vanityurl".to_string())
        		}
        	} else {
        		Err("Steam API response was not successful".to_string())
        	}
        },
        Err(error) => {
        	Err(format!("{}", error))
        }
    }
    	


    
    // Object(
    //     {
    //         "body": String(
    //             "https://docs.rs/reqwest"
    //         ),
    //         "id": Number(
    //             101
    //         ),
    //         "title": String(
    //             "Reqwest.rs"
    //         ),
    //         "userId": Number(
    //             1
    //         )
    //     }
    // )
    
}


#[derive(Deserialize,Debug)]
struct SteamResponse {
    response: ResponseObj
}

#[derive(Deserialize,Debug)]
struct ResponseObj {
	success: u32,
	#[serde(default)]
	steamid: String
}

