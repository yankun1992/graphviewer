use std::fs::File;

use polars::prelude::*;

use super::Rename;

pub fn read_parquet_schema(file: &str) -> Schema {
    let ldf = LazyFrame::scan_parquet(file.into(), Default::default()).unwrap();
    ldf.schema().unwrap().as_ref().clone()
}

pub fn read_parquet(file: &str, n_row: Option<usize>) -> DataFrame {
    let arg = match n_row {
        Some(n) => {
            let mut arg: ScanArgsParquet = Default::default();
            arg.n_rows = n_row;
            arg
        }
        None => ScanArgsParquet::default(),
    };

    let ldf = LazyFrame::scan_parquet(file.into(), arg).unwrap();
    ldf.collect().unwrap()
}

pub fn read_parquet_with_rename(
    file: &str,
    n_row: Option<usize>,
    renames: &Vec<Rename>,
) -> DataFrame {
    let arg = match n_row {
        Some(n) => {
            let mut arg: ScanArgsParquet = Default::default();
            arg.n_rows = n_row;
            arg
        }
        None => ScanArgsParquet::default(),
    };

    let mut df = ParquetReader::new(File::open(file).unwrap())
        .read_parallel(ParallelStrategy::RowGroups)
        .with_n_rows(n_row)
        .set_rechunk(true)
        .finish()
        .unwrap();

    for rename in renames {
        if let Some(ref name) = rename.rename {
            if name != &"".to_string() {
                df.rename(&rename.name, &name).unwrap();
            };
        };
    }

    df
}

pub fn read_parquet_lazy(file: &str, n_row: Option<usize>) -> LazyFrame {
    let arg = match n_row {
        Some(n) => {
            let mut arg: ScanArgsParquet = Default::default();
            arg.n_rows = n_row;
            arg
        }
        None => ScanArgsParquet::default(),
    };
    LazyFrame::scan_parquet(file.into(), arg).unwrap()
}

pub fn write_parquet(file: &str, df: &mut DataFrame) {
    let file = File::create(file).unwrap();
    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .with_statistics(true)
        .finish(df)
        .unwrap();
}

pub fn write_lazy_parquet(file: &str, ldf: LazyFrame) {
    let file = File::create(file).unwrap();
    let mut df = ldf.collect().unwrap();
    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .with_statistics(true)
        .finish(&mut df)
        .unwrap();
}
