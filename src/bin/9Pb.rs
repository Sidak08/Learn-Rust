fn is_prime(n: f64) -> bool {
    if n as u32 == 2 {
        return true;
    }

    if n as u32 == 1 {
        return false;
    }

    for i in 2..n.sqrt().ceil() as i32 + 1 {
        //println!("checking is prime {}, {}, {}", n, i, n as i32 % i == 0);
        if n as i32 % i == 0 {
            return false;
        }
    }
    if n as i32 % 2 == 0 {
        return false;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let n: u32 = n + 1;
    let mut counter: u32 = 1;
    let mut counter_num: u32 = 2;

    // println!("{}, {}", is_prime(12.0), is_prime(6.0))
    while true {
        if counter == n + 1 {
            return counter_num - 1;
            break;
            // return counter_num;
        }
        if is_prime(counter_num as f64) {
            //println!("{}", counter_num);
            counter += 1
        }
        counter_num += 1
    }

    return 0;
}

fn main() {
    println!("{}", nth(1_000_000));
}
