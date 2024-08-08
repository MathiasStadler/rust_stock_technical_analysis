// FROM HERE
// https://docs.rs/ta/latest/ta/indicators/struct.MovingAverageConvergenceDivergence.html

use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use ta::Next;


fn main(){
let mut macd = Macd::new(3, 6, 4).unwrap();

assert_eq!(round(macd.next(2.0).into()), (0.0, 0.0, 0.0));
assert_eq!(round(macd.next(3.0).into()), (0.21, 0.09, 0.13));
assert_eq!(round(macd.next(4.2).into()), (0.52, 0.26, 0.26));
assert_eq!(round(macd.next(7.0).into()), (1.15, 0.62, 0.54));
assert_eq!(round(macd.next(6.7).into()), (1.15, 0.83, 0.32));
assert_eq!(round(macd.next(6.5).into()), (0.94, 0.87, 0.07));

fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
    let n0 = (nums.0 * 100.0).round() / 100.0;
    let n1 = (nums.1 * 100.0).round() / 100.0;
    let n2 = (nums.2 * 100.0).round() / 100.0;
    (n0, n1, n2)
}

}