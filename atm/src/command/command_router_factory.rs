use crate::command::*;

use ::attr::component;

#[component(modules = ["HelloWorldModule"])]
pub trait CommandRouterFactory {
    fn router(&self) -> CommandRouter;
}
