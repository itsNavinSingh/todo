use clap::Parser;
use commandimpl::{add::add, clear::clear, delete::delete, edit::edit, init::init, list::list, reset::reset, search::search};

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
        arguments::Commands::Delete(del_cmd) => delete(&del_cmd)?,
        arguments::Commands::Clear => clear()?,
        arguments::Commands::Edit(edit_cmd) => edit(&edit_cmd)?,
        arguments::Commands::Reset => reset()?,
        arguments::Commands::Search(search_cmd) => search(&search_cmd)?,
        _ => println!("Command is not implemented yet"),
    }
    Ok(())
}
