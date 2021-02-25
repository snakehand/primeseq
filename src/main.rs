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
    let prime = is_prime(&number);
    if prime {
        println!("Prime: {}->{}", start, num);
    } else {
        // println!("[{}]", num);
    }
    prime
}


fn check_start(start: u32) {
    let pr = ((start+1)..1000_u32).into_par_iter().any(|n| test(start,n));
    println!("pr {} {}", start, pr);
}

fn main() {
    for num in 20..30 {
        check_start(num);
    }
}
