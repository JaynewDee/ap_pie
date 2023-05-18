mod cli;
mod parser;
mod server;

use server::connection::{launch, Server};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let args = parse_args()?;
    // println!("{:#?}", args);

    launch(Server::new())?;

    Ok(())
}
