#![doc(html_logo_url = "https://rubigo.github.io/coreutils/logo.png")]

extern crate rubigo_pwd;
extern crate clap;
use clap::{App, Arg};
use rubigo_pwd::*;
use std::io::stdout;

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::with_name("logical")
            .short("L")
            .long("logical")
            .help("Use working directory from environment"))
        .arg(Arg::with_name("physical")
            .short("P")
            .long("physical")
            .help("Evaluate all symlinks in path (default)"))
        .get_matches();

    if matches.is_present("logical") {
        println!("{}", logical().display());
    } else {
        // default
        println!("{}", physical());
    }
}
