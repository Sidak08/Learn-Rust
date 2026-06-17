pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    } else {
        return Some(recursive(n, 0));
    }
}

fn recursive(num: u64, step: u64) -> u64 {
    if num == 1 {
        return step;
    } else if num % 2 == 0 {
        return recursive(num / 2, step + 1);
    } else {
        return recursive((num * 3) + 1, step + 1);
    }
}

fn main() {
    println!("{}", recursive(12, 0))
}
