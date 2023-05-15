mod parser;

use parser::parser::read_game_sales;

fn main() -> Result<(), ()> {
    let max_records = Some(10_000); // Control results length for prototyping
    let vg_sales_path: &str = "./vgsales.csv";

    if let Err(_) = read_game_sales(vg_sales_path, max_records.unwrap()) {
        eprint!("Error reading csv @ {}", vg_sales_path);
        Err(())
    } else {
        Ok(())
    }
}
