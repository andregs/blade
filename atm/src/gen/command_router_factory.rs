use dagger::internal;

use command::CommandRouter;

// because #inject in CommandRouter's constructor
pub struct CommandRouter_Factory {
    commandProvider: internal::Provider<HelloWorldCommand>,
}

// because #inject in CommandRouter's constructor
impl internal::Factory<CommandRouter> for CommandRouter_Factory {
    
    // because params in CommandRouter's constructor
    pub fn new(commandProvider: internal::Provider<HelloWorldCommand>) -> CommandRouter_Factory {
        CommandRouter_Factory {
            commandProvider
        }
    }
    
    pub fn get(&self) -> CommandRouter {
        self.new_instance(self.commandProvider.get())
    }

    pub fn create(commandProvider: internal::Provider<HelloWorldCommand>) -> CommandRouter_Factory {
        println!("CommandRouter_Factory create");
        CommandRouter_Factory::new(commandProvider)
    }

    pub fn new_instance(&self, command: HelloWorldCommand) -> CommandRouter {
        println!("CommandRouter_Factory new_instance");
        CommandRouter::new(command)
    }
}
