use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("couldn't read file");
    let lines = contents.lines();
    let _numbers = lines.map(|line| {
        let numbers: Vec<char> = line.chars().filter(|char| {
            char.is_numeric()
        }).collect();

        String::new().push(numbers.first())).push(numbers.last())
    });

    println!(
        "{:?}",
        _numbers.clone().next()
    );

}
