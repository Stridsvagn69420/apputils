use crate::dirs::config_home;
use std::env::VarError;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

/// Local config folder
/// 
/// Returns the path to your app's local config folder.  
/// `appname`: The short name of your app in lower case.
pub fn local_dir(appname: &str) -> Result<PathBuf, VarError> {
	Ok(config_home()?.join(appname))
}

/// Local config file
/// 
/// Loads the config resulting from [local_dir] joined with the `cfg` parameter as a [String] to be parsed.
/// 
/// Unlike [cascade], this returns an [io::Result] instead of an [Option], as it is just a glorified [fs::read_to_string].
pub fn local_file(appname: &str, cfg: &str) -> io::Result<String>{
	let path = match local_dir(appname) {
		Ok(p) => p,
		Err(e) => return Err(io::Error::new(io::ErrorKind::NotFound, e.to_string()))
	};
	fs::read_to_string(path.join(cfg))
}

/// Global config folder
/// 
/// Returns the path to your app's global config folder.  
/// `appname`: The short name of your app in lower case.
/// 
/// Due to there not being a proper `/etc` equivalent on Windows,
/// this defaults to the [local](local_dir) config folder on Windows.
pub fn global_dir(appname: &str) -> Result<PathBuf, VarError> {
	if cfg!(target_family = "unix") {
		Ok(Path::new("/etc").join(appname))
	} else {
		local_dir(appname)
	}
}

/// Cascading config read wrapper
/// 
/// `appname`: The short name of your app in lower case.  
/// `cfg`: The name of the config file to be read
/// 
/// Attempts to read the given file from your app's config paths in a cascading way.  
/// It reads the file instead of just checking the existence of it due to the [TOCTOU problem](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use).
/// 
/// Due to the simplicity of this program, a [None] can mean that the config does not exists or also that it exists but that it cannot be opened.  
/// The latter seems like an edge case that needs to be fixed on the user-side, i.e. user variables or file permissions are messed up.
pub fn cascade(appname: &str, cfg: &str) -> Option<String> {
	// Possible config dir paths
	let paths = [
		local_dir(appname).ok(),
		global_dir(appname).ok()
	];

	// Read the first found config file
	paths.into_iter()
		.flatten()
		.flat_map(|path| fs::read_to_string(path.join(cfg)).ok())
		.next()
}