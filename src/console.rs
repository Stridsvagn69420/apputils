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

/// ANSI Color Reset
pub const RESET: &str = "\x1b[0m";

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

/// print! with colors
/// 
/// Drop-in replacement for the [print!]-macro from [std].
#[macro_export]
macro_rules! paint {
    ($color:expr, $($arg:tt)*) => {{
        print!("{}", $color);
        print!($($arg)*);
        print!("{}", $crate::console::RESET);
    }};
}

/// println! with colors
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