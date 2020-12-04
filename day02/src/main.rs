use regex::Regex;
use std::{fs::File, convert::TryInto};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Entry {
    bounds: (i32, i32),
    c: char,
    pw: String,
}

impl Entry {
    fn is_valid_old_policy(&self) -> bool {
        let sorted_pw = sorted_chars(&self.pw);
        let char_count = sorted_pw.iter().filter(|&&c| c == self.c).count();
        char_count >= self.bounds.0.try_into().unwrap() && char_count <= self.bounds.1.try_into().unwrap()
    }

    fn is_valid_new_policy(&self) -> bool {
        let chars: Vec<char> = self.pw.as_str().chars().collect();

        let bound_a: usize = (self.bounds.0 - 1).try_into().unwrap();
        let bound_b: usize = (self.bounds.1 - 1).try_into().unwrap();
        let a_is_match = chars[bound_a] == self.c;
        let b_is_match = chars[bound_b] == self.c;

        a_is_match ^ b_is_match
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let r = Regex::new(r"^(?P<lb>\d+)-(?P<ub>\d+)\s(?P<c>\w):\s(?P<pw>\w+)$").unwrap();
    let mut passwords = Vec::with_capacity(1000);

    for line_ref in io::BufReader::new(file).lines() {
        let line = String::from(line_ref.unwrap());
        let caps = r.captures(&line).unwrap();
        
        let bounds = (caps.name("lb").unwrap().as_str().parse::<i32>().unwrap(),
                                caps.name("ub").unwrap().as_str().parse::<i32>().unwrap());
        let c = caps.name("c").unwrap().as_str().chars().next().unwrap();
        let pw = String::from(caps.name("pw").unwrap().as_str());

        passwords.push(Entry {
            bounds,
            c,
            pw
        });
    }

    println!("{} passwords are valid on the old policy", passwords.iter().filter(|&e| e.is_valid_old_policy()).count());
    println!("{} passwords are valid on the new policy", passwords.iter().filter(|&e| e.is_valid_new_policy()).count());
}

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars
}
