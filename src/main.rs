mod app;
mod buffer;
mod cmd_dispatcher;
mod cursor;
mod input_handler;
mod logger;
mod renderer;
mod state;

use app::Application;
use logger::Logger;
use std::{
    env,
    io::{BufWriter, stdout},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init()?;

    Logger::log(String::from("[main] Start App"));

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "[main] Argument len should be 2. {{ len:{}, args:{:?}}}",
            args.len(),
            args
        );
    }

    let mut editor = Application::new(BufWriter::new(stdout()), &args[1]);
    editor.init()?;
    editor.run()?;
    editor.drop()?;

    Logger::log(String::from("[main] Terminate App"));

    Ok(())
}
