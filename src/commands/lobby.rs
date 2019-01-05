extern crate reqwest;
use ::STEAMIDS;
use ::STEAMKEY;


command!(lobby(_ctx, msg) {
	let mut _steamids = STEAMIDS.lock().unwrap();
	if _steamids.contains_key(&msg.author.id.to_string())
	{
        match get_lobby_json(_steamids[&msg.author.id.to_string()]) {
            Ok(response) => {
                let ref player: Player = response.response.players[0];
                match &player.gameid {
                    Some(gameid) => {
                        match &player.lobbyid {
                            Some(lobbyid) => {
                                let _ = msg.channel_id.say(format!("steam://joinlobby/{}/{}/{}", gameid, lobbyid, player.steamid));
                            },
                            None => {
                                let _ = msg.channel_id.say("Lobby not found.");
                            }

                        }
                    }
                    None => {
                        let _ = msg.channel_id.say("Player not in a game.");
                    }
                }
            },
            Err(e) => {
                println!("{:#?}", e);
                let _ = msg.channel_id.say("Error fetching lobby data.");
            }
        }
		/*if let Ok(response) = get_lobby_json(_steamids[&msg.author.id]) {
            let ref player: Player = response.response.players[0];
			let _ = msg.channel_id.say(format!("steam://joinlobby/{}/{}/{}", player.gameid.unwrap(), player.steamid, player.lobbyid.unwrap()));
        } else {
            let _ = msg.channel_id.say("Error fetching lobby data.");
        }*/
	}
	else
	{
		let _ = msg.channel_id.say("SteamID not registered, use .steamid");
	}
});

/*fn handle_lobby_response(json: serde_json::Value) -> Result<Player, Error> {
    match json {
        Object(Map<String, Value>) => match json["response"] {
            Object(Map<String, Value>) => match json["response"]["players"] {
                Array(Vec<Value>) => 
        }
    }
} */

fn get_lobby_json(_steamid: u64) -> Result<SteamResponse, reqwest::Error> {
    let resp_json: SteamResponse = reqwest::Client::new()
        .get(format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", STEAMKEY.read().unwrap(), _steamid).as_str())
        .send()?
        .json()?;

    println!("{:#?}", resp_json);
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
    Ok(resp_json)
}



#[derive(Deserialize,Debug)]
struct SteamResponse {
    response: ResponseObj
}

#[derive(Deserialize,Debug)]
struct ResponseObj {
    players: Vec<Player>
}

#[derive(Deserialize,Debug)]
struct Player {
    gameid: Option<String>,
    #[serde(rename = "personaname")] 
    name: String,
    steamid: String,
    #[serde(rename = "lobbysteamid")] 
    lobbyid: Option<String>
}


/*
Object(
    {
        "response": Object(
            {
                "players": Array(
                    [
                        Object(
                            {
                                "avatar": String(
                                    "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/fa/fa7c116fbc6413db0b0222eaa03bb086a7b5033c.jpg"
                                ),
                                "avatarfull": String(
                                    "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/fa/fa7c116fbc6413db0b0222eaa03bb086a7b5033c_full.jpg"
                                ),
                                "avatarmedium": String(
                                    "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/fa/fa7c116fbc6413db0b0222eaa03bb086a7b5033c_medium.jpg"
                                ),
                                "commentpermission": Number(
                                    1
                                ),
                                "communityvisibilitystate": Number(
                                    3
                                ),
                                "gameextrainfo": String(
                                    "GUILTY GEAR Xrd -REVELATOR-"
                                ),
                                "gameid": String(
                                    "520440"
                                ),
                                "lastlogoff": Number(
                                    1546221575
                                ),
                                "lobbysteamid": String(
                                    "109775240935857845"
                                ),
                                "loccountrycode": String(
                                    "FI"
                                ),
                                "personaname": String(
                                    "Naksu"
                                ),
                                "personastate": Number(
                                    1
                                ),
                                "personastateflags": Number(
                                    0
                                ),
                                "primaryclanid": String(
                                    "103582791429573148"
                                ),
                                "profilestate": Number(
                                    1
                                ),
                                "profileurl": String(
                                    "https://steamcommunity.com/id/vuonojenmustaturska/"
                                ),
                                "steamid": String(
                                    "76561197971401911"
                                ),
                                "timecreated": Number(
                                    1101977199
                                )
                            }
                        )
                    ]
                )
            }
        )
    }
)
*/