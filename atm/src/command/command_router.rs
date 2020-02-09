use std::collections::HashMap;

use ::attr::inject;
use super::*;

pub struct CommandRouter {
    
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRouter {
    #[inject]
    pub fn new(command: HelloWorldCommand) -> CommandRouter {
        println!("CommandRouter new");
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert(command.key(), Box::new(command));
        
        CommandRouter {
            commands,
        }
    }

    pub fn route(&self, input: String) -> Status {
        // println!("CommandRouter route");
        if input.is_empty() {
            return self.invalid_command(input);
        }

        let mut parts = input.split_whitespace();

        if let Some(command_key) = parts.next() {

            if let Some(command) = self.commands.get(command_key) {
                let args = parts.map(str::to_string).collect();
                let status = command.handle_input(args);
                
                if let Status::INVALID = status {
                    println!("{}: invalid arguments", command_key);
                }

                return status;
            }
        }

        self.invalid_command(input)
    }

    fn invalid_command(&self, input: String) -> Status {
        println!("couldn't understand \"{}\". please try again.", input);
        
        Status::INVALID
    }

}
