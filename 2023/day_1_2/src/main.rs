use std::fs;
use std::io::Lines;
use std::path::Iter;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("couldn't read file");
    let lines = contents.lines();
    let _numbers = lines.map(|line| {
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "fo4r")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne");

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
