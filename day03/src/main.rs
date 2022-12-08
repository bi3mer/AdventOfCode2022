use std::fs;
use std::time::Instant;

fn get_index(b: u8) -> usize {
    if b >= b'a' && b <= b'z' {
        (b - b'a') as usize
    } else {
        (b - b'A' + 26) as usize
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mid_point = line.len() / 2;
            let c1 = &line[..mid_point];
            let c2 = &line[mid_point..];

            // mark characters in first half
            let mut seen = [false; 52];
            c1
                .bytes()
                .for_each(|b| {
                    seen[get_index(b)] = true;
                });

            // find duplicates in the second half
            c2
                .bytes()
                .map(|b| {
                    let index = get_index(b);
                    if seen[index] {
                        seen[index] = false;
                        (index + 1) as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}


fn part_2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut sum: u32 = 0;
    loop {
        let entries = match (lines.next(), lines.next(), lines.next()) {
            (Some(line1), Some(line2), Some(line3)) => [line1, line2, line3],
            _ => break,
        };

        let mut seen = [[false; 52]; 3]; // 1D would probably be faster
        for (i,e) in entries.iter().enumerate() {
            e.bytes().for_each(|b| {
                seen[i][get_index(b)] = true;
            });
        }

        for i in 0..52 {
            if seen[0][i] && seen[1][i] && seen[2][i] {
                sum += i as u32 + 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    pub fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 70);
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
