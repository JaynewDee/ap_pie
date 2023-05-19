pub mod wind_production {
    extern crate csv;

    use crate::parser::builder::ParserBuilder;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct SimpleDate {
        month: u16,
        year: u16,
        order: u16,
    }

    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct WindRecord {
        pub date: String,
        pub wind_california: u16,
        pub wind_texas: u16,
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
            _ => 0,
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

    impl WindRecord {
        pub fn read_wind_power(path: &str) -> Result<Vec<WindRecord>, Box<dyn std::error::Error>> {
            let parser = ParserBuilder::new().reader(path).build();

            let mut records: Vec<WindRecord> = vec![];

            for result in parser
                .reader
                .expect("Failed to unwrap csv reader ...")
                .deserialize()
            {
                let record: WindRecord = result?;
                records.push(record);
            }

            Ok(records)
        }

        pub fn wind_power_production(path: &str) -> Result<(), Box<dyn std::error::Error>> {
            if let Err(e) = Self::read_wind_power(path) {
                eprint!("Error reading csv @ {}", e);
                Err(e)
            } else {
                Ok(())
            }
        }
    }
}
