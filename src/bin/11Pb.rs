fn sent_gen(num: u32) -> String {
    let mut answer = String::new();
    let num_word = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "Unknown",
    };
    let num_word_2 = match num - 1 {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "Unknown",
    };
    answer.push_str(&format!(
        "{num_word} green bottle{} hanging on the wall,\n",
        if num == 1 { "" } else { "s" }
    ));
    answer.push_str(&format!(
        "{num_word} green bottle{} hanging on the wall,\n",
        if num == 1 { "" } else { "s" }
    ));
    answer.push_str(&format!(
        "And if one green bottle should accidentally fall,\n"
    ));

    if num - 1 != 0 {
        answer.push_str(&format!(
            "There'll be {} green bottle{} hanging on the wall.\n",
            num_word_2.to_ascii_lowercase(),
            if num - 1 == 1 { "" } else { "s" }
        ));
    } else {
        answer.push_str(&format!(
            "There'll be no green bottles hanging on the wall.\n"
        ));
    }

    return answer;
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut answer = String::new();
    for i in 0..take_down {
        answer.push_str(&sent_gen(start_bottles - i));
        answer.push_str("\n");
    }
    answer.pop();
    answer.pop();
    return answer;
}

fn main() {
    println!("{}", recite(2, 2));
}
