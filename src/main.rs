enum Suits {
    Spade,
    Heart,
    Diamond,
    Club,
}

enum Numbers {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

struct Card {
    suits: Suits,
    number: Numbers,
}

impl Card {
    fn new(suits: Suits, number: Numbers) -> Self {
        Self { suits, number }
    }
}

fn determine_type<'a>(cards: Vec<Card>) -> &'a str {
    let mut numbers_count: Vec<usize> = vec![0;13];
    let mut suit_count: Vec<usize> = vec![0;4];

    for card in cards {
        suit_count[card.suits as usize] += 1;
        numbers_count[card.number as usize] += 1;
    }

    if is_all_same(&suit_count, 5)
        && numbers_count[8] == 1
        && numbers_count[9] == 1
        && numbers_count[10] == 1
        && numbers_count[11] == 1
        && numbers_count[12] == 1
    {
        "Royal Flush"
    } else if is_all_same(&suit_count, 5) && is_continuous(&numbers_count) {
        "Straight Flush"
    } else if is_all_same(&numbers_count, 4) {
        "Four of a Kind"
    } else if is_pair(&numbers_count, 3, 2) {
        "Full House"
    } else if is_all_same(&suit_count, 5) {
        "Flush"
    } else if is_continuous(&numbers_count) {
        "Straight"
    } else if is_all_same(&numbers_count, 3) {
        "Three of a Kind"
    } else if is_pair(&numbers_count, 2, 2) {
        "Two Pair"
    } else if is_all_same(&numbers_count, 2) {
        "One Pair"
    } else {
        "High Card"
    }
}

fn is_continuous(count: &[usize]) -> bool {
    let mut seq = 0;

    for &num in count {
        if num > 0 {
            seq += 1;
            if seq == 5 {
                return true;
            }
        } else {
            seq = 0;
        }
    }
    false
}

fn is_all_same(count: &[usize], times: usize) -> bool {
    count.iter().any(|&num| num == times)
}

fn is_pair(count: &Vec<usize>, times1: usize, times2: usize) -> bool {
    let mut found_first = false;
    let mut found_second = false;

    for &num in count {
        if num == times1 && !found_first {
            found_first = true;
        } else if num == times2 {
            found_second = true;
        }
    }

    found_first && found_second
}

fn main() {
    let mut cards = vec![
        Card::new(Suits::Spade, Numbers::Ten),
        Card::new(Suits::Spade, Numbers::Jack),
        Card::new(Suits::Spade, Numbers::Queen),
        Card::new(Suits::Spade, Numbers::King),
        Card::new(Suits::Spade, Numbers::Ace),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Heart, Numbers::Three),
        Card::new(Suits::Heart, Numbers::Four),
        Card::new(Suits::Heart, Numbers::Five),
        Card::new(Suits::Heart, Numbers::Six),
        Card::new(Suits::Heart, Numbers::Seven),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Spade, Numbers::Eight),
        Card::new(Suits::Heart, Numbers::Eight),
        Card::new(Suits::Diamond, Numbers::Eight),
        Card::new(Suits::Spade, Numbers::Eight),
        Card::new(Suits::Diamond, Numbers::Two),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Spade, Numbers::Six),
        Card::new(Suits::Heart, Numbers::Six),
        Card::new(Suits::Club, Numbers::Six),
        Card::new(Suits::Diamond, Numbers::Three),
        Card::new(Suits::Club, Numbers::Three),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Club, Numbers::King),
        Card::new(Suits::Club, Numbers::Ten),
        Card::new(Suits::Club, Numbers::Eight),
        Card::new(Suits::Club, Numbers::Five),
        Card::new(Suits::Club, Numbers::Two),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Spade, Numbers::Five),
        Card::new(Suits::Diamond, Numbers::Six),
        Card::new(Suits::Diamond, Numbers::Seven),
        Card::new(Suits::Heart, Numbers::Eight),
        Card::new(Suits::Club, Numbers::Nine),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Spade, Numbers::Nine),
        Card::new(Suits::Heart, Numbers::Nine),
        Card::new(Suits::Diamond, Numbers::Nine),
        Card::new(Suits::Spade, Numbers::Queen),
        Card::new(Suits::Club, Numbers::Seven),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Heart, Numbers::Queen),
        Card::new(Suits::Spade, Numbers::Queen),
        Card::new(Suits::Heart, Numbers::Two),
        Card::new(Suits::Diamond, Numbers::Two),
        Card::new(Suits::Club, Numbers::Seven),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Diamond, Numbers::Three),
        Card::new(Suits::Club, Numbers::Three),
        Card::new(Suits::Spade, Numbers::Jack),
        Card::new(Suits::Heart, Numbers::Four),
        Card::new(Suits::Diamond, Numbers::Ace),
    ];
    println!("{}", determine_type(cards));

    cards = vec![
        Card::new(Suits::Spade, Numbers::Ace),
        Card::new(Suits::Heart, Numbers::Jack),
        Card::new(Suits::Diamond, Numbers::Five),
        Card::new(Suits::Club, Numbers::Eight),
        Card::new(Suits::Diamond, Numbers::Two),
    ];
    println!("{}", determine_type(cards));
}
