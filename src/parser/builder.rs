use std::fs::File;

type CsvReader = Option<csv::Reader<File>>;
type CsvWriter = Option<csv::Writer<File>>;

pub struct Parser {
    pub reader: CsvReader,
    pub writer: CsvWriter,
}

pub struct ParserBuilder {
    parser: Parser,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self {
            parser: Parser {
                reader: None,
                writer: None,
            },
        }
    }

    pub fn reader(mut self, path: &str) -> Self {
        self.parser.reader = Some(csv::Reader::from_reader(File::open(path).unwrap()));
        self
    }

    fn _writer(mut self, path: &str) -> Self {
        self.parser.writer = Some(csv::Writer::from_path(path).unwrap());
        self
    }

    pub fn build(self) -> Parser {
        self.parser
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            reader: None::<csv::Reader<File>>,
            writer: None,
        }
    }
}
