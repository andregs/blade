use crate::command::*;

// because #component in CommandRouterFactory trait definition
pub struct DaggerCommandRouterFactory;

// because #component in CommandRouterFactory trait definition
impl DaggerCommandRouterFactory {

    pub fn builder() -> Builder {
        println!("DaggerCommandRouterFactory builder");
        Builder::new()
    }

    pub fn create() -> Box<dyn CommandRouterFactory> {
        println!("DaggerCommandRouterFactory create");
        Builder::new().build()
    }

}

impl CommandRouterFactory for DaggerCommandRouterFactory {

    fn router(&self) -> CommandRouter {
        println!("DaggerCommandRouterFactory router");
        CommandRouter::new(Box::new(HelloWorldCommand::new()))
    }

}

// because #component in CommandRouterFactory trait definition
pub struct Builder;
impl Builder {
    fn new() -> Builder {
        Builder {}
    }

    fn build(&self) -> Box<dyn CommandRouterFactory> {
        Box::new(DaggerCommandRouterFactory {})
    }
}
