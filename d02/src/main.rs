use std::fs;

fn first_task(nums: &Vec<i32>) -> i32 {
    let mut diffs = vec![];
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        if diff == 0 {
            return 0;
        }
        diffs.push(diff);
    }

    let sign = diffs.first().unwrap() > &0;
    for elem in diffs {
        if sign && elem < 0 || !sign && elem > 0 || elem.abs() > 3 {
            return 0;
        }
    }
    1
}

fn second_task(nums: &Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let mut cl = vec![];
        for j in 0..nums.len() {
            if j == i {
                continue;
            }
            cl.push(nums[j] as i32);
        }

        if first_task(&cl) == 1 {
            return 1;
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut safe1 = 0;
    let mut safe2 = 0;
    for line in content.lines() {
        if line.is_empty() {
            break;
        }

        let nums: Vec<i32> = line
            .split(" ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        safe1 += first_task(&nums);
        safe2 += second_task(&nums);
    }

    println!("Task 1: {safe1}");
    println!("Task 2: {safe2}");
}
