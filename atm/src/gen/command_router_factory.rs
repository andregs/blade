use dagger::internal;

use super::super::command::*;

// because #inject in CommandRouter's constructor
#[allow(non_camel_case_types)]
pub struct CommandRouter_Factory {
    command_provider: Box<dyn internal::Provider<Box<dyn Command>>>,
}

impl CommandRouter_Factory {
    // because params in CommandRouter's constructor
    fn new(command_provider: Box<dyn internal::Provider<Box<dyn Command>>>) -> CommandRouter_Factory {
        CommandRouter_Factory {
            command_provider
        }
    }
    
    fn create(command_provider: Box<dyn internal::Provider<Box<dyn Command>>>) -> CommandRouter_Factory {
        println!("CommandRouter_Factory create");
        CommandRouter_Factory::new(command_provider)
    }

    fn new_instance(&self, command: Box<dyn Command>) -> CommandRouter {
        println!("CommandRouter_Factory new_instance");
        CommandRouter::new(command)
    }
}

// because #inject in CommandRouter's constructor
impl internal::Factory<CommandRouter> for CommandRouter_Factory {
}

// because #inject in CommandRouter's constructor
impl internal::Provider<CommandRouter> for CommandRouter_Factory {
    
    fn get(&self) -> CommandRouter {
        self.new_instance(self.command_provider.get())
    }
}
