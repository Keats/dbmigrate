use std::io;
use std::io::prelude::*;

use term;

pub fn error(message: &str) {
    if let Some(mut t) = term::stderr() {
        t.fg(term::color::BRIGHT_RED).unwrap();
        writeln!(t, "{}", message).unwrap();
        t.reset().unwrap();
    } else {
        writeln!(io::stderr(), "{}", message).unwrap();
    }
}


pub fn success(message: &str) {
    if let Some(mut t) = term::stdout() {
        t.fg(term::color::GREEN).unwrap();
        writeln!(t, "{}", message).unwrap();
        t.reset().unwrap();
    } else {
        println!("{}", message);
    }
}
