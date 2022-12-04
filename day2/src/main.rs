use std::fs::read_to_string;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Play {
    Rock = 1,
    Paper,
    Scissors
}

fn parse_round(line: &str) -> i32 {
    let (op_play, my_play) = line.split_once(" ")
        .expect("To have 2 plays");
    return parse_play(op_play, my_play);
}

fn parse_play(opponent: &str, mine: &str) -> i32 {
    let rules = [Play::Rock, Play::Paper, Play::Scissors];
    let op_play = Play::from_str(opponent).expect("To be something");
    let my_play = Play::from_str(mine).expect("To be someting");

    let op_idx = rules.iter().position(|x| x == &op_play).expect("To be an index");
    let my_idx = rules.iter().position(|x| x == &my_play).expect("To be an index");

    if op_idx == my_idx {
        return my_play as i32 + 3; 
    }
    else if my_idx == ((op_idx+1) % 3) {
        return my_play as i32  + 6;
    }
    else {
        return my_play as i32 + 0;
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            "A"|"X" => Ok(Play::Rock),
            "B"|"Y" => Ok(Play::Paper),
            "C"|"Z" => Ok(Play::Scissors),
            _ => Err(())
        }
    }
}

fn main() {
    let result: i32 = read_to_string("input.txt").expect("To have stuff")
        .lines()
        .map(parse_round).sum();

    println!("{:?}", result);
}
