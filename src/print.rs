use std::io::prelude::*;

use term;


pub fn error(message: &str) {
    let mut t = term::stderr().unwrap();
    t.fg(term::color::BRIGHT_RED).unwrap();
    writeln!(t, "{}", message).unwrap();
    t.reset().unwrap();
}


pub fn success(message: &str) {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    writeln!(t, "{}", message).unwrap();
    t.reset().unwrap();
}
