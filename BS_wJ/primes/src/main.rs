// Trial division
fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => return false,
        2 | 3 => return true,
        _ => {
            let up_to: u32 = (n as f64).sqrt().ceil() as u32;
            for i in 2..=up_to {
                if n % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}

fn get_primes_v1(up_to: u32) -> Vec<u32> {
    if up_to < 2 {
        return vec![];
    } else {
        return (2..=up_to).filter(|n| is_prime(*n)).collect();
    }
}

// Sieve of Eratosthenes
fn get_primes_v2(up_to: u32) -> Vec<u32> {
    let mut is_prime: Vec<bool> = vec![true; up_to as usize];
    let mut result: Vec<u32> = vec![];
    is_prime[0] = false; // first prime is 2
    for num in 1..=up_to {
        // mark multiples of a prime (num) as not prime
        if is_prime[num as usize - 1] {
            for i in ((num * 2)..=up_to).step_by(num as usize) {
                is_prime[i as usize - 1] = false;
            }
        }
    }
    for i in 1..=is_prime.len() {
        if is_prime[i - 1] {
            result.push(i as u32);
        }
    }
    result
}

fn print_intro() {
    println!("Toy program for primes problem.");
    println!("Warning. It may or may not work correctly.\n");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn print_solution() {
    println!("Printing prime numbers upto 100");
    println!("\nMethod 1. Trial division.");
    println!("{:?}", get_primes_v1(100));
    println!("\nMethod 2. Sieve of Eratosthenes.");
    println!("{:?}", get_primes_v2(100));
}

fn main() {
    print_intro();

    print_solution();

    print_outro();
}
