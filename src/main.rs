use clap::Parser;
use pipesql::repl::App;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgsError {
    #[error("configuration error: {0}")]
    ConfigError(String),
}
#[derive(Parser, Debug)]
#[command(name = "pipesql")]
#[command(version = "0.0.1")]
#[command(about = "THIS IS PIPE_SQL", long_about = None)]
pub struct Args {
    /// debug mod
    #[arg(long, short)]
    pub debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), ArgsError> {
    // let args = Args::parse();

    if let Err(e) = App::new().run() {
        eprintln!("Error: {}", e);
    }
    Ok(())
}
