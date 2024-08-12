// FROM HERE
// https://qxf2.com/blog/reading-writing-csv-data-rust/

use csv::ReaderBuilder;

fn main() {
    println!("read csv!");


    let csv_path:&str = "./data/stock_example.csv"; 
    let reader_result = ReaderBuilder::new()
        .has_headers(is_header_present)
        .from_path(csv_path);
    let reader = match reader_result {
        Ok(reader) => reader,
        Err(err) => return Err(Box::new(err)),
    };
}

// cargo run --example read_csv
