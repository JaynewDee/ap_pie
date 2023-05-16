use std::env::{args, Args, ArgsOs, args_os};
use std::error::Error;


pub fn parse_os_args() -> Result<ArgsOs, Box<dyn Error>> {
    let all_os_args = args_os();

    Ok(all_os_args)
}

pub fn parse_args() -> Result<Args, Box< dyn Error>> {
    let all_args = args();
    
    Ok(all_args)
}
