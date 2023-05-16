#![allow(unused_variables, dead_code)]

pub mod game_records {
    extern crate csv;

    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, HashMap};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GameRecord {
        #[serde(rename = "Rank")]
        pub rank: Option<u16>,
        #[serde(rename = "Name")]
        name: Option<String>,
        #[serde(rename = "Platform")]
        platform: Option<String>,
        #[serde(rename = "Year", default, deserialize_with = "csv::invalid_option")]
        year: Option<u16>,
        #[serde(rename = "Genre")]
        genre: Option<String>,
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
}
