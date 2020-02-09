use ::attr::inject;
use super::command::*;

pub struct HelloWorldCommand;

impl HelloWorldCommand {

    #[inject]
    pub fn new() -> HelloWorldCommand {
        HelloWorldCommand{}
    }

}

impl Command for HelloWorldCommand {
    fn key(&self) -> String {
        String::from("hello")
    }
    
    fn handle_input(&self, input: Vec<String>) -> Status {
        if input.is_empty() {
            println!("World!");
            Status::HANDLED
        } else {
            Status::INVALID
        }
    }
}
