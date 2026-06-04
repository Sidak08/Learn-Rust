pub fn egg_count(display_value: u32) -> usize {
    let display_value: Vec<char> = format!("{display_value:b}").chars().collect();
    let mut counter: usize = 0;
    for char in display_value {
        if char == '1' {
            counter += 1
        }
    }
    counter
}

fn main() {
    println!("{}", egg_count(2_000_000_000));
}
