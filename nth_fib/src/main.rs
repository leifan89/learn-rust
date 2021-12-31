use std::io;

// A somewhat contrived example of how to calculate the nth Fibonacci
// number as part of learning Rust

const MAX_FIB_64_BIT : usize = 93;

fn main() {
    let n = get_input();

    if n > MAX_FIB_64_BIT {
        println!("The largest Fibonacci number that will fit in a uint64 is #{}", MAX_FIB_64_BIT);
        println!("If you are curious, that number is {}", nth_fib(MAX_FIB_64_BIT));   
    } else {
        let val = nth_fib(n);
        println!("The #{} Fibonacci number is {}", n, val);
    }
}

fn get_input() -> usize {
    println!("Which Fibonacci number would you like?");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
    let n : usize = n.trim().parse().expect("Needs to be a valid non-negative number");

    return n;
}

fn nth_fib(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    if n <= 2 {
        return 1;
    }

    let mut history : Vec<u64> = vec![0, 1, 1];
    
    for i in 3..=n {
        history.push(prev_two(i, &history));
    }

    return history[n];
}

fn prev_two(i: usize, history: &Vec<u64>) -> u64 {
    return history[i-2] + history[i-1];
}