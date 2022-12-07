use std::fs::read_to_string;

fn get_priority(character: char) -> i32 {
    if character.is_uppercase() {
        return character as i32 - 38;
    }
    return character as i32 - 96;
}

fn parse_items(line: &str) -> i32 {
    let (first_part, second_part) = line.split_at(line.chars().count()/2);
    let mut priorities: Vec<i32> = Vec::new();

    for c in first_part.chars() {
        if second_part.contains(c) {
            let priority = get_priority(c);
            if !priorities.contains(&priority) {
                priorities.push(priority);
            }

        }
    }

    return priorities.iter().sum();
}

fn main() {
    let result: i32 = read_to_string("input.txt").expect("To have stuff")
        .lines()
        .map(parse_items)
        .sum();

    println!("{:?}", result);
}
