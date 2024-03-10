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
//! apputils = 0.1.0
//! ```


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
/// use apputils::console::Colors;
/// use apputils::paintln;
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