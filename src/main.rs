mod app;
mod buffer;
mod cursor;
mod highlighter;
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

    Logger::log(String::from("[main] Start App"))?;

    // Check arguments before initializing terminal
    let args: Vec<String> = env::args().collect();

    let mut editor = Application::new(BufWriter::new(stdout()), &args[1])?;

    editor.init().unwrap();
    editor.run().unwrap();
    editor.drop().unwrap();

    /*
        // Write the modified content back to the file
        let f_write = File::create(filename)?;
        let mut buf_writer = BufWriter::new(f_write);
        for i in 0..buffer.len() {
            buf_writer.write_all(buffer.get(i).as_bytes())?;
            buf_writer.write_all(b"\n")?;
        }
        buf_writer.flush()?;
    */
    Logger::log(String::from("[main] Terminate App"))?;

    Ok(())
}
