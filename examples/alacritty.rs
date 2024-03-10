use apputils::config::cascade;
use apputils::console::Colors;
use apputils::paintln;

fn main() {
	paintln!(Colors::Rgb(42, 164, 69), "Attempting to read alacritty config file...");
	match cascade("alacritty", "alacritty.toml") {
		Some(data) => println!("Your alacritty config:\n{}", data),
		None => paintln!(Colors::Red, "You don't seem to have an alacritty config!"),
	}
}