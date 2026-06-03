pub fn factors(n: u64) -> Vec<u64> {
    let mut i: u64 = 2;
    let mut curr_num: u64 = n;
    let mut answer: Vec<u64> = Vec::new();
    while curr_num >= i {
        if curr_num % i == 0 {
            answer.push(i);
            curr_num = curr_num / i;
        } else {
            i += 1;
        }
    }
    return answer;
}

fn main() {
    println!("{:?}", factors(60));
}
