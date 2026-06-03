pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect();
}

fn main() {
    let a = reverse("Hello");
    println!("{}", a);
}
