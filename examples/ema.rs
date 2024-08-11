// FROM HERE
// https://github.com/greyblake/ta-rs/blob/master/examples/ema.rs

use ta::indicators::ExponentialMovingAverage as Ema;
use ta::DataItem;
use ta::Next;

fn main() {
    let mut ema = Ema::new(20).unwrap();
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
        let ema_val = ema.next(&dt);
        println!("{}: {} = {:2.2}", date, ema, ema_val);
    }
}

// cargo run --example ema