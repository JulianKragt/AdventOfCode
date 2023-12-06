use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("couldn't read file");
    let lines = contents.lines();
    let _numbers = lines.map(|line| {
        let numbers: Vec<char> = line.chars().filter(|char| {
            char.is_numeric()
        }).collect();

        let mut string = String::new();

        if let Some(num) = numbers.first() {
            string.push(*num);
        }
        if let Some(num) = numbers.last() {
            string.push(*num);
        }
        string.parse::<i32>().unwrap()
    });

    let sum: i32 = _numbers.sum();

    println!(
        "{:?}",
        sum
    );

}
