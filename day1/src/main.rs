fn get_calories() -> &'static str {
    return "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
}

fn sum_calories(calories: &'static str) -> i32 {
    let result = calories.lines().map(|cal| {cal.parse::<i32>().unwrap_or(0)}).sum();
    return result;
}

fn main() {
    let result = get_calories()
    .split("\n\n")
    .map(sum_calories)
        .max();

    match result {
        Some(cals) => println!("Most calories: {cals}"),
        None => println!("Oops!"),
    }
}
