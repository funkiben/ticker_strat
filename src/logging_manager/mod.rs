mod time_manager;

use std::fs::{create_dir, read_dir, remove_file, DirEntry, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use log::{Metadata, Record, Level, SetLoggerError, LevelFilter};

/// Struct holding sender to dedicated logging thread
pub struct LoggingService {
    sender: mpsc::SyncSender<LoggingCommands>,
}

// struct holding the body of a message to log
struct MessageBody {
    level: String,
    content: String,
}

// commands that can be sent to the logging service
enum LoggingCommands {
    Kill,
    Message(MessageBody),
}

/// Configuration struct for Logging service
pub struct LoggingConfig {
    /// Path from executable to directory to be used for log files
    pub logging_directory: &'static Path,
    /// The maximum size of the logging directory in bytes
    pub max_dir_size: usize,
}

impl LoggingService {

    /// Create a new LoggingService instance holding the sender to the dedicated logging thread.
    ///
    /// # Arguments
    ///
    /// * `options` - LoggingConfig struct containing the options for the new logging service instance.
    ///
    pub fn new(options: LoggingConfig) -> LoggingService {
        let (sender, receiver) = mpsc::sync_channel(1);

        // kick off logging thread
        thread::spawn(move || loop {
            match receiver.recv().unwrap() {
                LoggingCommands::Message(message) => {
                    log(message, &options)
                        .expect("Logging service failed when receiving message.");
                }
                LoggingCommands::Kill => break,
            }
        });

        LoggingService { sender }

    }

    /// Initiate global logger by boxing the service and sending it to the global logger.
    ///
    /// # Arguments
    ///
    /// * `max_logging_level` - LevelFilter representing the max logging level for the logging service.
    /// Note: The order of logging levels (decreasing) is: Trace, Debug, Info, Warn, Error.
    /// Therefore, specifying Debug as the max logging level will ignore Trace logging messages.
    ///
    pub fn init(self, max_logging_level: LevelFilter) -> Result<(), SetLoggerError> {

        // box logger
        let logger = Box::new(self);

        // set global logger
        log::set_boxed_logger(logger)
            .map(|()| log::set_max_level(max_logging_level))?;

        Ok(())
    }

}

impl Drop for LoggingService {
    fn drop(&mut self) {
        self.sender
            .send(LoggingCommands::Kill)
            .expect("Failed to kill logging service on drop.");
    }
}

impl log::Log for LoggingService {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {

        // convert level to string
        let level = match record.level() {
            Level::Error => String::from(" ERROR "),
            Level::Debug => String::from(" DEBUG "),
            Level::Info => String::from(" INFO  "),
            Level::Trace => String::from(" TRACE "),
            Level::Warn => String::from(" WARN  "),
        };

        self.sender
            .send(LoggingCommands::Message(MessageBody { content: record.args().to_string(), level}))
            .expect("Failed to send message to logging service.");
    }

    fn flush(&self) {
        unimplemented!()
    }
}

// write a message to a log file
// writes the given message to a log file for the current date in the logging directory
// a file will be created in the logging directory specified by the logging config containing the message
// the file will be titled with the current unix date in the format "YYYY_MM_DD.log"
// the message will be preceded with a unix timestamp in the format "[YYYY-MM-DD HH:MM:SS]"
fn log(message_body: MessageBody, options: &LoggingConfig) -> Result<(), Error> {
    // create logging dir if needed
    if !options.logging_directory.exists() {
        create_dir(&options.logging_directory)?;
    } else {
        check_size(options)?;
    }

    // path to file
    let log_file_path = options
        .logging_directory
        .join(format!("{}.log", time_manager::curr_datestamp()));

    // create or open
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)?;

    // write message
    file.write_all((time_manager::curr_timestamp() + message_body.level.as_str() + message_body.content.as_str() + "\n").as_bytes())?;
    file.sync_all()
}

// checks the size of the directory, deleting oldest files if too big
fn check_size(options: &LoggingConfig) -> Result<(), Error> {
    // get sorted Vec of DirEntries
    let files = get_sorted_files_from_dir(options.logging_directory)?;

    // check size of each file
    let mut total_size: usize = 0;
    let mut start_index: usize = 0;
    for i in 0..files.len() {
        // add file size to total
        total_size += files.get(i).unwrap().metadata()?.len() as usize;

        // delete oldest files until size is small enough
        while total_size > options.max_dir_size && start_index <= i {
            total_size -= files.get(start_index).unwrap().metadata()?.len() as usize;
            remove_file(files.get(start_index).unwrap().path())?;
            start_index += 1;
        }
    }

    Ok(())
}

