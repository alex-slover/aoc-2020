use std::{fs::File, convert::TryInto, io::{self, BufRead}};
use arr_macro::arr;

use regex::Regex;

fn main() {
    let r = Regex::new(r"(?P<row>[FB]{7})(?P<seat>[LR]{3})").unwrap();
    let file = File::open("input.txt").unwrap();
    let mut seen: [bool; 1024] = arr![false; 1024];
    let BASE: u32 = 2;
    let SIX: u32 = 6;
    let TWO: u32 = 2;
    let mut highest: u32 = 0;


    for line in io::BufReader::new(file).lines() {
        let caps = r.captures(line.as_ref().unwrap()).unwrap();
        let mut row = String::from(caps.name("row").unwrap().as_str());
        row.make_ascii_uppercase();
        let mut row_number = 0;
        for (r, dir) in row.chars().enumerate() {
            if dir == 'B' {
                let idx: u32 = r.try_into().unwrap();
                row_number += BASE.pow(SIX - idx);
            }
        }

        let mut seat = String::from(caps.name("seat").unwrap().as_str());
        seat.make_ascii_uppercase();
        let mut seat_number = 0;
        for (s, dir) in seat.chars().enumerate() {
            if dir == 'R' {
                let idx: u32 = s.try_into().unwrap();
                seat_number += BASE.pow(TWO - idx);
            }
        }

        let seat_id = row_number * 8 + seat_number;
        highest = if seat_id > highest { seat_id } else { highest };

        let as_index: usize = seat_id.try_into().unwrap();
        seen[as_index] = true;
    }

    let mut on_plane = false;
    for (seat, was_seen) in seen.iter().enumerate() {
        match (on_plane, was_seen) {
            (false, true) => on_plane = true,
            (true, false) => {
                println!("Your seat is {}", seat);
                break;
            },
            _ => ()
        }
    }

    println!("Highest seat id is {}", highest);
}
