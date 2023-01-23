// https://adventofcode.com/2022/day/2
// Day 2: Rock Paper Scissors (RPS)
// Part 1: Given a list of inputs, representing the Rock Paper Scissors choices for you and an opponent,
// sum the total number of points earned for RPS games played (weighted by the RPS choice you made).
// Part 2: Given a list of inputs, representing the Rock Paper Scissors choices for you and the intended result,
// sum the total number of points earned for RPS games played (weighted by the RPS choice you made).

// Inputs can be ABC or XYZ,and must resemble:
// A X 
// B Y
// A Z
// ...
// etc.


use super::*;

// Rock Paper Scissors choice
#[derive(PartialEq, Copy, Clone)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

// Result of a Rock Paper Scissors game
enum RPSResult {
    Win,
    Draw,
    Loss
}

// Run challenge.
// Main entry point to day 2 challenge.
pub fn run(part_2: bool) -> Result<(),Box<dyn error::Error>> {

    let mut score = 0;

    // Load data to buffer and iterate over lines
    let f = File::open("input/day2input.txt")?;
    let buf = BufReader::new(f);

    for line in buf.lines() {
        let line = line.expect("Could not read line.");
        let mut c = line.split_whitespace();

        // Opponent's choice depends on A,B,C in file.
        let opp_choice  = get_rps_choice(c.next().unwrap_or(" "));

        // Player choice depends on 'XYZ', which is semantically different for part_1 or part_2.
        let player_choice;
        if part_2 {
            // Part 2 - XYZ is the intended result (X/Y/Z => LOSS/DRAW/WIN), get player choice such that that result occurrs.
            player_choice = get_choice_for_desired_outcome(c.next().unwrap_or(" "), opp_choice);

        } else {
            // Part 1 - 'XYZ' values represent player choice (X/Y/Z => ROCK/PAPER/SCISSORS).
            player_choice = get_rps_choice(c.next().unwrap_or(" "));
        }

        // Score round and append to running total
        score += score_round(player_choice, opp_choice);
    }

    let part = if part_2 {2} else {1};
    println!("Result for day 2-{part} = {score}");
    Ok(())
}

impl RPSChoice {

    // What this RPS choice loses to
    fn loses_to(&self) -> RPSChoice {
        match *self {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    // What this RPS choice beats
    fn beats(&self) -> RPSChoice {
        match *self {
            RPSChoice::Scissors => RPSChoice::Paper,
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
        }
    }

    // Result of a game played by this choice against opp_choice as an RPSResult
    fn play_against(&self, opp_choice: RPSChoice) -> RPSResult {
        let beats_opp = opp_choice.loses_to() ;        
        if *self == beats_opp {
            RPSResult::Win
        } else if *self == opp_choice { 
            RPSResult::Draw
        } else {
            RPSResult::Loss
        }
    }

}

// Converts string slice 'o' to RPS choice
// X/A - Rock
// Y/B - Paper
// Z/C - Scissors
fn get_rps_choice(c: &str) -> RPSChoice {
    match c {
        "X" | "A" => RPSChoice::Rock,
        "Y" | "B" => RPSChoice::Paper,
        "Z"| "C" => RPSChoice::Scissors,
        _ => panic!("Line contained an invalid value {c}"),
    }
}

// Converts string slice 'o' to RPS choice, where string slice 'o' represents the intended result. 
// X - the choice that 'opp_choice' can beat
// Y - the choice that 'opp_choice'
// Z - the choice that 'opp_choice' loses to

fn get_choice_for_desired_outcome(o: &str, opp_choice: RPSChoice) -> RPSChoice {
    match o {
        "X" => opp_choice.beats(), // intended LOSS
        "Y" => opp_choice, // intended DRAW
        "Z" => opp_choice.loses_to(), // intended WIN
        _ => panic!("Line contained an invalid value {o}"),
    }
}

// Tallys the score for the round, based on:
// - the choice made by the player (R/P/S given scores of 1/2/3)
// - the game result based on the choices made (Win/Draw/Loss given scores of 6/3/0)
fn score_round(user_choice: RPSChoice, opp_choice: RPSChoice) -> i32 {
    let mut base_score = match user_choice {
        RPSChoice::Rock => 1,
        RPSChoice::Paper => 2,
        RPSChoice::Scissors => 3,
    };

    base_score += match user_choice.play_against(opp_choice) {
        RPSResult::Win => 6,
        RPSResult::Draw => 3,
        RPSResult::Loss => 0,
    };

    base_score
}