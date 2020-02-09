use crate::command::*;

use ::attr::component;

#[component]
pub trait CommandRouterFactory {
    fn router(&self) -> CommandRouter;
}
