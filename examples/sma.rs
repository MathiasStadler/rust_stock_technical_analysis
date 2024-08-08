// FROM HERE
// https://github.com/greyblake/ta-rs/blob/master/examples/ema.rs

use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;

fn main() {
    let mut ema = Sma::new(9).unwrap();
    let mut reader = csv::Reader::from_path("./data/stock_example.csv").unwrap();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();
        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        let sma_val = sma.next(&dt);
        println!("{}: {} = {:2.2}", date, sma, sma_val);
    }
}

// cargo run --example ema