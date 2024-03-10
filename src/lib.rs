//! A Rust crate for the small but mandatory steps of your app
//! 
//! It's designed to be framework-less and relatively simple while providing awesome helper functions for basic tasks that almost any program needs to do.  
//! These tasks include reading a config file with multiple paths (user and global), printing with color similar to `println!()` and getting user directories cross-platform.


/// Config file helpers
/// 
/// Functions to aid in loading and managing config files.
/// 
/// What you'd want to use 99% of the time is either [cascade](crate::config::cascade) or [local_file](crate::config::cascade),
/// depending on if your app should also read global config files.
pub mod config;

/// Console pretty-print
/// 
/// ## Example
/// 
/// ```rust
/// // Import Colors enum and paintln macro
/// use apputil::console::Colors;
/// use apputil::paintln;
/// 
/// paintln!(Colors::White, "I'm white.");
/// paintln!(Colors::Black, "I'm black.");
/// paintln!(Colors::Yellow, "I'm yellow.");
/// paintln!(Colors::Red, "I'm red.");
/// paintln!(Colors::Rgb(35, 170, 242), "I'm #23AAF2.");
/// ```
pub mod console;

/// User environment wrappers
/// 
/// This submodule's purpose is similar to [dirs](https://crates.io/crates/dirs), [directories](https://crates.io/crates/directories) or [xdg](https://crates.io/crates/xdg).
/// 
/// It currently just includes the XDG User Directories and a Windows translation of them.
pub mod dirs;