pub mod population {
    extern crate csv;

    use crate::parser::ParserBuilder;
    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, HashMap};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PopRecord {
        #[serde(rename = "Rank")]
        pub rank: Option<u16>,
        #[serde(rename = "Country")]
        pub country: Option<String>,
        #[serde(rename = "Continent")]
        pub continent: Option<String>,
        #[serde(rename = "Capital")]
        pub capital: Option<String>,
    }

    impl Default for PopRecord {
        fn default() -> Self {
            Self {
                rank: None,
                country: None,
                continent: None,
                capital: None,
            }
        }
    }

    pub trait PopulationSort {
        fn direction(&self, sort_direction: &str) -> BTreeMap<u16, String>;
    }

    impl PopulationSort for HashMap<String, u16> {
        fn direction(&self, _direction: &str) -> BTreeMap<u16, String> {
            todo!()
        }
    }

    impl PopRecord {
        pub fn all_population_data(
            path: &str,
        ) -> Result<Vec<PopRecord>, Box<dyn std::error::Error>> {
            let parser = ParserBuilder::new().reader(path).build();

            let mut records: Vec<PopRecord> = vec![];

            for result in parser
                .reader
                .expect("Failed to unwrap csv reader ...")
                .deserialize()
            {
                let record: PopRecord = result?;

                records.push(record);
            }

            Ok(records)
        }
    }
}
