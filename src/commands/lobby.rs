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
	}
	else
	{
		let _ = msg.channel_id.say("SteamID not registered, use .steamid");
	}
});


fn get_lobby_json(_steamid: u64) -> Result<SteamResponse, reqwest::Error> {
    let resp_json: SteamResponse = reqwest::Client::new()
        .get(format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", STEAMKEY.read().unwrap(), _steamid).as_str())
        .send()?
        .json()?;

    println!("{:#?}", resp_json);
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