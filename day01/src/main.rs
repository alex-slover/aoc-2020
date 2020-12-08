use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut nums = Vec::with_capacity(200);
    for line in io::BufReader::new(file).lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        nums.push(num);
    }
    nums.sort();

    let (a, b) = find_2_sum(&nums).expect("No solution found for 2 :(");
    println!("Found it! {} + {} = 2020, product is {}", a, b, a * b);

    let (a, b, c) = find_3_sum(&nums).expect("No solution found for 3 :(");
    println!("Found it! {} + {} + {} = 2020, product is {}", a, b, c, a * b * c);
}

fn find_2_sum(nums: &Vec<i32>) -> Option<(i32, i32)> {
    let mut start = 0;
    let mut end = nums.len() - 1;

    while start < end {
        let a = nums[start];
        let b = nums[end];

        match (a + b).cmp(&2020) {
            std::cmp::Ordering::Less => start = start + 1,
            std::cmp::Ordering::Greater => end = end - 1,
            std::cmp::Ordering::Equal => return Some((a, b))
        }
    }

    None
}

fn find_3_sum(nums: &Vec<i32>) -> Option<(i32, i32, i32)> {
    let mut start = 0;
    let mut end = nums.len() - 1;

    while start < end {
        while start < end {
            let a = nums[start];
            let b = nums[end];
            let c = 2020 - (a + b);

            if c > 0 {
                if nums.binary_search(&c).is_ok() {
                    return Some((a, b, c));
                }
            }
            start = start + 1;
        }
        start = 0;
        end = end - 1;
    }

    None
}