// gets a sorted list (old to new) of logging files from logging dir
fn get_sorted_files_from_dir(logging_directory: &Path) -> Result<Vec<DirEntry>, Error> {
    // files to be sorted
    let mut files: Vec<DirEntry> = Vec::new();

    // get all files in dir
    for file in read_dir(logging_directory)? {
        let file = file?;

        // check file type and name
        if file.file_type()?.is_file() {
            // get file name
            let filename = match file.file_name().into_string() {
                Ok(filename) => filename,
                Err(_) => continue,
            };

            // check filename
            if filename.ends_with(".log") && time_manager::check_date(&filename[0..10]) {
                files.push(file);
            }
        }
    }

    // sort files by date
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_dir_all, File, remove_dir};
    use std::thread;
    use std::time;
    use std::error::Error;
    use std::io::{BufReader, BufRead};
    use log::*;

    #[test]
    fn test_log() -> Result<(), Box<dyn Error>> {
        let logging_directory = Path::new("./test_logs/");
        let logging_service = LoggingService::new(LoggingConfig {
            logging_directory,
            max_dir_size: 10000,
        });
        logging_service.init(log::LevelFilter::Trace)?;
        let current_date = time_manager::curr_datestamp();
        let log_file_name = format!("{}.log", current_date);
        let log_file_path_buf = logging_directory
            .join(log_file_name);
        let log_file_path = log_file_path_buf.as_path();

        // try logging different messages
        // "[YYYY-MM-DD HH:MM:SS] DEBUG test debug"
        debug!("test debug");
        // "[YYYY-MM-DD HH:MM:SS] ERROR test error"
        error!("test error");
        // "[YYYY-MM-DD HH:MM:SS] INFO  test info"
        info!("test info");
        // "[YYYY-MM-DD HH:MM:SS] TRACE test trace"
        trace!("test trace");
        // "[YYYY-MM-DD HH:MM:SS] WARN  test warning"
        warn!("test warning");

        // sleep because logging is done on a different thread (and will take time)
        thread::sleep(time::Duration::from_millis(10));

        // make sure file is there and contents are correct
        assert!(
            log_file_path.exists()
        );
        let log_file = File::open(log_file_path)?;
        let mut lines = BufReader::new(log_file).lines();
        assert!(lines.next().unwrap().unwrap().ends_with("] DEBUG test debug"));
        assert!(lines.next().unwrap().unwrap().ends_with("] ERROR test error"));
        assert!(lines.next().unwrap().unwrap().ends_with("] INFO  test info"));
        assert!(lines.next().unwrap().unwrap().ends_with("] TRACE test trace"));
        assert!(lines.next().unwrap().unwrap().ends_with("] WARN  test warning"));
        assert!(lines.next().is_none());

        // clean up
        remove_dir_all(logging_directory)?;
        assert_eq!(false, logging_directory.exists());
        Ok(())
    }

    #[test]
    fn test_check_size() -> Result<(), std::io::Error> {

        // random files and configs
        let file1 = Path::new("2020_08_01.log");
        let file2 = Path::new("2020_08_02.log");
        let file3 = Path::new("2020_08_03.log");
        let random_text = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let config1 = LoggingConfig {
            logging_directory: Path::new("./test_logs2/"),
            max_dir_size: 0
        };
        let config2 = LoggingConfig {
            logging_directory: Path::new("./test_logs3/"),
            max_dir_size: 1000
        };
        let config3 = LoggingConfig {
            logging_directory: Path::new("./test_logs4/"),
            max_dir_size: 10000
        };

        // config 1: max dir size is 0, all files should be deleted
        create_dir(&config1.logging_directory)?;
        let mut file = File::create(&config1.logging_directory.join(&file1))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config1.logging_directory.join(&file2))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config1.logging_directory.join(&file3))?;
        file.write_all(random_text.as_bytes())?;
        file.sync_all()?;
        check_size(&config1)?;
        remove_dir(&config1.logging_directory)?;

        // config 2: max dir size is 1000 bytes, first file should be deleted (second and third remains)
        create_dir(&config2.logging_directory)?;
        let mut file = File::create(&config2.logging_directory.join(&file1))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config2.logging_directory.join(&file2))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config2.logging_directory.join(&file3))?;
        file.write_all(random_text.as_bytes())?;
        file.sync_all()?;
        check_size(&config2)?;
        assert_eq!(false, config2.logging_directory.join(&file1).exists());
        assert!(config2.logging_directory.join(&file2).exists());
        assert!(config2.logging_directory.join(&file3).exists());
        remove_dir_all(&config2.logging_directory)?;

        // config 3: max dir size is huge, no files should be deleted
        create_dir(&config3.logging_directory)?;
        let mut file = File::create(&config3.logging_directory.join(&file1))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config3.logging_directory.join(&file2))?;
        file.write_all(random_text.as_bytes())?;
        file = File::create(&config3.logging_directory.join(&file3))?;
        file.write_all(random_text.as_bytes())?;
        file.sync_all()?;
        check_size(&config3)?;
        assert!(config3.logging_directory.join(&file1).exists());
        assert!(config3.logging_directory.join(&file2).exists());
        assert!(config3.logging_directory.join(&file3).exists());
        remove_dir_all(&config3.logging_directory)?;

        Ok(())
    }

    #[test]
    fn test_sorted_files() -> std::io::Result<()> {
        let curr_date = format!("{}.log", time_manager::curr_datestamp());
        let files = ["2020_01_12.log", "2020_03_14.log", curr_date.as_str()];
        let logging_directory = Path::new("./test_logs4/");
        create_dir(logging_directory)?;
        for filename in files.iter() {
            File::create(logging_directory.join(filename).as_path())?;
        }
        let sorted = get_sorted_files_from_dir(logging_directory)?;
        for i in 0..files.len() {
            assert_eq!(files[i], sorted[i].file_name().to_str().unwrap());
        }
        remove_dir_all(logging_directory)?;
        assert_eq!(false, logging_directory.exists());
        Ok(())
    }
}
