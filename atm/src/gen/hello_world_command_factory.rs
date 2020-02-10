use dagger;

use super::super::command::*;

static INSTANCE: &HelloWorldCommand_Factory = &HelloWorldCommand_Factory{};

// because #inject in HelloWorldCommand's constructor
#[allow(non_camel_case_types)]
pub struct HelloWorldCommand_Factory;

impl HelloWorldCommand_Factory {
    fn create() -> &'static HelloWorldCommand_Factory {
        println!("HelloWorldCommand_Factory create");
        INSTANCE
    }

    fn new_instance(&self) -> HelloWorldCommand {
        println!("HelloWorldCommand_Factory new_instance");
        HelloWorldCommand{}
    }
}

// because #inject in HelloWorldCommand's constructor
impl dagger::internal::Factory<HelloWorldCommand> for HelloWorldCommand_Factory {
}

// because #inject in HelloWorldCommand's constructor
impl dagger::internal::Provider<HelloWorldCommand> for HelloWorldCommand_Factory {
    fn get(&self) -> HelloWorldCommand {
        self.new_instance()
    }
}
