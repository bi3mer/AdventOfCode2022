use std::fs;
use std::time::Instant;
use std::cmp::{min, max};

fn part_1(input: &str) -> u32 {
    input
    .lines()
    .map(|l| {
        let pair = l.split(",").map(|r| {
            r.split("-").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        }).collect::<Vec<_>>();

        let contained1 = pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1];
        let contained2 = pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1];

        (contained1 || contained2) as u32
    })
    .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    input
    .lines()
    .map(|l| {
        let pair = l.split(",").map(|r| {
            r.split("-").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        }).collect::<Vec<_>>();

        let min0 = max(pair[0][0], pair[1][0]);
        let max0 = min(pair[0][1], pair[1][1]);

        (min0 <= max0) as u32
    })
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    pub fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    pub fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 4);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1_now = Instant::now();
    let res1 = part_1(&input);
    let part1_elapsed = part1_now.elapsed();
    
    let part2_now = Instant::now();
    let res2 = part_2(&input);
    let part2_elapsed = part2_now.elapsed();

    println!("Part 1 ({:?}): {}", part1_elapsed, res1);
    println!("Part 2 ({:?}): {}", part2_elapsed, res2);
}
