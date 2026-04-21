use std::{
    io::Write,
    sync::{OnceLock, Mutex},
};

use core::str::FromStr;

use termcolor::{WriteColor, StandardStream, ColorChoice};

pub use termcolor::{ColorSpec, Color};

use ahash::AHashMap;

use leimu_mem::slot_map::*;

use crate::*;

use fmt::SegmentSpec;

pub type CustomFmt = SlotIndex<LogFmt>;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Always = -1,
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

pub enum LevelFmt {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Other(CustomFmt, Level),
}

impl FromStr for Level {

    type Err = ();

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("error") {
            Ok(Self::Error)
        } else if s.eq_ignore_ascii_case("warn") {
            Ok(Self::Warn)
        } else if s.eq_ignore_ascii_case("info") {
            Ok(Self::Info)
        } else if s.eq_ignore_ascii_case("debug") {
            Ok(Self::Debug)
        } else if s.eq_ignore_ascii_case("trace") {
            Ok(Self::Trace)
        } else {
            Err(())
        }
    }
}

struct Logger {
    stderr: StandardStream,
    info_fmt: LogFmt,
    warn_fmt: LogFmt,
    error_fmt: LogFmt,
    debug_fmt: LogFmt,
    trace_fmt: LogFmt,
    custom_fmt: SlotMap<LogFmt>,
    target_levels: AHashMap<String, Level>,
    base_level: Level,
}

impl Logger {

    #[inline(always)]
    fn new() -> Self {
        let mut target_levels = AHashMap::default();
        let mut base_level = Level::Error;
        if let Ok(env) = std::env::var("RUST_LOG") {
            let parse_arg: for<'a> fn(&'a str) -> (Option<&'a str>, &'a str) =
                |arg: &str| -> (Option<&str>, &str)
            {
                let mut module = None;
                let mut level = arg.trim();
                if let Some(j) = arg.find("=") {
                    module = Some(arg[0..j].trim());
                    level = arg[j+1..].trim();
                }
                (module, level)
            };
            let mut process_arg = |module: Option<&str>, level: &str| {
                if let Ok(level) = Level::from_str(level) {
                    if let Some(module) = module {
                        let entry = target_levels
                            .entry(module.to_string())
                            .or_insert(level);
                        *entry = (*entry).min(level);
                    } else {
                        base_level = level;
                    }
                }
            };
            let mut substr = &env[..];
            let mut offset = 0;
            while let Some(i) = substr.find(",")  {
                let arg = &env[offset..offset+i];
                let (module, level) = parse_arg(arg);
                process_arg(module, level);
                offset += i + 1;
                substr = &env[offset..];
            }
            let arg = &env[offset..];
            let (module, level) = parse_arg(arg);
            process_arg(module, level);
        }
        Self {
            stderr: StandardStream::stderr(ColorChoice::Auto),
            info_fmt: LogFmt::default(),
            warn_fmt: LogFmt::default(),
            error_fmt: LogFmt::default(),
            debug_fmt: LogFmt::default(),
            trace_fmt: LogFmt::default(),
            custom_fmt: SlotMap::default(),
            target_levels,
            base_level,
        }
    }

    fn target_level(&self, target: &str) -> Level {
        let mut substr = target;
        if let Some(&level) = self.target_levels.get(substr) {
            return level
        }
        while let Some(i) = substr.rfind("::") { 
            substr = &substr[0..i];
            if let Some(&level) = self.target_levels.get(substr) {
                return level
            }
        }
        self.base_level
    }

    #[inline]
    fn log(&mut self, target: &str, level: LevelFmt, msg: core::fmt::Arguments) -> Result<bool> {
        let target_level = self.target_level(target);
        let fmt = match level {
            LevelFmt::Error => {
                if target_level < Level::Error {
                    return Ok(false)
                }
                &self.error_fmt
            },
            LevelFmt::Warn => {
                if target_level < Level::Warn {
                    return Ok(false)
                }
                &self.warn_fmt
            },
            LevelFmt::Info => {
                if target_level < Level::Info {
                    return Ok(false)
                }
                &self.info_fmt
            },
            LevelFmt::Debug => {
                if target_level < Level::Debug {
                    return Ok(false)
                }
                &self.debug_fmt
            },
            LevelFmt::Trace => {
                if target_level < Level::Trace {
                    return Ok(false)
                }
                &self.trace_fmt
            },
            LevelFmt::Other(fmt, level) => {
                if target_level < level {
                    return Ok(false)
                }
                self.custom_fmt.get(fmt)?
            },
        };
        for segment in fmt {
            match segment {
                SegmentSpec::Message(log_spec) => {
                    if let Some(color_spec) = &log_spec.color_spec {
                        self.stderr.set_color(color_spec)?;
                        write!(self.stderr, "{}", msg)?;
                        self.stderr.reset()?;
                    } else {
                        write!(self.stderr, "{}", msg)?;
                    }
                },
                SegmentSpec::Text(text, log_spec) => {
                    if let Some(color_spec) = &log_spec.color_spec {
                        self.stderr.set_color(color_spec)?;
                        self.stderr.write_all(text.as_bytes())?;
                        self.stderr.reset()?;
                    } else {
                        self.stderr.write_all(text.as_bytes())?;
                    }
                },
            }
        }
        self.stderr.write_all(b"\n")?;
        Ok(true)
    }
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn init() {
    if LOGGER.get().is_some() { return }
    LOGGER
        .set(Mutex::new(Logger::new()))
        .unwrap_or_else(|_| panic!("leimu logger initialized twice"));
}

