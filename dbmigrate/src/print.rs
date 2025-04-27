use std::io;
use std::io::prelude::*;

use term;

pub fn error(message: &str) {
    if let Some(mut t) = term::stderr() {
        match t.fg(term::color::BRIGHT_RED) {
            Ok(_) => {
                writeln!(t, "{}", message).unwrap();
                t.reset().unwrap();
            }
            Err(_) => writeln!(t, "{}", message).unwrap(),
        };
    } else {
        writeln!(io::stderr(), "{}", message).unwrap();
    }
}

pub fn success(message: &str) {
    if let Some(mut t) = term::stdout() {
        match t.fg(term::color::GREEN) {
            Ok(_) => {
                writeln!(t, "{}", message).unwrap();
                t.reset().unwrap();
            }
            Err(_) => writeln!(t, "{}", message).unwrap(),
        };
    } else {
        println!("{}", message);
    }
}
