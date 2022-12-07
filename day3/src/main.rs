use std::fs::read_to_string;

trait Priority {
    type Err;
    fn into_priority(&self) -> Result<i32, Self::Err>;
}

impl Priority for char {
    type Err = ();
    fn into_priority(&self) -> Result<i32, Self::Err> {
        let ascii = *self as i32;

        match ascii {
            65..=90 => Ok(ascii - 38),
            97..=122 => Ok(ascii - 96),
            _ => Err(())
        }
    }

}

fn part1(line: &str) -> i32 {
    let (first_part, second_part) = line.split_at(line.chars().count()/2);


    let mut result: Vec<char> = first_part
        .chars()
        .filter(|&x| second_part.contains(x))
        .collect();

    result.sort();
    result.dedup();
    
    return result
        .iter()
        .map(|c| char::into_priority(&c).expect("to be a priority"))
        .sum();
}

fn main() {
    let result: i32 = read_to_string("input.txt").expect("To have stuff")
        .lines()
        .map(part1)
        .sum();

    println!("{:?}", result);
}
