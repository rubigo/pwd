#![doc(html_logo_url = "https://rubigo.github.io/coreutils/logo.png")]

extern crate rubigo_pwd;
extern crate clap;
use clap::{App, Arg};
use rubigo_pwd::*;
use std::env;

fn main() {
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::with_name("logical")
            .short("L")
            .long("logical")
            .help("Use working directory from environment"))
        .arg(Arg::with_name("physical")
            .short("P")
            .long("physical")
            .help("Evaluate all symlinks in path (default)"));

    let mat = app.get_matches();

    // from gnu coreutils:
    // POSIX requires a default of -L, but most scripts expect -P. under normal
    // circumstances, we use a default of -P, but if POSIXLY_CORRECT is set, we
    // follow the standard. we have to use var_os here, because var will return 
    // Err(x) if the variable is set but invalid utf8.
    let posixly = env::var_os("POSIXLY_CORRECT").is_some();

    let working_dir = match (mat.is_present("logical"), mat.is_present("physical")) {
        ( true, false) => logical(),
        (false,  true) => physical(),

        // if neither -L and -P are specified, we behave as if -P were set,
        // unless we are forced to conform to the POSIX standard.
        (false, false) => if posixly { logical() } else { physical() },

        // if both -L and -P are specified, the last should apply.
        ( true,  true) => logical() // FIXME
    };

    match working_dir {
        Ok(r) => println!("{}", r.display()),
        Err(x) => {
            eprintln!("Error: {}", x);
            std::process::exit(1);
        }
    }
}
