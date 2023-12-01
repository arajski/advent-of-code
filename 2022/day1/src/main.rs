use std::fs::read_to_string;


fn sum_calories(calories: &str) -> i32 {
    let cals = calories.lines().map(|cal| {cal.parse::<i32>().unwrap_or(0)}).sum();
    return cals;
}

fn main() {
    let mut result: Vec<i32> = read_to_string("input.txt")
        .expect("To have content")
        .as_str()
    .split("\n\n")
    .map(sum_calories)
        .collect();

    result.sort();

    let part1: i32 = result.to_owned().into_iter().rev().take(1).sum();
    let part2: i32 = result.to_owned().into_iter().rev().take(3).sum();

    println!("{:?}", part1);
    println!("{:?}", part2);
}
