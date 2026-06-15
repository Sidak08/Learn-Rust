#[derive(Debug, PartialEq, Eq)]

pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    //case 1) both of them are eqaual
    if first_list == second_list {
        return Comparison::Equal;
    };

    if first_list.is_empty() || second_list.is_empty() {
        return if first_list.is_empty() {
            Comparison::Sublist
        } else {
            Comparison::Superlist
        };
    }

    // Case 2) List 1 contains List 2
    for i in 0..first_list.len() {
        if i + second_list.len() <= first_list.len() && first_list[i] == second_list[0] {
            if (0..second_list.len()).all(|j| first_list[i + j] == second_list[j]) {
                return Comparison::Superlist;
            }
        }
    }

    // Case 3) List 2 contains List 1
    for i in 0..second_list.len() {
        if i + first_list.len() <= second_list.len() && second_list[i] == first_list[0] {
            if (0..first_list.len()).all(|j| second_list[i + j] == first_list[j]) {
                return Comparison::Sublist;
            }
        }
    }

    return Comparison::Unequal;
}

fn main() {
    println!("{:?}", sublist(&[1, 2, 3], &[0, 1, 2, 3, 5, 6]))
}
