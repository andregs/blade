use dagger;

use command::HelloWorldCommand;

static INSTANCE = HelloWorldCommand_Factory{};

// because #inject in HelloWorldCommand's constructor
pub struct HelloWorldCommand_Factory;

// because #inject in HelloWorldCommand's constructor
impl dagger::internal::Factory<HelloWorldCommand> for HelloWorldCommand_Factory {
    pub fn get() -> HelloWorldCommand {
        self.new_instance()
    }

    pub fn create() -> HelloWorldCommand_Factory {
        println!("HelloWorldCommand_Factory create");
        INSTANCE
    }

    fn new_instance(&self) -> HelloWorldCommand {
        println!("HelloWorldCommand_Factory new_instance");
        HelloWorldCommand{}
    }
}
