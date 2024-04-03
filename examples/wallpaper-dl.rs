use apputils::config::{Cfg, Appdata};
use apputils::{paint, paintln, Colors};

const APP_NAME: &str = "wallpaper-dl";
const CONFIG_FILE: &str = "config.toml";
const DB_FILE: &str = "wallpapers.toml";

fn main() {
	paint!(Colors::Green, "Your wallpaper-dl config: ");
	match Cfg::read(APP_NAME, CONFIG_FILE) {
		Err(_) => paintln!(Colors::YellowBold, "Not found"),
		Ok(x) => println!("\n{x}")
	}

	println!("\n{}\n", "=".repeat(36));

	paint!(Colors::Blue, "Your wallpaper database: ");
	match Appdata::read_str(APP_NAME, DB_FILE) {
		Err(_) => paintln!(Colors::RedBold, "Not found"),
		Ok(x) => println!("\n{x}")
	}
}