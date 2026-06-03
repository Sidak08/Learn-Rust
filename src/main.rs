struct bankAccount {}

// #![allow(warnings)]
fn main() {
    println!("Hello, world! coming from cargo");
    let x: i32 = -42;
    let y: u128 = 142;
    println!("x: {}", x);
    println!("y: {}", y);

    let fraction: f64 = -3.1426;
    println!("{}", fraction);

    let isSnowing: bool = true;
    println!("{}", isSnowing);

    //only a sinlge unicode value
    let myNameChar: char = 'S';
    println!("{}", myNameChar);

    let numArray: [i32; 5] = [1, 2, 3, 4, 5];

    let fruits: [&str; 3] = ["orang", "apple", "cow"];

    // let human: (i32, i32, bool) = (43, 30, false);
    let myMixTuple = ("Kratos", true, 23, [0, 4, 56]);

    let mut stone_cold: String = String::from("Hello, ");
    stone_cold.push_str("world");
    println!("{}", &stone_cold[0..7]);

    something_about_me(180, 18, "Sidak Singh");

    println!("{} + {} = {}", 5, 6, add(5, 6));

    let s1 = String::from("Rust");
    println!("{}, {}, {}, {}", s1, &s1, s1.len(), &s1.len());
    println!("{}, {}, {}, {}", s1, &s1, s1.len(), &s1.len());

    count_to_inf();
}

// fn what_you_hieght_boi(height: i32) {
//     let la: String = String::from("LA LA");
//     println!("{}", la)
// }

fn something_about_me(height: u8, age: u8, name: &str) {
    println!(
        "Hi my name is {} and I am {} years old while being {}cm tall",
        name, age, height,
    )
}

fn add(a: i16, b: i16) -> i16 {
    a + b
    // or you can write return a + b
}

fn count_to_inf() {
    let mut num: u32 = 0;
    loop {
        let var = 2__i32.pow(num);
        println!("{var}");
        if num == 100000000 {
            break;
        } else {
            num = num + 1;
        }
    }
}
