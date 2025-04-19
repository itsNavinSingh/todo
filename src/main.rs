use clap::Parser;
use commandimpl::{init::init, list::list};

mod arguments;
mod commandimpl;
mod tasks;
mod utility;
fn main() -> Result<(), anyhow::Error>{
    let args = arguments::Cli::parse();
    match args.command {
        arguments::Commands::Init => init()?,
        arguments::Commands::List => list()?,
        _ => println!("Command is not implemented yet"),
    }
    Ok(())
}
