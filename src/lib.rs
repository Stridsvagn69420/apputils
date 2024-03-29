//! A lightweight Rust crate to help you build awesome tools
//!
//! It's designed to be framework-less and relatively simple while providing awesome helper functions for basic tasks that almost any program needs to do. These tasks include reading a config file with multiple paths (user and global), printing with color similar to `println!()` and getting user directories cross-platform.
//! 
//! To add it to your dependencies, either run:
//! ```bash
//! cargo add apputils
//! ```
//! 
//! Or update your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! apputils = "0.1.0"
//! ```
//! 
//! 
//! ## Printing with color
//! ```rust
//! use apputils::Colors;
//! use apputils::paintln;
//! 
//! paintln!(Colors::White, "I'm white.");
//! paintln!(Colors::Black, "I'm black.");
//! paintln!(Colors::Yellow, "I'm yellow.");
//! paintln!(Colors::Red, "I'm red.");
//! paintln!(Colors::Rgb(35, 170, 242), "I'm #23AAF2.");
//! ```

use std::fmt;

/// Console Colors
/// 
/// This enum contains a member for every color, one for True Color via RGB, and a dummy to disable color in-place.
pub enum Colors {
	None,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Rgb(u8, u8, u8)
}

impl Into<u8> for &Colors {
	fn into(self) -> u8 {
		match self {
			Colors::None => 0,
			Colors::Black => 30,
			Colors::Red => 31,
			Colors::Green => 32,
			Colors::Yellow => 33,
			Colors::Blue => 34,
			Colors::Magenta => 35,
			Colors::Cyan => 36,
			Colors::White => 37,
			Colors::Rgb(_, _, _) => 38,
		}
	}
}

impl fmt::Display for Colors {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Colors::None => write!(f, ""),
			Colors::Rgb(r, g, b) => write!(f, "\x1b[38;2;{};{};{}m", r, g, b),
			_ => {
				let cc: u8 = self.into();
				write!(f, "\x1b[{}m", cc)
			}
		}
	}
}

/// [print!] with [Colors]
/// 
/// Drop-in replacement for the [print!]-macro from [std].
#[macro_export]
macro_rules! paint {
    ($color:expr, $($arg:tt)*) => {{
        print!("{}", $color);
        print!($($arg)*);
        print!("\x1b[0m");
    }};
}

/// [println!] with [Colors]
/// 
/// Drop-in replacement for the [println!]-macro from [std].
/// This macro is just a wrapper around the [paint!]-macro with an added newline.
#[macro_export]
macro_rules! paintln {
    ($color:expr, $($arg:tt)*) => {{
        $crate::paint!($color, $($arg)*);
        println!("");
    }};
}

/// Config file helpers
/// 
/// Functions to aid in loading and managing config files.
/// 
/// What you'd want to use 99% of the time is either [cascade](crate::config::cascade) or [local_file](crate::config::cascade),
/// depending on if your app should also read global config files.
pub mod config;

/// User environment wrappers
/// 
/// This submodule's purpose is similar to [dirs](https://crates.io/crates/dirs), [directories](https://crates.io/crates/directories) or [xdg](https://crates.io/crates/xdg).
/// 
/// It currently just includes the XDG User Directories and a Windows translation of them.
pub mod dirs;