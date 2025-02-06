use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};

use log::{set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};

use crate::{utils::directory::get_home_dir, AppResult};

struct Logger {
    file: Mutex<BufWriter<File>>,
}

impl Logger {
    fn new(file: File) -> Self {
        Self {
            file: Mutex::new(BufWriter::new(file)),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().unwrap();
            let red = "\x1b[31m";
            let blue = "\x1b[34m";
            let yellow = "\x1b[33m";
            let purple = "\x1b[35m";
            let clear = "\x1b[0m";
            let file_path = extract_after_src(record.file());
            let line_number = match record.line() {
                Some(line) => line.to_string(),
                None => "".to_string(),
            };

            match record.level() {
                Level::Info => writeln!(file, "\n{}{}{}", blue, record.args(), clear).unwrap(),
                Level::Error => writeln!(
                    file,
                    "\n{}File: {}{}\n{}Line Number: {}{}\n{}Error: {}{}",
                    purple,
                    clear,
                    file_path,
                    purple,
                    clear,
                    line_number,
                    red,
                    record.args(),
                    clear
                )
                .unwrap(),
                Level::Debug => writeln!(
                    file,
                    "\n{}File: {}{}\n{}Line Number: {}{}\n{}{}{}",
                    purple,
                    clear,
                    file_path,
                    purple,
                    clear,
                    line_number,
                    yellow,
                    record.args(),
                    clear,
                )
                .unwrap(),
                _ => writeln!(
                    file,
                    "\n{}File: {}{}\n{}Line Number: {}{}\n{}",
                    purple,
                    clear,
                    file_path,
                    purple,
                    clear,
                    line_number,
                    record.args(),
                )
                .unwrap(),
            }
            file.flush().unwrap();
        }
    }

    fn flush(&self) {
        let mut file = self.file.lock().unwrap();
        file.flush().unwrap();
    }
}

pub fn setup_logging() -> AppResult<()> {
    let mut log_path = PathBuf::from(get_home_dir()?);

    log_path.push(".spotify-client-tui/logs");
    create_dir_all(&log_path)?;

    log_path.push("app.log");
    let file = File::create(&log_path)?;

    let logger = Logger::new(file);

    set_boxed_logger(Box::new(logger))?;
    set_max_level(LevelFilter::Debug);

    Ok(())
}

fn extract_after_src(path: Option<&str>) -> String {
    match path {
        Some(path) => {
            let src_prefix = "src/";

            if let Some(start_index) = path.find(src_prefix) {
                let start_index = start_index + src_prefix.len();

                path[start_index..].to_string()
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    }
}
