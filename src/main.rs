#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

const BASEDIR: &str = "/Users/dga/rust/advent2023";
const INPUTDIR: &str = "inputs";

fn input_file(puzzle: &str) -> String {
    format!("{BASEDIR}/{INPUTDIR}/{puzzle}.txt")
}

fn open_buffered(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn all_lines(filename: &str) -> impl Iterator<Item = String> {
    open_buffered(filename).lines().map_while(Result::ok)
}

fn all_nonblank_lines(filename: &str) -> impl Iterator<Item = String> {
    all_lines(filename).filter(|line| !line.is_empty())
}

fn read_all_lines(filename: &str) -> Vec<String> {
    all_lines(filename).collect()
}

// byte slice to hashset
fn bs_to_set(bytes: &[u8]) -> HashSet<u8> {
    let mut hs = HashSet::with_capacity(bytes.len());
    for &b in bytes {
        hs.insert(b);
    }
    hs
}

fn a1(part_two: bool) -> u32 {
    let input = input_file("a1.1");
    let mut patterns = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let values = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    if !part_two {
        patterns.truncate(10);
    }

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();

    let tot: u32 = all_lines(&input)
        .map(|line| {
            let digits: Vec<_> = ac
                .find_overlapping_iter(&line)
                .map(|pid| values[pid.pattern().as_usize()])
                .collect();
            if digits.is_empty() {
                0
            } else {
                digits[0] * 10 + digits[digits.len() - 1]
            }
        })
        .sum();
    println!("Total: {tot}");
    tot
}

// day 2:
// only 12 red cubes, 13 green cubes, and 14 
// input looks like:
// Game 77: 10 blue, 2 red, 5 green; 5 green, 3 red, 12 blue; ...

fn a2() -> (usize, u64) {
    let input = input_file("a2.1");
    let mut gametot = 0;
    let mut powertot = 0;
    for (line_no, line) in all_lines(&input).enumerate() {
        let gameid = line_no + 1;
        let rgb = ["red", "green", "blue"].into_iter().map(|color| {
            let re = Regex::new(&format!("(\\d+) {color}")).unwrap();
            
            re.captures_iter(&line).map(|c| {
                let (_, [count]) = c.extract();
                count.parse::<u32>().unwrap()
            }).max().unwrap_or(0)
        }).collect::<Vec<_>>();

        if rgb[0] <= 12 && rgb[1] <= 13 && rgb[2] <= 14 {
            gametot += gameid;
        }
        powertot += (rgb[0] * rgb[1] * rgb[2]) as u64;

    }
    println!("Gametot: {gametot}");
    println!("Powertot: {powertot}");
    (gametot, powertot)
}


// Just solving to see what utility functions are handy. :-)
fn a3_2022() -> usize {
    let input = input_file("2022.a3");
    all_nonblank_lines(&input).map(|line| {
        let half = line.len() / 2;
        let b = line.as_bytes();
        let chars1 = bs_to_set(&b[0..half]);
        let chars2 = bs_to_set(&b[half..]);
        let mut shared = chars1.intersection(&chars2);
        let shared = *shared.next().unwrap();
        if shared >= b'a' && shared <= b'z' { (shared - b'a' + 1) as usize } else { (shared - b'A' + 27) as usize}
    }).sum()
}

fn main() {
    //a1(false);
    //a1(true);
    //a2();
    println!("{}", a3_2022());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {
        assert_eq!(54968, super::a1(false));
        assert_eq!(54094, super::a1(true));
        assert_eq!((2795, 75561), super::a2());
    }

    fn test_2022() {
        assert_eq!(super::a3_2022(), 8298);
    }
}