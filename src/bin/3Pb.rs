pub fn is_armstrong_number(num: u32) -> bool {
    let num: String = num.to_string();
    let mut total: u32 = 0;
    for char in num.chars() {
        total += char.to_digit(10).unwrap().pow(num.len() as u32)
    }
    total == num.parse().unwrap()
}

fn main() {
    println!("{}", is_armstrong_number(9))
}
