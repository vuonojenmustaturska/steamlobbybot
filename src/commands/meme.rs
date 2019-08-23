use std::sync::RwLock;
lazy_static! {
	static ref MUIKEA: RwLock<Option<serenity::model::guild::Emoji>> = RwLock::new(None);
}

command!(meme(_ctx, msg) {

	let mut muikea = MUIKEA.write().unwrap();
	if muikea.is_none() {
		if let Some(arc) = msg.guild_id.unwrap().to_guild_cached() {
			if let Some(guild) = arc.try_read() {
				for (_id, emoji) in &guild.emojis {
					if emoji.name == "muikea" {
						
						*muikea = Some(emoji.clone());
					}
				}
			}
		}
	}
	if muikea.is_some()
	{
		let klooni = muikea.clone().unwrap();
		msg.react(klooni);
	}

});