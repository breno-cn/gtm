mod tm;
mod yaml;
mod tui_app;

use std::env;

use tui_app::app::{App};

fn main() -> Result<(), &'static str> {
    let filepath = env::args().collect::<Vec<String>>()[1].clone();
    let mut app = App::new();
    println!("machine: {}", filepath);

    app.init(filepath);
    app.run();

    Ok(())
}
