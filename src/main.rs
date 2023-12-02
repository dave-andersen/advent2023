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

fn read_all_lines(filename: &str) -> Vec<String> {
    open_buffered(filename)
        .lines()
        .map_while(Result::ok)
        .collect()
}

#[allow(dead_code)]
fn a1(part_two: bool) {
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

    let tot: u32 = open_buffered(&input)
        .lines()
        .map_while(Result::ok)
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
}

fn a2() {
    let input = input_file("a2.1");
    let lines = read_all_lines(&input);
    // only 12 red cubes, 13 green cubes, and 14 
    let mut gametot = 0;
    let mut powertot = 0;
    for (line_no, line) in lines.iter().enumerate() {
        let gameid = line_no + 1;
        // Game 77: 10 blue, 2 red, 5 green; 5 green, 3 red, 12 blue; ...
        let rgb = ["red", "green", "blue"].into_iter().map(|color| {
            let re = Regex::new(&format!("(\\d+) {color}")).unwrap();
            
            re.captures_iter(line).map(|c| {
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
}

fn main() {
    // a1(false);
    // a1(true);
    a2();
}
