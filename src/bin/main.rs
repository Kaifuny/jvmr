extern crate jvmr;

use jvmr::cli::app::App;

fn main() {
    let _app = App {
        name: String::from("JVM-R")
    };
    println!("{}", _app.name)
}
