pub struct App{
    // The name of the program.
    pub name: String
}

pub fn build_app() -> App {
    App {
        name: String::from("JVM-R")
    }
}

