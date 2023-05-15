mod cli;
mod parser;

use cli::parse_args;
use parser::{read_game_sales, read_wind_power};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    wind_power_production()?;
//    game_sales_figures()?;

    Ok(())
}

fn game_sales_figures() -> Result<(), Box<dyn Error>> {
    let max_records = Some(100); // Control results length for prototyping

    let vg_sales_path: &str = "./input/vgsales.csv";
    //
    if let Err(e) = read_game_sales(vg_sales_path, max_records.unwrap()) {
        eprint!("Error reading csv @ {}", vg_sales_path);
        Err(e)
    } else {
        Ok(())
    }
}

fn wind_power_production() -> Result<(), Box<dyn Error>> {
    let wind_power_path: &str = "./input/wind-power-production-us.csv";
   
    if let Err(e) = read_wind_power(wind_power_path) {
        eprint!("Error reading csv @ {}", wind_power_path);
        Err(e)
    } else {
        Ok(())
    }
}
