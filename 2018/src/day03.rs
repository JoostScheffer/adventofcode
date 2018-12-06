use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::ops::Range;

use common;
use regex;

#[derive(Copy, Clone, Debug)]
struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {

    fn xrange(&self) -> Range<usize> {
        self.x..(self.x + self.width)
    }

    fn yrange(&self) -> Range<usize> {
        self.y..(self.y + self.height)
    }

    pub fn range(&self) -> impl Iterator<Item=(usize, usize)> {
        iproduct!(self.xrange(), self.yrange())
    }
}

#[derive(Default)]
pub struct Day03 {
    claims: Vec<Claim>
}


impl Day03 {
    pub fn new() -> Day03 {
        Default::default()
    }

    fn read_claims(&mut self, input: &mut io::Read) {
        let reader = io::BufReader::new(input);
        self.claims.clear();

        let matcher = regex::Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let matched = matcher.captures(&line).unwrap();

            let claim = Claim{
                x: matched[2].parse().unwrap(),
                y: matched[3].parse().unwrap(),
                width: matched[4].parse().unwrap(),
                height: matched[5].parse().unwrap(),
            };
            self.claims.push(claim);
        }
    }

    fn get_claims(&self) -> HashMap<(usize, usize), i32> {
        let mut claims = HashMap::new();
        for claim in &self.claims {
            for coordinate in claim.range() {
                *claims.entry(coordinate).or_insert(0) += 1;
            }
        }

        claims
    }
}


impl common::Solution for Day03 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        self.read_claims(input);
        let claim_map = self.get_claims();

        let multi_claim = claim_map.values()
            .filter(|&&x| x > 1)
            .count();

        format!("{}", multi_claim)
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.read_claims(input);
        let claims = self.get_claims();

        let uncontested = self.claims.iter()
            .position(|x| x.range().all(|x| *claims.get(&x).unwrap() == 1))
            .unwrap();

        format!("{}", uncontested + 1)
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;

    use super::*;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/03.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day03::new();
        let result = instance.part1(&mut SAMPLE_INPUT);
        assert_eq!("4", result);
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day03::new();
        let result = instance.part2(&mut SAMPLE_INPUT);
        assert_eq!("3", result);
    }
}
