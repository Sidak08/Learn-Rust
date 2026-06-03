pub fn build_proverb(list: &[&str]) -> String {
    let mut answer: String = String::new();
    if list.len() == 0 {
        return answer;
    }
    for i in 0..(list.len() - 1) {
        answer.push_str(&build_line(list[i], list[i + 1]));
    }

    answer.push_str(&format!("And all for the want of a {}.", list[0]));
    return answer;
}

fn build_line(word1: &str, word2: &str) -> String {
    format!("For want of a {word1} the {word2} was lost.\n")
}

fn main() {
    println!(
        "{}",
        build_proverb(&[
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom"
        ])
    );
}
