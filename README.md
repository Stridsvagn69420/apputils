# apputils
A lightweight Rust crate to help you build awesome tools

It's designed to be framework-less and relatively simple while providing awesome helper functions for basic tasks that almost any program needs to do. These tasks include reading a config file with multiple paths (user and global), printing with color similar to `println!()` and getting user directories cross-platform.

To add it to your dependencies, either run:
```bash
cargo add apputils
```

Or update your `Cargo.toml`:
```toml
[dependencies]
apputils = "0.1.0"
```

## Submodules
- `dirs`: User directories using environment variables
- `console`: Console pretty-print
- `config`: Config file helpers

There's currently one example, `alacritty`, that you can run. It uses both the `config`-module (thus `dirs`) as well as `console`:
```rust
use apputil::config::cascade;
use apputil::console::Colors;
use apputil::paintln;

fn main() {
	paintln!(Colors::Rgb(42, 164, 69), "Attempting to read alacritty config file...");
	match cascade("alacritty", "alacritty.toml") {
		Some(data) => println!("Your alacritty config:\n{}", data),
		None => paintln!(Colors::Red, "You don't seem to have an alacritty config!"),
	}
}
```