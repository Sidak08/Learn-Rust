use time::Duration;
use time::PrimitiveDateTime as DateTime;
use time::macros::{date, time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let result = start + Duration::seconds(1_000_000_000);
    //println!("{:?}", result);
    return result;
}

fn main() {
    let start = DateTime::new(date!(2026 - 6 - 1), time!(0:00:00));
    after(start);
}
