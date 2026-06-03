/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code: Vec<char> = code.to_string().chars().filter(|&str| str != ' ').collect();
    if code.len() == 1 || code.iter().any(|num| !num.is_digit(10)) {
        return false;
    }
    let mut sum: u32 = 0;
    for i in (1..code.len()).rev().step_by(2) {
        let mut ans: u32 = code[i - 1].to_digit(10).unwrap() * 2;
        if ans > 9 {
            ans -= 9
        };
        code[i - 1] = ans.to_string().chars().next().unwrap();
    }

    for i in 0..code.len() {
        sum += code[i].to_digit(10).unwrap()
    }

    sum % 10 == 0
}

fn main() {
    println!("{}", is_valid("066 123 478"));
}
