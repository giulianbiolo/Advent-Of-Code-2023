fn load_input() -> Vec<String> {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    input.lines().map(|s| s.to_string()).collect()
}

fn part_one(s: &str) -> u32 {
    let first_digit: char = s.chars().find(|c| c.is_digit(10)).unwrap();
    let last_digit: char = s.chars().rev().find(|c| c.is_digit(10)).unwrap();
    let number: String = format!("{}{}", first_digit, last_digit);
    number.parse::<u32>().unwrap()
}

fn part_two(s: &str) -> u32 {
    let spelled: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    let mut found_numbers: Vec<u32> = vec![];
    for (idx, character) in s.chars().enumerate() {
        if character.is_digit(10) {
            found_numbers.push(character.to_string().parse::<u32>().unwrap());
        } else {
            for (idx2, word) in spelled.iter().enumerate() {
                if s[idx..].starts_with(word) {
                    found_numbers.push((idx2 + 1) as u32);
                }
            }
        }
    }
    let first_digit: char = found_numbers[0].to_string().chars().next().unwrap();
    let last_digit: char = found_numbers[found_numbers.len() - 1].to_string().chars().next().unwrap();
    let number: String = format!("{}{}", first_digit, last_digit);
    println!("String: {} | Number: {} | Found: {:?}", s, number, found_numbers);
    number.parse::<u32>().unwrap()
}

fn main() {
    println!("Hello, world!");
    let input: Vec<String> = load_input();
    println!("{:?}", input);
    //println!("Part One Result: {}", input.iter().map(|s| part_one(s)).sum::<u32>());
    println!("Part Two Result: {}", input.iter().map(|s| part_two(s)).sum::<u32>());

}
