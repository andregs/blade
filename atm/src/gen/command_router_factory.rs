use dagger;

use command::CommandRouter;

static INSTANCE = CommandRouter_Factory{};

// because #inject in CommandRouter's constructor
pub struct CommandRouter_Factory;

// because #inject in CommandRouter's constructor
impl dagger::internal::Factory<CommandRouter> for CommandRouter_Factory {
    pub fn get() -> CommandRouter {
        self.new_instance()
    }

    fn create() -> CommandRouter_Factory {
        println!("CommandRouter_Factory create");
        INSTANCE
    }

    fn new_instance(&self) -> CommandRouter {
        println!("CommandRouter_Factory new_instance");
        CommandRouter{}
    }
}