#[inline(always)]
pub fn error_fmt(mut f: impl FnMut(&mut LogFmtBuilder)) {
    let mut logger = LOGGER.get().expect("leimu logger not initialized").lock().unwrap();
    let mut builder = LogFmtBuilder::new(&mut logger.error_fmt);
    f(&mut builder);
}

#[inline(always)]
pub fn warn_fmt(mut f: impl FnMut(&mut LogFmtBuilder)) {
    let mut logger = LOGGER.get().expect("leimu logger not initialized").lock().unwrap();
    let mut builder = LogFmtBuilder::new(&mut logger.warn_fmt);
    f(&mut builder);
}

#[inline(always)]
pub fn info_fmt(mut f: impl FnMut(&mut LogFmtBuilder)) {
    let mut logger = LOGGER.get().expect("leimu logger not initialized").lock().unwrap();
    let mut builder = LogFmtBuilder::new(&mut logger.info_fmt);
    f(&mut builder);
}

#[inline(always)]
pub fn debug_fmt(mut f: impl FnMut(&mut LogFmtBuilder)) {
    let mut logger = LOGGER.get().expect("leimu logger not initialized").lock().unwrap();
    let mut builder = LogFmtBuilder::new(&mut logger.debug_fmt);
    f(&mut builder);
}

#[inline(always)]
pub fn trace_fmt(mut f: impl FnMut(&mut LogFmtBuilder)) {
    let mut logger = LOGGER.get().expect("leimu logger not initialized").lock().unwrap();
    let mut builder = LogFmtBuilder::new(&mut logger.trace_fmt);
    f(&mut builder);
}

#[inline]
pub fn custom_fmt(fmt: LogFmt) -> CustomFmt {
    LOGGER
        .get()
        .expect("leimu logger not initialized")
        .lock()
        .unwrap()
        .custom_fmt.insert(fmt)
}

#[inline]
pub fn log(target: &str, level: LevelFmt, args: core::fmt::Arguments) -> Result<bool> {
    LOGGER
        .get()
        .expect("leimu logger not initialized")
        .lock()
        .unwrap()
        .log(target, level, args)
}

#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::log(module_path!(), $crate::LevelFmt::Error, format_args!($fmt, $($arg),*))
            .unwrap_or(false)
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::log(module_path!(), $crate::LevelFmt::Warn, format_args!($fmt, $($arg),*))
            .unwrap_or(false)
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::log(module_path!(), $crate::LevelFmt::Info, format_args!($fmt, $($arg),*))
            .unwrap_or(false)
    };
} 

#[macro_export]
macro_rules! debug {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::log(module_path!(), $crate::LevelFmt::Debug, format_args!($fmt, $($arg),*))
            .unwrap_or(false);
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::log(module_path!(), $crate::LevelFmt::Trace, format_args!($fmt, $($arg),*))
            .unwrap_or(false)
    };
}
