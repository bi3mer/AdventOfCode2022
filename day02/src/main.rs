use std::fs;



#[derive(Copy, Clone, Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    pub fn score(&self) -> u32{
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

pub fn rps_from_string(element: &str) -> RPS {
    match element {
        "A" => RPS::Rock,
        "X" => RPS::Rock,
        "B" => RPS::Paper,
        "Y" => RPS::Paper,
        "C" => RPS::Scissors,
        "Z" => RPS::Scissors,
        _ => {
            panic!("Unhandled RPS type: {}", element);
        }
    }
}

pub fn get_desired_move(other_action: RPS, result: MatchResult) -> RPS {
    match result {
        MatchResult::Win => match other_action {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
        MatchResult::Loss => match other_action {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        MatchResult::Draw => match other_action {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
        },
    }
}

pub enum MatchResult {
    Win,
    Loss,
    Draw
}

impl MatchResult {
    pub fn score(&self) -> u32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        }
    }
}

pub fn get_match_result(bot: RPS, player: RPS) -> MatchResult{
    match (bot, player) {
        (RPS::Rock, RPS::Rock) => MatchResult::Draw,
        (RPS::Rock, RPS::Paper) => MatchResult::Win,
        (RPS::Rock, RPS::Scissors) => MatchResult::Loss,
        (RPS::Paper, RPS::Rock) => MatchResult::Loss,
        (RPS::Paper, RPS::Paper) => MatchResult::Draw,
        (RPS::Paper, RPS::Scissors) => MatchResult::Win,
        (RPS::Scissors, RPS::Rock) => MatchResult::Win,
        (RPS::Scissors, RPS::Paper) => MatchResult::Loss,
        (RPS::Scissors, RPS::Scissors) => MatchResult::Draw,
    }
}

pub fn result_from_str(element: &str) -> MatchResult {
    match element {
        "X" => MatchResult::Loss,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => { panic!("Unhandled result_from_str(\"{}\").", element) }
    }
}

/*
 A and X for Rock with a score of 1
 B and Y for Paper with a score of 2
 C and Z for Scissors with a score of 3

        Lose  Draw  Win
 Score     0     3    6
 */
fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let actions: Vec<RPS> = line
                .split(" ")
                .map(rps_from_string)
                .collect();

            actions[1].score() + get_match_result(actions[0], actions[1]).score()
        })
        .sum()
}

/*
 X means I need to lose
 Y means I need to draw
 Z means I need to win
 */
fn part_2(input: &str) -> u32 {
    input
    .lines()
    .map(|line| {
        let actions: Vec<&str> = line.split(" ").collect();
        let elf_action = rps_from_string(actions[0]);

        let desired_result = result_from_str(actions[1]);
        let bot_action = get_desired_move(elf_action, desired_result);

        bot_action.score() + get_match_result(elf_action, bot_action).score()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    pub fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    pub fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 12);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
