use ::attr::{module, binds};
use super::{HelloWorldCommand, Command};

#[module]
pub trait HelloWorldModule {

    #[binds]
    fn hello_world_command(command: HelloWorldCommand) -> dyn Command;
}

