use std::io::{self, BufRead};

extern crate atm;
use atm::gen;

fn main() {
    let command_router_factory = gen::DaggerCommandRouterFactory::create();
    let command_router = command_router_factory.router();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        command_router.route(line);
    }

    println!("yeah, whatever.");
}
