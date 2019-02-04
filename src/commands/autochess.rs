extern crate reqwest;
use ::STEAMIDS;

command!(autochess(_ctx, msg) {
	let mut _steamids = STEAMIDS.lock().unwrap();
	if _steamids.contains_key(&msg.author.id.to_string())
	{
        match get_autochess_json(_steamids[&msg.author.id.to_string()]) {
            Ok(response) => {
            	if response.ranking_info.len() > 0
            	{
            		let ref player: AutochessRanking = response.ranking_info[0];
            		let rank = level_to_rank(player.mmr_level);
            		let _ = msg.reply(format!("Autochess MMR: {}, Rank: {} ({}), Matches: {}.", player.score, rank, player.mmr_level, player.matches).as_str());
            	}

            },
            Err(e) => {
                println!("{:#?}", e);
                let _ = msg.channel_id.say("Error fetching autochess MMR data.");
            }
        }
	}
	else
	{
		let _ = msg.channel_id.say("SteamID not registered, use .steamid");
	}
});

fn get_autochess_json(_steamid: u64) -> Result<AutochessResponse, reqwest::Error> {
    let resp_json: AutochessResponse = reqwest::Client::new()
        .get(format!("http://101.200.189.65:431/dac/ranking/get?player_ids={}", _steamid).as_str())
        .send()?
        .json()?;

    println!("{:#?}", resp_json);
    Ok(resp_json)
}

fn level_to_rank(input: u64) -> std::string::String {
    match input{
        38 => format!("♕Queen"),
        37 => format!("♔King"),
        28...36 => format!("♖Rook {}", input-27),
        19...27 => format!("♗Bishop {}", input-18),
        10...18 => format!("♘Knight {}", input-9),
        1...9 => format!("♙Pawn {}", input),
        _ => format!("Unknown")
    }
}

#[derive(Deserialize,Debug)]
struct AutochessResponse {
    err: u64,
    msg: String,
    ranking_info: Vec<AutochessRanking>
}

#[derive(Deserialize,Debug)]
struct AutochessRanking {
    player: String,
    score: String,
    mmr_level: u64,
    #[serde(rename = "match")] 
    matches: u64
}