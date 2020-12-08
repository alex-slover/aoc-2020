use std::{fs::File, convert::TryInto, io::{self, BufRead}};

use arr_macro::arr;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut yes_to_all: [bool; 26] = arr![true; 26];

    for line_ref in io::BufReader::new(file).lines() {
        let line = line_ref.unwrap();
        if line.is_empty() {
            result += yes_to_all.iter().filter(|&&x| x).count();
            yes_to_all = arr![true; 26];
        } else {
            let mut yes_to_this: [bool; 26] = arr![false; 26];
            for c in line.to_ascii_lowercase().as_bytes() {
                let idx: usize = (c - 97).try_into().unwrap(); // ASCII a
                yes_to_this[idx] = true;
            }

            for (idx, yes) in yes_to_this.iter().enumerate() {
                yes_to_all[idx] = yes_to_all[idx] & yes;
            }
        }
    }
    result += yes_to_all.iter().filter(|&&x| x).count();

    println!("Total count is {}", result);
}
