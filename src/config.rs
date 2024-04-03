use crate::dirs::{config_home, data_home};
use std::env::VarError;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

/// Config file helper
/// 
/// A simple helper for managing config files.
pub struct Cfg;
impl Cfg {
	/// Config file path
	/// 
	/// Returns the formatted path to a config file of an app.
	pub fn path(appname: &str, file: &str) -> PathBuf {
		local_cfg_dir(appname).unwrap().join(file)
	}

	/// Read config file
	/// 
	/// Reads an app's config file into a [String].
	/// This can then be used to parse it via Serde.
	pub fn read(appname: &str, file: &str) -> io::Result<String> {
		fs::read_to_string(Self::path(appname, file))
	}

	/// Save config file
	/// 
	/// Saves to an app's config file. Note that this will override the file!
	pub fn save(appname: &str, file: &str, data: impl AsRef<[u8]>) -> io::Result<()> {
		fs::write(Self::path(appname, file), data)
	}

	/// Global config file path
	/// 
	/// Returns the formatted path to an app's global config file.
	pub fn global_path(appname: &str, file: &str) -> PathBuf {
		global_cfg_dir(appname).unwrap().join(file)
	}

	/// Read global config
	/// 
	/// Reads the global config file of an app into a [String].
	/// Will most likely be parsed, e.g. via Serde.
	pub fn global_read(appname: &str, file: &str) -> io::Result<String> {
		fs::read_to_string(Self::global_path(appname, file))
	}

	/// Save global
	/// 
	/// Saves to an app's global config file. Note that this will override the file!
	pub fn global_save(appname: &str, file: &str, data: impl AsRef<[u8]>) -> io::Result<()> {
		fs::write(Self::global_path(appname, file), data)
	}
}

/// Local config folder
/// 
/// Returns the path to your app's local config folder.  
/// `appname`: The short name of your app in lower case.
pub fn local_cfg_dir(appname: &str) -> Result<PathBuf, VarError> {
	Ok(config_home()?.join(appname))
}

/// Global config folder
/// 
/// Returns the path to your app's global config folder.  
/// `appname`: The short name of your app in lower case.
/// 
/// Due to there not being a proper `/etc` equivalent on Windows,
/// this defaults to the [local](local_cfg_dir) config folder on Windows.
pub fn global_cfg_dir(appname: &str) -> Result<PathBuf, VarError> {
	if cfg!(target_family = "unix") {
		Ok(Path::new("/etc").join(appname))
	} else {
		local_cfg_dir(appname)
	}
}

/// Appdata helper
/// 
/// A simple 
pub struct Appdata;
impl Appdata {
	/// Appdata file path
	/// 
	/// Returns the formatted path to a data file of an app.
	pub fn path(appname: &str, file: &str) -> PathBuf {
		local_data_dir(appname).unwrap().join(file)
	}

	/// Unicode Read
	/// 
	/// Reads a text file from the app's data folder.
	pub fn read_str(appname: &str, file: &str) -> io::Result<String> {
		fs::read_to_string(Self::path(appname, file))
	}
	
	/// Binary Read
	/// 
	/// Reads a binary file from the app's data folder.
	pub fn read(appname: &str, file: &str) -> io::Result<Vec<u8>> {
		fs::read(Self::path(appname, file))
	}

	/// Save data file
	/// 
	/// Saves the data file of an app. Note that this overrides the target file!
	pub fn save(appname: &str, file: &str, data: impl AsRef<[u8]>) -> io::Result<()> {
		fs::write(Self::path(appname, file), data)
	}
}

/// Application data folder
/// 
/// Returns the path to your app's data folder.  
/// `appname`: The short name of your app in lower case.
pub fn local_data_dir(appname: &str) -> Result<PathBuf, VarError> {
	Ok(data_home()?.join(appname))
}