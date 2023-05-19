#![allow(unused_variables, dead_code)]

pub mod game_records {
    extern crate csv;

    use crate::parser::ParserBuilder;
    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, HashMap};

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct GameRecord {
        #[serde(rename = "Rank")]
        pub rank: Option<u16>,
        #[serde(rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "Platform")]
        pub platform: Option<String>,
        #[serde(rename = "Year", default, deserialize_with = "csv::invalid_option")]
        pub year: Option<u16>,
        #[serde(rename = "Genre")]
        pub genre: Option<String>,
        #[serde(rename = "Publisher")]
        pub publisher: Option<String>,
        #[serde(rename = "NA_Sales")]
        na_sales: Option<f32>,
        #[serde(rename = "EU_Sales")]
        eu_sales: Option<f32>,
        #[serde(rename = "JP_Sales")]
        jp_sales: Option<f32>,
        #[serde(rename = "Other_Sales")]
        other_sales: Option<f32>,
    }

    pub trait GameSort {
        fn pub_count(&self, sort_direction: &str) -> BTreeMap<u16, String>;
    }

    impl GameSort for HashMap<String, u16> {
        fn pub_count(&self, direction: &str) -> BTreeMap<u16, String> {
            let mut pairs_vec: Vec<(&String, &u16)> = self.iter().fold(vec![], |mut acc, v| {
                acc.push(v);
                acc
            });
            // Mutate in place
            pairs_vec.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
            pairs_vec.reverse();

            let as_map: BTreeMap<u16, String> =
                pairs_vec.iter().fold(BTreeMap::new(), |mut acc, (k, v)| {
                    acc.insert(**v, k.to_string());
                    acc
                });

            println!("{:#?}", as_map);
            as_map
        }
    }

    impl GameRecord {
        pub fn all_game_data(path: &str) -> Result<Vec<GameRecord>, Box<dyn std::error::Error>> {
            let parser = ParserBuilder::new().reader(path).build();

            let mut records: Vec<GameRecord> = vec![];

            for result in parser
                .reader
                .expect("Failed to unwrap csv reader ...")
                .deserialize()
            {
                let record: GameRecord = result?;

                records.push(record);
            }

            Ok(records)
        }

        fn read_game_sales(
            path: &str,
            total_recs: u16,
        ) -> Result<BTreeMap<u16, String>, Box<dyn std::error::Error>> {
            let parser = ParserBuilder::new().reader(path).build();

            let mut publisher_map: HashMap<String, u16> = HashMap::new();

            for result in parser
                .reader
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

        pub fn game_sales_figures() -> Result<(), Box<dyn std::error::Error>> {
            let max_records = 100; // Control results length for prototyping

            let vg_sales_path: &str = "./input/vgsales.csv";
            //
            if let Err(e) = Self::read_game_sales(vg_sales_path, max_records) {
                eprint!("Error reading csv @ {}", vg_sales_path);
                Err(e)
            } else {
                Ok(())
            }
        }

        pub fn filter_by(param: &str, value: String, records: Vec<GameRecord>) -> Vec<GameRecord> {
            match param {
                "year" => records
                    .into_iter()
                    .filter(|rec| {
                        rec.year != None && rec.year.unwrap() == value.parse::<u16>().unwrap()
                    })
                    .collect(),
                "publisher" => records
                    .into_iter()
                    .filter(|rec| rec.publisher != None && rec.publisher.clone().unwrap() == value)
                    .collect(),
                "platform" => records
                    .into_iter()
                    .filter(|rec| rec.platform != None && rec.platform.clone().unwrap() == value)
                    .collect(),
                "genre" => records
                    .into_iter()
                    .filter(|rec| rec.genre != None && rec.genre.clone().unwrap() == value)
                    .collect(),
                _ => records,
            }
        }
    }
}
