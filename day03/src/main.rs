use std::{fs::File, convert::TryInto};
use std::io::{self, BufRead};

fn main() {
    let hill = read_slope("input.txt");
    let mut hit_count = 0;
    let mut hit_count_product = 1;
    let mut rightward_index = 1;

    for row in hill.iter().skip(1) {
        if row[rightward_index % row.len()] {
            hit_count += 1;
        }
        rightward_index += 1;
    }

    println!("Hit count is {}", hit_count);
    hit_count_product *= hit_count;
    hit_count = 0;
    rightward_index = 3;

    for row in hill.iter().skip(1) {
        if row[rightward_index % row.len()] {
            hit_count += 1;
        }
        rightward_index += 3;
    }

    println!("Hit count is {}", hit_count);
    hit_count_product *= hit_count;
    hit_count = 0;
    rightward_index = 5;
    
    for row in hill.iter().skip(1) {
        if row[rightward_index % row.len()] {
            hit_count += 1;
        }
        rightward_index += 5;
    }

    println!("Hit count is {}", hit_count);
    hit_count_product *= hit_count;
    hit_count = 0;
    rightward_index = 7;

    for row in hill.iter().skip(1) {
        if row[rightward_index % row.len()] {
            hit_count += 1;
        }
        rightward_index += 7;
    }

    println!("Hit count is {}", hit_count);
    hit_count_product *= hit_count;
    hit_count = 0;
    rightward_index = 1;

    for row in hill.iter().skip(2).step_by(2) {
        println!("{:?}", row);
        if row[rightward_index % row.len()] {
            hit_count += 1;
            println!("hit, r_i = {}", rightward_index);
        } else {
            println!("miss r_i = {}", rightward_index);
        }
        rightward_index += 1;
    }

    println!("Hit count is {}", hit_count);
    hit_count_product *= hit_count;

    println!("Final hit count product is {}", hit_count_product);
}

fn read_slope(filename: &str) -> Vec<Vec<bool>> {
    let file = File::open(filename).unwrap();
    let mut hill = Vec::with_capacity(323);
    for line in io::BufReader::new(file).lines() {
        let row: Vec<bool> = line.unwrap().as_bytes().iter().map(|&b| b == '#' as u8).collect();
        hill.push(row);
    }
    
    hill
}