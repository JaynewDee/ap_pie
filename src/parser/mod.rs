#![allow(unused_variables, dead_code)]

mod games;
mod wind_power;

use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::error::Error;
pub use crate::parser::games::game_records::{GameRecord, GameSort};
use crate::parser::wind_power::us_production::{WindRecord, SimpleDate};

type CsvReader = Option<csv::Reader<File>>;
type CsvWriter = Option<csv::Writer<File>>;

pub struct Parser {
    csv_reader: CsvReader,
    csv_writer: CsvWriter,
}

pub struct ParserBuilder {
    parser: Parser,
}

impl ParserBuilder {
    fn new() -> Self {
        Self {
            parser: Parser {
                csv_reader: None,
                csv_writer: None,
            },
        }
    }

    fn csv_reader(mut self, path: &str) -> Self {
        self.parser.csv_reader = Some(csv::Reader::from_reader(File::open(path).unwrap()));
        self
    }

    fn csv_writer(mut self, path: &str) -> Self {
        self.parser.csv_writer = Some(csv::Writer::from_path(path).unwrap());
        self
    }

    fn build(self) -> Parser {
        self.parser
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            csv_reader: None::<csv::Reader<File>>,
            csv_writer: None,
        }
    }
}

pub fn all_game_data(path: &str) -> Result<Vec<GameRecord>, Box<dyn std::error::Error>> {
    let parser = ParserBuilder::new().csv_reader(path).build();

    let mut records: Vec<GameRecord> = vec![];

    for result in parser.csv_reader.expect("Failed to unwrap csv reader ...").deserialize() {
        let record: GameRecord = result?;

        records.push(record);
    };

    Ok(records)
}

pub fn read_game_sales(path: &str, total_recs: u16) -> Result<BTreeMap<u16, String>, Box<dyn std::error::Error>> {
    let parser = ParserBuilder::new().csv_reader(path).build();

    let mut publisher_map: HashMap<String, u16> = HashMap::new();

    for result in parser
        .csv_reader
        .expect("Failed to unwrap csv reader ...")
        .deserialize()
    {
        let record: GameRecord = result?;

        if let Some(res) = record.publisher {
            publisher_map
                .entry(res)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if let Some(res) = record.rank {
            match res {
                x if x == total_recs => break,
                _ => continue,
            }
        }
    }

    let publish_sorted = publisher_map.pub_count("desc");

    Ok(publish_sorted)
}

fn read_wind_power(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser = ParserBuilder::new().csv_reader(path).build();

    for result in parser
        .csv_reader
        .expect("Failed to unwrap csv reader ...")
        .deserialize()
    {
        let record: WindRecord = result?;

        println!("{:#?}", record);
        let simple_date = SimpleDate::from(record.date);
    }

    Ok(())
}

pub fn game_sales_figures() -> Result<(), Box<dyn Error>> {
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

pub fn wind_power_production() -> Result<(), Box<dyn Error>> {
    let wind_power_path: &str = "./input/wind-power-production-us.csv";
   
    if let Err(e) = read_wind_power(wind_power_path) {
        eprint!("Error reading csv @ {}", wind_power_path);
        Err(e)
    } else {
        Ok(())
    }
}

