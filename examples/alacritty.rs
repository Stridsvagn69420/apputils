use apputils::config::Cfg;
use apputils::Colors;
use apputils::paintln;

fn main() {
	paintln!(Colors::Rgb(42, 164, 69), "Attempting to read alacritty config file...");
	match Cfg::read("alacritty", "alacritty.toml") {
		Ok(data) => println!("Your alacritty config:\n{}", data),
		Err(_) => paintln!(Colors::Red, "You don't seem to have an alacritty config!"),
	}
}