use std::fs::read_to_string;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper,
    Scissors
}

trait Strategy: Sized {
    type Err;
    fn adjust(s: &str, play: i32) -> Result<i32, Self::Err>;
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

impl Strategy for Play {
    type Err = ();

    fn adjust(input: &str, op_num: i32) -> Result<i32, Self::Err> {
        match input {
            "X" => Ok(((op_num +1) % 3) + 1),
            "Y" => Ok(op_num),
            "Z" => Ok((op_num % 3) + 1),
            _ => Err(())
        }
    }
}

fn parse_round1(line: &str) -> i32 {
    let (op_play, my_play) = line.split_once(" ")
        .expect("To have 2 plays");

    let op_num = Play::from_str(op_play).expect("To be something") as i32;
    let my_num = Play::from_str(my_play).expect("To be someting") as i32;

    return parse_play(op_num, my_num);
}

fn parse_round2(line: &str) -> i32 {
    let (op_play, my_play) = line.split_once(" ")
        .expect("To have 2 plays");

    let op_num = Play::from_str(op_play).expect("To be something") as i32;
    let my_num = Play::adjust(my_play, op_num).expect("To be someting");

    return parse_play(op_num, my_num);
}

fn parse_play(op_play: i32, my_play: i32) -> i32 {
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
        .map(parse_round2).sum();

    println!("{:?}", result);
}
