use std::fs::read_to_string;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper,
    Scissors
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

fn parse_round(line: &str) -> i32 {
    let (op_play, my_play) = line.split_once(" ")
        .expect("To have 2 plays");
    return parse_play(op_play, my_play);
}

fn parse_play(opponent: &str, mine: &str) -> i32 {
    let op_play = Play::from_str(opponent).expect("To be something") as i32;
    let my_play = Play::from_str(mine).expect("To be someting") as i32;

    if op_play == my_play {
        return my_play + 3; 
    }
    else if my_play -1 == op_play % 3 {
        return my_play + 6;
    }
    else {
        return my_play + 0;
    }
}

fn main() {
    let result: i32 = read_to_string("input.txt").expect("To have stuff")
        .lines()
        .map(parse_round).sum();

    println!("{:?}", result);
}
