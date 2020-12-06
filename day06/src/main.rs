use std::{fs::File, convert::TryInto, io::{self, BufRead}};

use arr_macro::arr;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut all_yes: [bool; 26] = arr![true; 26];

    for line_ref in io::BufReader::new(file).lines() {
        let line = line_ref.unwrap();
        if line.is_empty() {
            result += all_yes.iter().filter(|&&x| x).count();
            println!("all_yes = {:?}", all_yes);
            println!("result is {}", result);
            all_yes = arr![true; 26];
        } else {
            let mut this_yes: [bool; 26] = arr![false; 26];
            for c in line.to_ascii_lowercase().as_bytes() {
                let idx: usize = (c - 97).try_into().unwrap(); // ASCII a
                this_yes[idx] = true;
            }
            println!("{}, {:?}", line, this_yes);
            for (idx, yes) in this_yes.iter().enumerate() {
                all_yes[idx] = all_yes[idx] & yes;
            }
        }
    }
    result += all_yes.iter().filter(|&&x| x).count();

    println!("Total count is {}", result);
}
