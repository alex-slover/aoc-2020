use std::{fs::File, io::{self, BufRead, BufReader}};

// struct Passport {
//     byr: Option<i32>,
//     iyr: Option<i32>,
//     eyr: Option<i32>,
//     hgt: Option<i32>,
//     hcl: Option<String>,
//     ecl: Option<String>,
//     pid: Option<String>,
//     cid: Option<String>
// }

// impl Passport {
//     fn new() -> Passport {
//         Passport {
//             byr: None,
//             iyr: None,
//             eyr: None,
//             hgt: None,
//             hcl: None,
//             ecl: None,
//             pid: None,
//             cid: None
//         }
//     }
// }

static FIELDS: [&str; 8] = ["cid", "pid", "ecl", "hcl", "hgt", "eyr", "iyr", "byr"];

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut valid_passports = 0;
    let mut has_fields: u8 = 0;
     
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

            if k != "cid" { 
                if let Some(i) = FIELDS.iter().position(|&f| f == k) {
                    has_fields += 1 << i;
                    print!(" field {}, has_fields = {}   ", k, has_fields);
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