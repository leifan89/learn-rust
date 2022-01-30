use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, R> where T: Fn(U) -> R, U: Eq + Hash + Copy {
    calculation: T,
    values: HashMap<U, R>
}

impl<T, U, R> Cacher<T, U, R> where T: Fn(U) -> R, U: Eq + Hash + Copy {
    fn new(calculation: T) -> Cacher<T, U, R> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: U) -> &R {
        self.values.entry(arg).or_insert_with(|| (self.calculation)(arg))
    }
}



fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn generate_text(name: &str, message: &str) {
    let mut expensive_result = Cacher::new(|msg| {
        println!("Printing slowly...");
        thread::sleep(Duration::from_secs(2));
        msg
    });

    println!("Message for {}: {}", name, expensive_result.value(message));
    println!("Print message again {}", expensive_result.value(message));
}


fn main() {
    generate_workout(10, 7);
    generate_workout(15, 7);

    generate_text("Alice", "hello");
    generate_text("Bob", "hi");
}

