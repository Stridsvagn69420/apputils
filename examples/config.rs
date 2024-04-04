use apputils::config::Cfg;
use apputils::{Colors, paintln};
use std::io;

const APP: &str = "apputils";
const FILE: &str = "test.txt";
const PAYLOAD: &str = "Hello, Wörld!";

fn main() -> io::Result<()> {
	paintln!(Colors::Blue, "Saving test file...");
	Cfg::save(APP, FILE, PAYLOAD)?;

	paintln!(Colors::Cyan, "Reading test file...");
	let text = Cfg::read(APP, FILE)?;

	println!("Test file: {text}");
	assert_eq!(text, PAYLOAD.to_owned());
	Ok(())
}