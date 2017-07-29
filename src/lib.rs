#![doc(html_logo_url = "https://rubigo.github.io/coreutils/logo.png")]
#![warn(missing_docs)]

/// Version of this crate. It is parsed from the environment variable that
/// cargo sets when building the crate.
pub const VERSION: &'static str         = env!("CARGO_PKG_VERSION");

/// Name of this tool.
pub const NAME: &'static str            = "pwd";

/// A description of this tool.
pub const DESCRIPTION: &'static str     = "Print full path to the current \
working directory.";

use std::path::PathBuf;
use std::env::current_dir;
use std::io::Result;

/// Returns the logical current working directory.
pub fn logical() -> Result<PathBuf> {
    current_dir()
}

/// Returns the physical current working directory.
pub fn physical() -> String {
    String::new()
}
