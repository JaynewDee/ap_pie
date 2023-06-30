mod parser;
mod server;

use server::connection;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let server_config = connection::Config::new();

    connection::launch(server_config)?;

    Ok(())
}
