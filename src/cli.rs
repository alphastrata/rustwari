//! Main controls for the CLI.

use clap::Parser; //, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enjoy a silent app with nothing more than a progressbar.
    #[arg(short, long, default_value_t = true)]
    pub quiet: bool,

    /// Not reccomended unless developing.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// <WIP>Override the `completed` dir in your existing config file.
    #[arg(long)]
    pub completed_dir: Option<String>,

    /// <WIP>If you have a specific path to the config.yml this app uses to provide specific paths to
    /// where you want temporary, and, completed Images stored.
    #[arg(short, long)]
    pub config_file: Option<String>,

    /// Resize the image after processing, default is false.
    #[arg(short, long, default_value_t = false)]
    pub resize: bool,

    /// Open the image after completing it's retrival.
    #[arg(long, default_value_t = false)]
    pub open: bool,

    /// Get the entry for YYYY-MM-DD HH:MM, note you'll need to wrap it all in ' or "s
    #[arg(long)]
    pub oneshot: Option<String>,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
