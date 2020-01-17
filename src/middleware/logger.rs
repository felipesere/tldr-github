use std::fs::File;

use simplelog::*;

pub fn terminal() -> Box<dyn SharedLogger> {
    TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap()
}

pub fn file(name: &'static str) -> Box<dyn SharedLogger> {
    WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(name).unwrap(),
    )
}
