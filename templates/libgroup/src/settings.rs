use aidoku::{
	alloc::{String, string::ToString},
	imports::defaults::defaults_get,
};

pub fn get_api_url() -> String {
	defaults_get::<String>("apiUrl").unwrap_or_else(|| "https://api.imglib.info".to_string())
}

pub fn get_image_server_url() -> String {
	defaults_get::<String>("imageServerUrl").unwrap_or_else(|| "compress".to_string())
}

pub fn get_cover_quality_url() -> String {
	defaults_get::<String>("coverQuality").unwrap_or_else(|| "default".to_string())
}
