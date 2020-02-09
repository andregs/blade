pub trait Command {

    fn key(&self) -> String;
    
    fn handle_input(&self, input: Vec<String>) -> Status;
}

pub enum Status {
    INVALID,
    HANDLED,
}
