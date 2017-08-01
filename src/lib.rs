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
use std::env;
use std::io::{Result, Error, ErrorKind};

/// Returns the logical current working directory.
pub fn logical() -> Result<PathBuf> {
    let pwd = env::var_os("PWD");

    // make sure that PWD exists
    if pwd.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "The PWD environment variable is not set."));
    }

    let pwd = pwd.unwrap();

    // create pathbuf from given pwd
    let path = PathBuf::from(pwd);

    // validate pathbuf
    if !path.is_absolute() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "The PWD environment variable is set to a relative path."));
    }

    // make sure that PWD doesn't contain
    // TODO


    Ok(path)
}

/// Returns the physical current working directory.
pub fn physical() -> Result<PathBuf> {
    env::current_dir()
}
