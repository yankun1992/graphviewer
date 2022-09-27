use polars::{
    export::rayon::vec,
    prelude::{ndjson::read, *},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CsvOptions {
    has_header: bool,
    ignore_parser_errors: bool,
    delimiter: String,
    comment_char: String,
    quote_char: String,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            has_header: true,
            ignore_parser_errors: true,
            delimiter: ",".to_string(),
            comment_char: "".to_string(),
            quote_char: "".to_string(),
        }
    }
}

pub fn read_csv_schema(file: &str, options: &CsvOptions) -> Schema {
    let mut reader = LazyCsvReader::new(file.into())
        .has_header(options.has_header)
        .with_ignore_parser_errors(options.ignore_parser_errors);

    if options.delimiter != "".to_string() {
        reader = reader.with_delimiter(options.delimiter.trim().as_bytes()[0]);
    }
    if options.comment_char != "".to_string() {
        reader = reader.with_comment_char(Some(options.comment_char.trim().as_bytes()[0]));
    }
    if options.quote_char != "".to_string() {
        reader = reader.with_quote_char(Some(options.quote_char.trim().as_bytes()[0]))
    }

    let df = reader.finish().unwrap();

    df.schema().unwrap().as_ref().clone()
}

pub fn read_csv(file: &str, options: &CsvOptions) -> DataFrame {
    let mut a = CsvReader::from_path(file)
        .unwrap()
        .has_header(options.has_header)
        .with_ignore_parser_errors(options.ignore_parser_errors);
    let df = a.finish().unwrap();

    df
}

pub fn read_csv_with_schema(file: &str, options: &CsvOptions, schema: &Schema) -> DataFrame {
    let mut a = CsvReader::from_path(file)
        .unwrap()
        .has_header(options.has_header)
        .with_ignore_parser_errors(options.ignore_parser_errors)
        .with_schema(&schema);

    let df = a.finish().unwrap();

    df
}
