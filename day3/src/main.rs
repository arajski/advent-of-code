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

fn part2(parta: &str, partb: &str, partc: &str)-> i32 {
    let mut result: Vec<char> = parta
        .chars()
        .filter(|&x| partb.contains(x) && partc.contains(x))
        .collect();

    result.sort();
    result.dedup();

    return result.iter().map(|c| char::into_priority(&c).expect("to be a priority")).sum()
}

fn main() {
    let part1_result: i32 = read_to_string("input.txt").expect("To have stuff")
        .lines()
        .map(part1)
        .sum();

    let part2_input = read_to_string("input.txt").expect("To have stuff");
    let mut lines = part2_input.lines();

    let mut part2_sum = 0;
    while let(
    Some(parta), 
    Some(partb), 
    Some(partc)) = (
        lines.next(), 
        lines.next(), 
        lines.next()) {
        part2_sum = part2_sum + part2(parta, partb, partc);
            
    }

    println!("{:?}", part1_result);
    println!("{:?}", part2_sum);
}
