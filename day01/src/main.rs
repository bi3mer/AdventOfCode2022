use std::fs;

fn part_1(input: &str) -> u32 {
    input  
        .split("\n\n")
        .map(|load| {
            load   
                .lines()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> u32 {
    let mut temp_vec = input  
        .split("\n\n")
        .map(|load| {
            load   
                .lines()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    temp_vec.sort_by(|a, b| b.cmp(a));
    temp_vec
        .iter()
        .take(3)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    pub fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    pub fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 45000);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
