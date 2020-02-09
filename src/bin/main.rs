use std::io::{self, BufRead};

extern crate dagger;

fn main() {
    let command_router = dagger::CommandRouter::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        command_router.route(line);
    }
}
