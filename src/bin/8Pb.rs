pub fn is_leap_year(year: u64) -> bool {
    (year % 100 == 0 && year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

fn main() {
    println!("{}", is_leap_year(1900));
}
