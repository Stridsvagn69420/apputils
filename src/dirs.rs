use std::path::PathBuf;
use std::env::var;

/// Home Directory
///
/// Alternative to [std::env::home_dir].  
/// On Windows, it reads the `USERPFOILE` environment variable.  
/// On Unix-like systems, it reads the `HOME` environment variable.
/// 
/// Can be unwrapped, because most programs will run in an environment where the required envvars are set.
/// You wouldn't `unset $HOME`!
pub fn home_dir() -> PathBuf {
	#[cfg(target_family = "windows")]
	let envstr = var("USERPROFILE");
	#[cfg(target_family = "unix")]
	let envstr = var("HOME");
	PathBuf::from(envstr.unwrap_or_default())
}

/// Directory PathBuf Builder
/// 
/// The core behavior of the functions in this submodule.
/// 
/// `xdg`: The XDG environment variable  
/// `home_alt`: The default value in the home directory  
/// `win`: Windows environment variable  
fn xdg_path(xdg: &str, home_alt: &str, win: &str) -> PathBuf {
	if cfg!(target_family = "unix") {
		let confighome = match var(xdg) {
    		Ok(s) => PathBuf::from(s),
    		Err(_) => home_dir().join(home_alt),
		};
		confighome
	} else {
		PathBuf::from(var(win).unwrap_or_default())
	}
}

/// Config Directory
///
/// Gets the user config directory.
/// 
/// On Unix, it complies with `XDG_CONFIG_HOME`.  
/// On Windows, it returns the value of `APPDATA`.
pub fn config_home() -> PathBuf {
	xdg_path("XDG_CONFIG_HOME", ".config", "APPDATA")
}

/// Cache Directory
///
/// Gets the user cache directory.  
/// On Unix, it complies with `XDG_CONFIG_HOME`.  
/// On Windows, it returns the value of `TEMP`.
pub fn cache_home() -> PathBuf {
	xdg_path("XDG_CACHE_HOME", ".cache", "TEMP")
}

/// Application Data Directory
///
/// Gets the user data directory.  
/// On Unix, it complies with `XDG_DATA_HOME`.  
/// On Windows, it returns the value of `APPDATA`.
pub fn data_home() -> PathBuf {
	xdg_path("XDG_DATA_HOME", ".local/share", "APPDATA")
}

/// Application State Directory
///
/// Gets the app state directory.  
/// On Unix, it complies with `XDG_STATE_HOME`.  
/// On Windows, it returns the value of `LOCALAPPDATA`.
pub fn state_home() -> PathBuf {
	xdg_path("XDG_STATE_HOME", ".local/state", "LOCALAPPDATA")
}