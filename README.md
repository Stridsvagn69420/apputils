# apputils
A lightweight Rust crate to help you build awesome tools

Check out [the docs](https://docs.rs/apputils) to know what it can do. To add it to your dependencies, either run:
```bash
cargo add apputils
```

Or update your `Cargo.toml`:
```toml
[dependencies]
apputils = "0.1.5"
```

## Categories
- `dirs`: User directories using environment variables
- `config`: Config file helpers

An example 
```rust
use apputils::config::local_file;
use apputils::Colors;
use apputils::paintln;

fn main() {
	paintln!(Colors::Rgb(42, 164, 69), "Attempting to read alacritty config file...");

	match local_file("alacritty", "alacritty.toml") {
		Ok(data) => println!("Your alacritty config:\n{}", data),
		Err(_) => paintln!(Colors::Red, "You don't seem to have an alacritty config!")
	}

	// You can also print with bold colors
	paintln!(Colors::MagentaBold, "I use Gentoo, btw.");
}
```