use rayon::prelude::*;
use is_prime::is_prime;

fn test(start: u32, num: u32) -> bool {
    let last = num % 10;
    if (last!=1) && (last!=3) && (last!=7) && (last!=9) {
        return false;
    }
    let mut number = String::new();
    for i in start..=num {
        number.push_str(&format!("{}",i));
    }
    let csum: u32 = number.chars().map(|c| c.to_digit(10).unwrap()).sum();
    if csum % 3 == 0 {
        // Multiple of 3
        return false;
    }
    let prime = is_prime(&number);
    if prime {
        println!("Prime: {}->{}", start, num);
    } else {
        println!("[{}]", num);
    }
    prime
}


fn check_start(start: u32) {
    let pr = ((start+1)..10000_u32).into_par_iter().any(|n| test(start,n));
    println!("pr {} {}", start, pr);
}

fn main() {
    /*
    for num in 20..30 {
        check_start(num);
    }
    */
    check_start(22);
}
