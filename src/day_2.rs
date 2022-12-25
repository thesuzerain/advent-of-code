// https://adventofcode.com/2022/day/2
// Rock Paper Scissors challenge.
// Given a list of inputs, return the total score of the described Rock Paper Scissors values.
// Inputs can be ABC, XYZ, and must resemble:
// A X
// B Y
// A Z
// ...


use super::*;

#[derive(PartialEq, Copy, Clone)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

enum RPSResult {
    Win,
    Draw,
    Loss
}

impl RPSChoice {

    // What you need to throw to beat this throw
    fn what_this_loses_to(&self) -> RPSChoice {
        match *self {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    // What this throw beats
    fn what_this_beats(&self) -> RPSChoice {
        match *self {
            RPSChoice::Scissors => RPSChoice::Paper,
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
        }
    }

    // Result of this throw against opp_choice
    fn play_against(&self, opp_choice: RPSChoice) -> RPSResult {
        let beats_opp = opp_choice.what_this_loses_to() ;        
        if *self == beats_opp {
            RPSResult::Win
        } else if *self == opp_choice { 
            RPSResult::Draw
        } else {
            RPSResult::Loss
        }
    }

}


pub fn run(part_2: bool) -> std::io::Result<()> {

    let mut score = 0;

    // Load data to buffer and iterate over lines
    let f = File::open("input/day2input.txt")?;
    let buf = BufReader::new(f);

    for line in buf.lines() {
        let line = line.expect("Could not read line.");
        let mut c = line.split_whitespace();

        // Opponent's choice depends on A,B,C in file.
        let opp_choice  = get_rps_choice(c.next().unwrap_or(" "));

        // Player choice depends on which Part of the challenge.
        let player_choice;
        if part_2 {
            // Part 2 - XYZ is the intended result, get player choice for that.
            player_choice = get_choice_for_desired_outcome(c.next().unwrap_or(" "), opp_choice);

        } else {
            // Part 1 - 'XYZ' values represent *your* choice.
            player_choice = get_rps_choice(c.next().unwrap_or(" "));
        }

        // Append score to running total
        score += score_round(player_choice, opp_choice);
    }

    println!("Result for day 2 = {score}");
    Ok(())
}


// Converts string slice to RPS throw
fn get_rps_choice(c: &str) -> RPSChoice {
    match c {
        "X" | "A" => RPSChoice::Rock,
        "Y" | "B" => RPSChoice::Paper,
        "Z"| "C" => RPSChoice::Scissors,
        _ => panic!("Line contained an invalid value {c}"),
    }
}

// Converts string slice to RPS throw based on intended result against a given opp_choice
fn get_choice_for_desired_outcome(o: &str, opp_choice: RPSChoice) -> RPSChoice {
    match o {
        "X" => opp_choice.what_this_beats(), // LOSS
        "Y" => opp_choice, // DRAW
        "Z" => opp_choice.what_this_loses_to(), // WIN
        _ => panic!("Line contained an invalid value {o}"),
    }
}

// Tallys the score for the round
// - the type of RPS throw contributes
// - the result contributes
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