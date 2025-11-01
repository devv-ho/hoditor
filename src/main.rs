use logger::Logger;
use std::{thread, time};

pub mod logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init()?;

    let one_second = time::Duration::from_secs(1);

    for i in 0..10 {
        thread::sleep(one_second);
        Logger::log(format!("{i}"))?;
    }
    Ok(())
}
