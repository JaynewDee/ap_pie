#![allow(unused_imports)]


mod cli;
mod parser;
mod server;

use cli::parse_args;
use parser::{game_sales_figures, wind_power_production};
use server::connection::{launch, Server};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let args = parse_args()?;
    // println!("{:#?}", args);
    // wind_power_production()?;
    // game_sales_figures()?;

    launch(Server::new())?;

    Ok(())
}
