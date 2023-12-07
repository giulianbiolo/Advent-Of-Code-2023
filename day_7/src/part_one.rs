#[derive(Debug, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => true,
            (HandType::FourOfAKind, HandType::FourOfAKind) => true,
            (HandType::FullHouse, HandType::FullHouse) => true,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => true,
            (HandType::TwoPair, HandType::TwoPair) => true,
            (HandType::OnePair, HandType::OnePair) => true,
            (HandType::HighCard, HandType::HighCard) => true,
            _ => false
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => Some(std::cmp::Ordering::Equal),
            (HandType::FiveOfAKind, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::FiveOfAKind) => Some(std::cmp::Ordering::Less),
            (HandType::FourOfAKind, HandType::FourOfAKind) => Some(std::cmp::Ordering::Equal),
            (HandType::FourOfAKind, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::FourOfAKind) => Some(std::cmp::Ordering::Less),
            (HandType::FullHouse, HandType::FullHouse) => Some(std::cmp::Ordering::Equal),
            (HandType::FullHouse, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::FullHouse) => Some(std::cmp::Ordering::Less),
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => Some(std::cmp::Ordering::Equal),
            (HandType::ThreeOfAKind, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::ThreeOfAKind) => Some(std::cmp::Ordering::Less),
            (HandType::TwoPair, HandType::TwoPair) => Some(std::cmp::Ordering::Equal),
            (HandType::TwoPair, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::TwoPair) => Some(std::cmp::Ordering::Less),
            (HandType::OnePair, HandType::OnePair) => Some(std::cmp::Ordering::Equal),
            (HandType::OnePair, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::OnePair) => Some(std::cmp::Ordering::Less),
            (HandType::HighCard, HandType::HighCard) => Some(std::cmp::Ordering::Equal),
            (HandType::HighCard, _) => Some(std::cmp::Ordering::Greater),
            (_, HandType::HighCard) => Some(std::cmp::Ordering::Less),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    value: u32,
    hand_type: HandType
}

impl Hand {
    fn new(cards: String, value: u32) -> Hand {
        let hand_type: HandType = Hand::get_hand_type((cards.clone(), value));
        Hand { cards, value, hand_type }
    }
    fn get_hand_type(hand: (String, u32)) -> HandType {
        let mut counts: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for card in hand.0.chars() {
            let count: &mut u32 = counts.entry(card as u32).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<(&u32, &u32)> = counts.iter().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        let mut counts: Vec<u32> = counts.iter().map(|x| *x.1).collect();
        counts.sort_by(|a, b| b.cmp(a));
        if counts[0] == 5 {
            HandType::FiveOfAKind
        } else if counts[0] == 4 {
            HandType::FourOfAKind
        } else if counts[0] == 3 && counts[1] == 2 {
            HandType::FullHouse
        } else if counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if counts[0] == 2 && counts[1] == 2 {
            HandType::TwoPair
        } else if counts[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type == other.hand_type {
            let vals: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
            let self_cards: Vec<u32> = self.cards.chars().map(|x| vals.iter().position(|&y| y == x).unwrap() as u32).collect();
            let other_cards: Vec<u32> = other.cards.chars().map(|x| vals.iter().position(|&y| y == x).unwrap() as u32).collect();
            for i in 0..self_cards.len() {
                if self_cards[i] > other_cards[i] {
                    return Some(std::cmp::Ordering::Greater)
                } else if self_cards[i] < other_cards[i] {
                    return Some(std::cmp::Ordering::Less)
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            self.hand_type.partial_cmp(&other.hand_type)
        }
    }
}


fn load_data(filename: &str) -> Result<Vec<Hand>, std::io::Error> {
    let mut data: Vec<Hand> = Vec::new();
    let file: String = std::fs::read_to_string(filename)?;
    for line in file.lines() {
        let mut fields: std::str::Split<'_, char> = line.split(' ');
        let cards: String = fields.next().unwrap().to_string();
        let value: u32 = fields.next().unwrap().parse::<u32>().unwrap();
        data.push(Hand::new(cards, value));
    }
    Ok(data)
}

fn part_one(data: &Vec<Hand>) -> u32 {
    let mut data: Vec<Hand> = data.clone();
    data.sort_by(|a, b| b.partial_cmp(a).unwrap());
    data.iter().rev().enumerate().fold(0, |acc, (i, x)| acc + (i + 1) as u32 * x.value)
}

fn main() {
    let data: Vec<Hand> = load_data("input.txt").unwrap();
    println!("Part One: {}", part_one(&data));
}
