use std::io::Error;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct ScratchCard {
    card_number: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl ScratchCard { fn winning(&self) -> u32 { self.numbers.iter().filter(|x| self.winning_numbers.contains(x)).count() as u32 } }


fn load_data(filename: &str) -> Result<Vec<ScratchCard>, Error> {
    let mut scratchcards: Vec<ScratchCard> = Vec::new();
    let contents: String = std::fs::read_to_string(filename)?;
    for (nline, line) in contents.lines().enumerate() {
        let line_numbers: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
        let (winning_numbers, numbers) = (line_numbers[0].split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>(), line_numbers[1].split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>());
        scratchcards.push(ScratchCard { card_number: nline as u32 + 1, winning_numbers: winning_numbers, numbers: numbers });
    }
    Ok(scratchcards)
}

fn main() -> Result<(), Error> {
    let scratchcards: Vec<ScratchCard> = load_data("input.txt")?;
    let part_one_total: u32 = scratchcards.iter().map(|x| 2_u32.pow(x.winning() - 1)).sum();
    println!("Part One: {}", part_one_total);

    let mut occurrencies: HashMap<u32, u32> = HashMap::new();
    for card in scratchcards.iter() { *occurrencies.entry(card.card_number).or_insert(0) += 1; }
    for card in scratchcards {
        let winning: usize = card.winning() as usize;
        let card_number: u32 = card.card_number;
        let card_occurrencies: u32 = *occurrencies.get(&card_number).unwrap();
        for j in 0..winning { *occurrencies.entry(card_number + j as u32 + 1).or_insert(0) += card_occurrencies; }
    }
    let part_two_total: u32 = occurrencies.values().sum();
    println!("Part Two: {}", part_two_total);
    Ok(())
}
