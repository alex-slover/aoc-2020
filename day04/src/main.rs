use std::{fs::File, io::{self, BufRead, BufReader}};
use regex::Regex;

static FIELDS: [&str; 8] = ["cid", "pid", "ecl", "hcl", "hgt", "eyr", "iyr", "byr"];

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut valid_passports = 0;
    let mut has_fields: u8 = 0;

    let BYR: Regex = Regex::new(r"^\d{4}$").unwrap();
    let IYR: Regex = Regex::new(r"^\d{4}$").unwrap();
    let EYR: Regex = Regex::new(r"^\d{4}$").unwrap();
    let HGT: Regex = Regex::new(r"^(?P<val>\d+)(?P<unit>cm|in)$").unwrap();
    let HCL: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    let ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let PID: Regex = Regex::new(r"^\d{9}$").unwrap();
     
    for line_ref in io::BufReader::new(file).lines() {
        let line = line_ref.unwrap();
        println!("{}\n", line);
        if line.is_empty() {
            if has_fields >= 254 {
                println!("Valid!");
                valid_passports += 1;
            } else {
                println!("Invalid...");
            }
            has_fields = 0;
            continue;
        }

        for field in line.split_ascii_whitespace() {
            let parts: Vec<&str> = field.splitn(2, ':').collect();
            let k = parts[0];
            let v = parts[1];

            if let Some(i) = FIELDS.iter().position(|&f| f == k) {
                let valid = match k {
                    "byr" => {
                        if BYR.is_match(v) {
                            let year = v.parse::<i32>().unwrap();
                            year >= 1920 && year <= 2002
                        } else {
                            false
                        }
                    },
                    "iyr" => {
                        if IYR.is_match(v) {
                            let year = v.parse::<i32>().unwrap();
                            year >= 2010 && year <= 2020
                        } else {
                            false
                        }
                    },
                    "eyr" => {
                        if EYR.is_match(v) {
                            let year = v.parse::<i32>().unwrap();
                            year >= 2020 && year <= 2030
                        } else {
                            false
                        }
                    },
                    "hgt" => {
                        HGT.captures(v).map(|cap| {
                            let val = cap.name("val").unwrap().as_str().parse::<i32>().unwrap();
                            if cap.name("unit").unwrap().as_str() == "cm" {
                                val >= 150 && val <= 193
                            } else {
                                val >= 59 && val <= 76
                            }
                        }).unwrap_or(false)
                    },
                    "hcl" => HCL.is_match(v),
                    "ecl" => ECL.is_match(v),
                    "pid" => PID.is_match(v),
                    "cid" => false,
                    _ => false
                };
                print!("{}: {}", k, valid);
                if valid {
                    has_fields += 1 << i;
                }
            }

        }
    }
    if has_fields >= 254 {
        println!("Valid!");
        valid_passports += 1;
    } else {
        println!("Invalid...");
    }

    println!("{} valid passports", valid_passports);
}