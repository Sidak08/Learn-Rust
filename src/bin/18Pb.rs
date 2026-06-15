fn give_num(garden: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    // i = up/down
    // j = left right
    let mut counter: u32 = 0;
    if garden
        .get(i.wrapping_sub(1))
        .and_then(|r| r.get(j.wrapping_sub(1)))
        .copied()
        == Some('*')
    {
        counter += 1
    }
    if garden
        .get(i.wrapping_sub(1))
        .and_then(|r| r.get(j))
        .copied()
        == Some('*')
    {
        counter += 1
    }
    if garden
        .get(i.wrapping_sub(1))
        .and_then(|r| r.get(j + 1))
        .copied()
        == Some('*')
    {
        counter += 1
    }

    if garden
        .get(i)
        .and_then(|r| r.get(j.wrapping_sub(1)))
        .copied()
        == Some('*')
    {
        counter += 1
    }
    if garden.get(i).and_then(|r| r.get(j + 1)).copied() == Some('*') {
        counter += 1
    }

    if garden
        .get(i + 1)
        .and_then(|r| r.get(j.wrapping_sub(1)))
        .copied()
        == Some('*')
    {
        counter += 1
    }
    if garden.get(i + 1).and_then(|r| r.get(j)).copied() == Some('*') {
        counter += 1
    }
    if garden.get(i + 1).and_then(|r| r.get(j + 1)).copied() == Some('*') {
        counter += 1
    }

    if counter == 0 {
        return garden[i][j];
    }
    char::from_digit(counter, 10).unwrap()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    //convert to vec
    let mut garden: Vec<Vec<char>> = garden.iter().map(|word| word.chars().collect()).collect();
    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if garden[i][j] != '*' {
                garden[i][j] = give_num(&garden, i, j)
            }
        }
    }

    garden.iter().map(|row| row.iter().collect()).collect()
}

fn main() {
    let input = &["  *  ", "  *  ", "*****", "  *  ", "  *  "];

    println!("{:?}", annotate(input));
}
