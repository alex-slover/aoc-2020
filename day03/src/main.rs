use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let hill = read_slope("input.txt");
    let mut hit_count_product = 1;

    hit_count_product *= tree_impacts(&hill, 1, 1);
    hit_count_product *= tree_impacts(&hill, 3, 1);
    hit_count_product *= tree_impacts(&hill, 5, 1);
    hit_count_product *= tree_impacts(&hill, 7, 1);
    hit_count_product *= tree_impacts(&hill, 1, 2);

    println!("Final hit count product is {}", hit_count_product);
}

fn tree_impacts(hill: &Vec<Vec<bool>>, right_by: usize, down_by: usize) -> i32 {
    let mut hit_count = 0;
    let mut righward_index = right_by;
    for row in hill.iter().skip(down_by).step_by(down_by) {
        if row[righward_index % row.len()] {
            hit_count += 1;
        }
        righward_index += right_by;
    }
    hit_count
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