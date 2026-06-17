pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: Vec<char> = diagram
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let row2Start: usize = diagram.len() / 2;
    let student: usize = (student.chars().collect::<Vec<char>>()[0] as usize - 'A' as usize) * 2;
    let mut answer: Vec<&'static str> = Vec::new();

    answer.push(map(diagram[student]));
    answer.push(map(diagram[student + 1]));
    answer.push(map(diagram[student + row2Start]));
    answer.push(map(diagram[student + row2Start + 1]));

    println!("{:?}", diagram);
    println!("student: {student}, row to start: {row2Start}");
    return answer;
}

fn map(n: char) -> &'static str {
    //println!("the n passed in was {n}");
    match n {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "No Clue",
    }
}

fn main() {
    println!(
        "{:?}",
        plants(
            "VRCGVVRVCGGCCGVRGCVCGCGV
            VRCCCGCRRGVCGCRVVCVGCGCV",
            "Bob"
        )
    )
}
