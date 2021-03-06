use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn most_frequent(counts: &HashMap<char, i32>) -> char
{
    let (mut c, mut freq) = (' ', 0);
    for (cc, cfreq) in counts {
        if *cfreq > freq {
            freq = *cfreq;
            c = *cc;
        }
    }
    return c;
}

fn least_frequent(counts: &HashMap<char, i32>) -> char
{
    let (mut c, mut freq) = (' ', i32::max_value());
    for (cc, cfreq) in counts {
        if *cfreq < freq {
            freq = *cfreq;
            c = *cc;
        }
    }
    return c;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut counts: Vec<HashMap<char, i32>> = Vec::new();

    for line in reader.lines() {
        for (i, c) in line.unwrap().trim().chars().enumerate() {
            if i >= counts.len() {
                counts.push(HashMap::new());
            }

            let cur = counts[i].entry(c).or_insert(0);
            *cur += 1;
        }
    }

    let decoded: String = counts.iter().map(|counts| most_frequent(counts)).collect();
    let least_decoded: String = counts.iter().map(|counts| least_frequent(counts)).collect();
    println!("Final (most frequent) word: {}", decoded);
    println!("Final (least frequent) word: {}", least_decoded);
}
