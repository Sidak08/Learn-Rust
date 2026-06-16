use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let list: Vec<char> = string.chars().collect();
    let mut stack: Vec<char> = vec![];
    for i in 0..list.len() {
        match list[i] {
            '(' | '[' | '{' => stack.push(list[i]),
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return false;
                }
            }

            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    println!("{}", brackets_are_balanced("[]"))
}
