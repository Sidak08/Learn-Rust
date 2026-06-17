pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits: Vec<char> = digits.chars().collect();
    let mut answer: Vec<String> = Vec::new();
    for window in digits.windows(len) {
        answer.push(window.iter().collect());
    }
    return answer;
}

fn main() {
    println!("{:?}", series("49142", 3))
}
