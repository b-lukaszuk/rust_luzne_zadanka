use std::collections::HashMap;

fn get_median(v: &Vec<i64>) -> f64 {
    let mut v2: Vec<i64> = v.clone();
    v2.sort();
    let len: usize = v2.len();
    if len % 2 == 0 {
        let mid: usize = len / 2;
        return (v2[mid - 1] + v2[mid]) as f64 / 2.;
    } else {
        return v2[len / 2] as f64;
    }
}

fn get_counts(v: &Vec<i64>) -> HashMap<i64, u64> {
    let mut counts: HashMap<i64, u64> = HashMap::new();
    for elt in v {
        let count = counts.entry(*elt).or_insert(0);
        *count += 1;
    }
    counts
}

fn get_max_count(counts: &HashMap<i64, u64>) -> u64 {
    let mut max: u64 = 0;
    for (_, v) in counts.iter() {
        if *v > max {
            max = *v;
        }
    }
    max
}

fn get_keys_for_max_count(counts: &HashMap<i64, u64>) -> Vec<i64> {
    let max: u64 = get_max_count(counts);
    let mut result: Vec<i64> = Vec::new();
    for (k, v) in counts.iter() {
        if *v == max {
            result.push(*k);
        }
    }
    result
}

fn get_mode(nums: &Vec<i64>) -> Vec<i64> {
    let counts: HashMap<i64, u64> = get_counts(nums);
    get_keys_for_max_count(&counts)
}

fn main() {
    let nums: Vec<i64> = vec![4, 3, 2, 8, 10];
    println!("The median of {:?} is {}", nums, get_median(&nums));
    let nums: Vec<i64> = vec![4, 3, 2, 8, 10, 1];
    println!("The median of {:?} is {}", nums, get_median(&nums));
    let nums: Vec<i64> = vec![3, 4, 3, 2, 8, 8, 1, 10, 1];
    println!("mode(s) in: {:?} is: {:?}", nums, get_mode(&nums));
    let nums: Vec<i64> = vec![1, 3, 4, 3, 2, 8, 8, 1, 10, 1];
    println!("mode(s) in: {:?} is: {:?}", nums, get_mode(&nums));
}
