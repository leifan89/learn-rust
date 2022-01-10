use std::collections::HashMap;

// Calculate mean, median, mode for a list of numbers

fn main() {
    let nums = vec![0, -1, 1000, -1, 5, 3, 0];
    match mean(&nums) {
        Some(mean) => println!("Mean is {}", mean),
        None => println!("No mean exists - list is empty")
    };
    match median(&nums) {
        Some(median) => println!("Median is {}", median),
        None => println!("No median exists - list is empty")
    };
    match mode(&nums) {
        Some(mode) => println!("Mode is {:?}", mode),
        None => println!("No mode exists")
    };
}

fn mean(nums: &Vec<i64>) -> Option<f64> {
    if nums.len() == 0 {
        return None;
    }

    let mut sum = 0.0;
    for &num in nums {
        sum += num as f64;
    }
    return Some(sum / (nums.len() as f64));
}

fn median(nums: &Vec<i64>) -> Option<f64> {
    if nums.len() == 0 {
        return None;
    }

    let mut sorted = nums.clone();
    sorted.sort();

    if nums.len() % 2 == 0 {
        // even number of elements, average the middle two
        let index = nums.len() / 2;
        return Some((sorted[index-1] as f64 + sorted[index] as f64) / 2.0);
    } else {
        // odd number of elements, take the middle one
        let index = nums.len() / 2;  // floors
        return Some(sorted[index] as f64);
    }
}

fn mode(nums: &Vec<i64>) -> Option<Vec<i64>> {
    let mut map = HashMap::new();
    let mut largest_count = -1;

    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;

        if *count > largest_count {
            largest_count = *count;
        }
    }

    // There is no mode if the list contains only 1 of each element, or if list is empty
    if largest_count <= 1 {
        return None;
    }    

    let mut modes = Vec::new();
    for (&value, count) in map {
        if count == largest_count {
            modes.push(value);
        }
    }

    return Some(modes);
}
