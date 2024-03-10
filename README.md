# apputil
A Rust crate for the small but mandatory steps of your app

It's designed to be framework-less and relatively simple while providing awesome helper functions for basic tasks that almost any program needs to do. These tasks include reading a config file with multiple paths (user and global), printing with color similar to `println!()` and getting user directories cross-platform.

## Submodules
- `dirs`: User directories using environment variables
- `console`: Console pretty-print
- `config`: Config file helpers

There's currently one example, `alacritty`, that you can run. It will both use the `config`-module (thus `dirs`) as well as `console`.