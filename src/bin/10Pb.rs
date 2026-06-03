use std::collections::HashSet;

fn mutiples(num: u32, to_reach_num: u32) -> Vec<u32> {
    let mut answer: Vec<u32> = Vec::new();
    if num == 0 {
        answer.push(0);
        return answer;
    }
    for i in 1..=(to_reach_num / num) {
        if num * i != to_reach_num {
            answer.push(num * i);
        }
    }
    return answer;
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_factors: HashSet<u32> = HashSet::new();
    let mut total: u32 = 0;
    for i in 0..factors.len() {
        let mutiples: Vec<u32> = mutiples(factors[i], limit);
        for mutiple in mutiples {
            all_factors.insert(mutiple);
        }
    }

    for num in all_factors {
        total += num
    }

    return total;
}

fn main() {
    println!("{:#?}", sum_of_multiples(4, &[3, 0]));
}
