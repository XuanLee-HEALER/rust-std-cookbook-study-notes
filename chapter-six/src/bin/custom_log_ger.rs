use std::{
    error,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
    result,
    sync::RwLock,
    time::{SystemTime, UNIX_EPOCH},
};

use log::{Level, debug, error, info, trace, warn};

struct FileLogger {
    level: Level,
    writer: RwLock<BufWriter<File>>,
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // logger是否对指定的log level开启
        // 这里也可以添加自定义的基于target和regex的过滤器
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self
                .writer
                .write()
                .expect("Failed to unlock log file writer in write mode");
            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).expect(
                "Failed to generate timestamp: This system is operating before the unix epoch",
            );
            write!(
                writer,
                "{} {} at {}: {}\n",
                record.level(),
                timestamp.as_secs(),
                record.target(),
                record.args()
            )
            .expect("Failed to log the file");
        }
        self.flush();
    }

    fn flush(&self) {
        self.writer
            .write()
            .expect("Failed to unlock log file writer in write mode")
            .flush()
            .expect("Failed to flush log file writer")
    }
}

impl FileLogger {
    fn init(level: Level, file_name: &str) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
        let writer = RwLock::new(BufWriter::new(file));
        let logger = FileLogger { level, writer };
        log::set_max_level(level.to_level_filter());
        // 将此logger设置为宏使用的logger
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

#[derive(Debug)]
enum FileLoggerError {
    Io(io::Error),
    SetLogger(log::SetLoggerError),
}

type Result<T> = result::Result<T, FileLoggerError>;

impl error::Error for FileLoggerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Io(ref err) => Some(err),
            Self::SetLogger(ref err) => Some(err),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

impl Display for FileLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Io(ref err) => write!(f, "IO error: {}", err),
            Self::SetLogger(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<io::Error> for FileLoggerError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<log::SetLoggerError> for FileLoggerError {
    fn from(value: log::SetLoggerError) -> Self {
        Self::SetLogger(value)
    }
}

/// 根据 The Twelve-Factor App指引，你不应该直接将日志写到磁盘上。日志应该为看作事件流，原始格式会导向 stdout。生产环境将所有的日志流路由到它们的最终目的地（通过systemd 或者专门的日志路由器，例如Logplex或Fluentd）
///
/// 每个logger需要实现 log::Log 特质，包含 enabled，log和flush方法。enabled返回那些被接受的日志事件，这里可以指定任何你想实现的过滤效果，这个方法不会直接（自动）被log调用，而是作为辅助方法在log方法中调用。flush也是一样的，它会应用日志缓冲区的修改，但是不会直接被log调用
///
/// 对于call方法，它几乎总是以 `if self.enabled(record.metadata()) {` 开头，以 `self.flush()` 结尾
///
/// 如果你想要更好（一点）的性能，可以使用 log::set_logger()，它接受一个 static值
///
/// 本例中 FileLogger对任何target都一样，更复杂的logger（例如env_logger）可以为不同的target设置不同的等级。这就是LevelFilter枚举的作用，它有一个 off 状态绑定了 no logging enabled for this target。可以通过查看env_logger的源码了解如何实现一个基于target的过滤器
fn main() {
    FileLogger::init(Level::Info, "log.txt").expect("Failed to init FileLogger");
    trace!("Beginning the operation");
    info!("A lightning strikes a body");
    warn!("It's moving");
    error!("It's alive!");
    debug!("Dr. Frankenstein now knows how it feels to be god");
    trace!("End of the operation");
}
