#![allow(unused_variables, dead_code)]

pub mod game_records {
    extern crate csv;

    use serde::Deserialize;
    use std::collections::{BTreeMap, HashMap};

    #[derive(Debug, Deserialize)]
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
            let mut pairs_vec: Vec<(String, u16)> = Vec::new();

            let _ = self
                .into_iter()
                .map(|(k, v)| pairs_vec.push((k.to_owned(), *v)));

            // Mutate in place
            pairs_vec.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
            pairs_vec.reverse();

            let mut as_map: BTreeMap<u16, String> = BTreeMap::new();

            let _ = pairs_vec
                .drain(..)
                .map(|(k, v)| as_map.insert(v, k.to_string()));

            println!("{:#?}", as_map);
            as_map
        }
    }
}
