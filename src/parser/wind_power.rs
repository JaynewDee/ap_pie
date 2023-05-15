pub mod us_production {
    extern crate csv;

    use serde::Deserialize;
    use std::collections::{BTreeMap, HashMap};

    #[derive(Debug, Deserialize)]
    pub struct SimpleDate {
        month: u16,
        year: u16,
        order: u16,
    }

    #[derive(Debug, Deserialize)]
    pub struct WindRecord {
        // Struct fields are private by default
        pub date: String,
        pub wind_texas: u16,
    }

    impl Default for WindRecord {
        fn default() -> Self {
            Self {
                date: String::new(),
                wind_texas: 0,
            }
        }
    }

    fn from_trichar(val: &str) -> u16 {
        match val {
            "Jan" => 1,
            "Feb" => 2,
            "Mar" => 3,
            "Apr" => 4,
            "May" => 5,
            "Jun" => 6,
            "Jul" => 7,
            "Aug" => 8,
            "Sep" => 9,
            "Oct" => 10,
            "Nov" => 11,
            "Dec" => 12,
            _ => 0
        }
    }

    impl From<String> for SimpleDate {
        fn from(val: String) -> Self {
            let vals = val.split(" ").collect::<Vec<&str>>();

            let year = vals[1].parse::<u16>().unwrap();
            let month = from_trichar(vals[0]);

            println!("Year ::: {}\nMonth :: {}", year, month);

            Self {
                month,
                year,
                order: (month + year),
            }
        }
    } 
}
