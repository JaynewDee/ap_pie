#![allow(unused_variables, dead_code)]
//
extern crate csv;
use std::collections::{HashMap, BTreeMap};
use std::error::Error;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameRecord {
    #[serde(rename = "Rank")]
    rank: Option<u16>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Platform")]
    platform: Option<String>,
    #[serde(rename = "Year", default, deserialize_with = "csv::invalid_option")]
    year: Option<u16>,
    #[serde(rename = "Genre")]
    genre: Option<String>,
    #[serde(rename = "Publisher")]
    publisher: Option<String>,
    #[serde(rename = "NA_Sales")]
    na_sales: Option<f32>,
    #[serde(rename = "EU_Sales")]
    eu_sales: Option<f32>,
    #[serde(rename = "JP_Sales")]
    jp_sales: Option<f32>,
    #[serde(rename = "Other_Sales")]
    other_sales: Option<f32>,
}

trait GameSort {
    fn pub_count(&self, sort_direction: &str) -> BTreeMap<u16, String>;
}

impl GameSort for HashMap<String, u16> {
    fn pub_count(&self, direction: &str) -> BTreeMap<u16, String> {
        let mut pairs_vec: Vec<(String, u16)> = Vec::new();

        for (k, v) in self.into_iter() {
            let tuple = (k.to_owned(), *v);
            pairs_vec.push(tuple);

            println!("{}: {}", &k, &v);
        }

 
        pairs_vec.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        pairs_vec.reverse(); 

        let mut as_map: BTreeMap<u16, String> = BTreeMap::new();

        println!("{:#?}", &pairs_vec);

        for (k, v) in pairs_vec.drain(..) {
            as_map.insert(v, k.to_string());
        }

        println!("{:#?}", as_map);
        as_map
    }
}

impl Default for GameRecord {
    fn default() -> Self {
        Self {
            rank: None,
            name: None,
            platform: None,
            year: None,
            genre: None,
            publisher: None,
            na_sales: None,
            eu_sales: None,
            jp_sales: None,
            other_sales: None,
        }
    }
}

fn main() {
    let max_records = 1000;
    let vg_sales_path: &str = "./vgsales.csv";
    let _ = read_sales_csv(vg_sales_path, max_records);
}

fn read_sales_csv(path: &str, total_recs: u16) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut publisher_map: HashMap<String, u16> = HashMap::new();

    for result in reader.deserialize() {
        let record: GameRecord = result?;

        // println!("{:#?}", record);

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

    publisher_map.pub_count("desc");
    Ok(())
}
