use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("couldn't read file");
    let lines = contents.lines();
    let _numbers = lines.map(|line| {
        let line = line
            .replace("zero", "0")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");


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
