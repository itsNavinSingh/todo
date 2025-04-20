use clap::Parser;
use commandimpl::{add::add, init::init, list::list};

mod arguments;
mod commandimpl;
mod tasks;
mod utility;
fn main() -> Result<(), anyhow::Error>{
    let args = arguments::Cli::parse();
    match args.command {
        arguments::Commands::Init => init()?,
        arguments::Commands::List => list()?,
        arguments::Commands::Add(add_cmd) => add(&add_cmd)?,
        _ => println!("Command is not implemented yet"),
    }
    Ok(())
}
