#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn setup_logger(verbose: bool) -> Result<(), fern::InitError> {
    let termite_path = format!("rustwari_{}.log", chrono::Local::now().format("%Y-%m-%d"));
    if verbose {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.level(),
                    record.target(),
                    record.line().unwrap_or(0),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(fern::log_file(termite_path)?)
            .chain(std::io::stdout())
            .apply()?;
        return Ok(());
    }
    Ok(())
}
