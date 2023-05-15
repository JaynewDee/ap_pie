

mod games;
use games::GameRecord;



pub mod parser {
    use std::collections::{BTreeMap, HashMap};
    use std::io::File;

    struct Paths<'a> {
        read: Option<&'a str>,
        write: Option<&'a str>,
    }

    struct Parser<'a> {
        paths: Paths<'a>,
        csv_reader: csv::Reader<File>,
        csv_writer: Option<csv::Writer<File>>,
    }

    pub struct ParserBuilder<'a> {
        parser: Parser<'a>,
    }

    impl ParserBuilder<'_> {
        fn new() -> Self {
            Self {
                parser: Parser {
                    paths: Paths {
                        read: None,
                        write: None,
                    },
                    csv_reader: None::<csv::Reader<File>>,
                    csv_writer: None,
                },
            }
        }

        fn read_path(mut self, path: &str) -> Self {
            self.parser.paths.read = Some(path);
            self
        }

        fn write_path(mut self, path: &str) -> Self {
            self.parser.paths.write = Some(path);
            self
        }

        fn csv_reader(mut self) -> Self {
           
                
            
            self.parser.csv_reader = csv::Reader::from_reader(File::open(self.parser.paths.read).unwrap());
            self
        }

        fn csv_writer(mut self) -> Self {
            self.parser.csv_writer = Some("");
            self
        }

        fn build(self) -> Parser<'static> {
            self.parser
        }
    }

    impl Default for Parser<'_> {
        fn default() -> Self {
            Self {
                paths: Paths {
                    read: Some("./input"),
                    write: Some("./output"),
                },
                csv_reader: None::<csv::Reader<File>>, 
                csv_writer: None,
            }
        }
    }

    pub fn read_game_sales(path: &str, total_recs: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mut parser = ParserBuilder::new().read_path(path).csv_reader().build();

        let mut publisher_map: HashMap<String, u16> = HashMap::new();

        for result in parser.csv_reader.deserialize() {
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
