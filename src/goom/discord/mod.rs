use bevy_discord_presence::config::{RPCConfig, RPCPlugin};
pub const DISCORD_APP_ID: u64 = 388338871475240965;

pub fn discord() -> RPCPlugin {
		let config = RPCConfig {
			app_id: DISCORD_APP_ID,
			show_time: true,
			..Default::default()
		};
		return RPCPlugin(config);
	}
