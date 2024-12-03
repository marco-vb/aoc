use std::fs;

use regex::Regex;

fn first_task(data: &str) -> i64 {
    let r = Regex::new(r"mul\((?<l>[0-9]+),(?<r>[0-9]+)\)").unwrap();
    let mut sum: i64 = 0;
    for line in data.lines() {
        if line.is_empty() {
            break;
        }

        let matches = r.captures_iter(line);
        for m in matches {
            let l = m["l"].parse::<i64>().unwrap();
            let r = m["r"].parse::<i64>().unwrap();
            sum += l * r;
        }
    }
    sum
}

fn second_task(data: &str) -> i64 {
    let r = Regex::new(r"(do\(\)|mul\((?<l>[0-9]+),(?<r>[0-9]+)\)|don't\(\))").unwrap();
    let mut sum: i64 = 0;
    let mut can = true;
    for line in data.lines() {
        if line.is_empty() {
            break;
        }

        let matches = r.captures_iter(line);
        for m in matches {
            let text = m.get(0).unwrap().as_str();

            if text.starts_with("don") {
                can = false;
            } else if text.starts_with("do") {
                can = true;
            } else if can {
                let l = m["l"].parse::<i64>().unwrap();
                let r = m["r"].parse::<i64>().unwrap();
                sum += l * r;
            }
        }
    }
    sum
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let sum1 = first_task(&data);
    let sum2 = second_task(&data);

    println!("Task 1: {sum1}");
    println!("Task 2: {sum2}");
}
