use anyhow::Context;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::{Mutex, OnceLock},
};

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub struct Logger {
    pub file: Mutex<File>,
}

impl Logger {
    const LOG_DIR: &'static str = "debug/logs";
    const COLUMN_INTERVAL: &'static str = "    ";

    pub fn init() -> Result<(), Box<dyn Error>> {
        let crate_path = std::env::var("CARGO_MANIFEST_DIR")
            .with_context(|| format!("Failed to get crate path."))?;

        let log_dir_abs_path = Path::new(&crate_path).join(Path::new(Self::LOG_DIR));

        if !log_dir_abs_path.exists() {
            fs::create_dir(&log_dir_abs_path)
                .with_context(|| format!("Failed to create directory. {:?}", &log_dir_abs_path))?;
        }

        let log_name = format!("log-{}", Self::current_time("_"));
        let log_path = log_dir_abs_path.join(Path::new(&log_name));

        let log_file: File = if !log_path.exists() {
            File::create(&log_path)
                .with_context(|| format!("Failed to create log file. {:?}", &log_path))
        } else {
            File::open(&log_path)
                .with_context(|| format!("Failed to open log file. {:?}", &log_path))
        }?;

        LOGGER
            .set(Self {
                file: Mutex::new(log_file),
            })
            .map_err(|_| anyhow::anyhow!("Logger already initialized"))?;

        Ok(())
    }

    pub fn log(text: String) -> Result<(), Box<dyn Error>> {
        if let Some(logger) = LOGGER.get() {
            // Format the log message
            let timestamp = Self::current_time(Self::COLUMN_INTERVAL);
            let log_line = format!("{}{}{}\n", timestamp, Self::COLUMN_INTERVAL, text);

            // Write to file
            let mut file = logger.file.lock().unwrap();
            file.write_all(log_line.as_bytes())?;
            file.flush()?;
        }

        Ok(())
    }

    fn current_time(delimiter: &str) -> String {
        chrono::Local::now()
            .format(format!("%Y-%m-%d{delimiter}%H:%M:%S%.6f").as_str())
            .to_string()
    }
}
