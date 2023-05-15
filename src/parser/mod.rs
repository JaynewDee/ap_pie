#![allow(unused_variables, dead_code)]

mod games;

pub mod parser {
    use std::collections::HashMap;
    use std::fs::File;

    use super::games::game_records::{GameRecord, GameSort};

    type CsvReader = Option<csv::Reader<File>>;
    type CsvWriter = Option<csv::Writer<File>>;

    struct Parser {
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
                    csv_reader: None::<csv::Reader<File>>,
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

    pub fn read_game_sales(path: &str, total_recs: u16) -> Result<(), Box<dyn std::error::Error>> {
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

        publisher_map.pub_count("desc");

        Ok(())
    }
}
