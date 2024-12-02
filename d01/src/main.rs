use std::{collections::HashMap, fs};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let pair = line.split("   ").collect::<Vec<&str>>();
        let a = pair[0].parse::<i32>().unwrap();
        let b = pair[1].parse::<i32>().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for (i, elem) in left.iter().enumerate() {
        let diff = elem - right[i];
        sum += diff.abs();
    }

    println!("Task 1: {sum}");

    let mut freq = HashMap::new();

    for elem in right {
        let cnt = freq.entry(elem).or_insert(0);
        *cnt += 1;
    }

    let mut sum2 = 0;
    for elem in &left {
        let f = freq.get(elem);
        let val = match f {
            Some(val) => val,
            None => &0,
        };
        sum2 += elem * val;
    }

    println!("Task 2: {sum2}");
}
