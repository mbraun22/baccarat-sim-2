use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::io;

fn main() {
    let deck_per_shoe: u8 = 8;
    let mut player_wins: u32 = 0;
    let mut dealer_wins: u32 = 0;
    let mut tie: u32 = 0;
    let mut user_input = String::new();
    println!("How many shoes would you like to simulate?");
    io::stdin().read_line(&mut user_input).expect("Failed");
    let mut num_shoes = user_input.trim().parse().expect("Not Valid");
    for i in 0..num_shoes {
        let mut shoe: Shoe = create_shoe(deck_per_shoe);
        let cut_lower: usize = ((shoe.cards.len() as f32) * 0.7) as usize;
        let cut_upper: usize = ((shoe.cards.len() as f32) * 0.9) as usize;
        let cut_card: usize = thread_rng().gen_range(cut_lower..cut_upper);
        while shoe.cards.len() > ((deck_per_shoe as usize * 52) as usize - cut_card) {
            let mut results = play_hand(shoe);
            shoe = results.0;
            let player_hand = results.1;
            let dealer_hand = results.2;
            match check_outcome(&player_hand, &dealer_hand) {
                Outcome::Tie => tie += 1,
                Outcome::Player => player_wins += 1,
                Outcome::Dealer => dealer_wins += 1,
            }
        }
        println!("Shoes Remaining: {}", num_shoes - i);
    }
    println!(
        "Player Wins: {} Dealer Wins: {} Ties: {}",
        player_wins, dealer_wins, tie
    );
}

#[derive(Clone, Debug)]
struct Card {
    value: u8,
    suit: Suits,
}
#[derive(Debug)]
struct Shoe {
    cards: Vec<Card>,
}
#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn shuffle(&mut self) {
        &self.cards.shuffle(&mut thread_rng());
    }
}
#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Clone, Debug)]
enum Suits {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

enum Outcome {
    Player,
    Dealer,
    Tie,
}

fn card_value(card: &Card) -> u8 {
    match &card.value {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        10 => 10,
        11 => 10,
        12 => 10,
        13 => 10,
        _ => panic!(),
    }
}

fn hand_value(hand: &Hand) -> u8 {
    let mut hand_value: u32 = 0;
    for cards in hand.cards.iter() {
        hand_value += card_value(cards) as u32;
    }
    (hand_value % 10) as u8
}

fn create_shoe(num_decks: u8) -> Shoe {
    let mut shoe: Shoe = Shoe { cards: Vec::new() };

    for _ in 0..num_decks {
        let mut deck = create_deck();
        shoe.cards.append(&mut deck.cards);
    }
    shoe
}

fn create_deck() -> Deck {
    let mut deck = Deck { cards: Vec::new() };
    for suit in [Suits::Spades, Suits::Clubs, Suits::Hearts, Suits::Diamonds] {
        for i in 1..14 {
            deck.cards.push(Card {
                value: i,
                suit: suit.clone(),
            });
        }
    }
    deck.shuffle();
    deck
}

fn play_hand(mut shoe: Shoe) -> (Shoe, Hand, Hand) {
    let mut player_hand = Hand { cards: Vec::new() };
    let mut dealer_hand = Hand { cards: Vec::new() };
    let mut shoe_index: u16 = 3;
    player_hand.cards.push(shoe.cards[0].clone());
    dealer_hand.cards.push(shoe.cards[1].clone());
    player_hand.cards.push(shoe.cards[2].clone());
    dealer_hand.cards.push(shoe.cards[3].clone());

    if hand_value(&player_hand) > 7 || hand_value(&dealer_hand) > 7 {
        shoe.cards.drain(0..(shoe_index as usize));
        return (shoe, player_hand, dealer_hand);
    }

    while hand_value(&player_hand) < 6 {
        player_hand
            .cards
            .push((shoe.cards[(shoe_index + 1) as usize]).clone());
        shoe_index += 1;
    }

    while hand_value(&dealer_hand) < 6 {
        dealer_hand
            .cards
            .push((shoe.cards[(shoe_index + 1) as usize]).clone());
        shoe_index += 1;
    }

    shoe.cards.drain(0..(shoe_index as usize));
    (shoe, player_hand, dealer_hand)
}

fn check_outcome(player_hand: &Hand, dealer_hand: &Hand) -> Outcome {
    if hand_value(player_hand) == hand_value(dealer_hand) {
        return Outcome::Tie;
    } else if hand_value(player_hand) > hand_value(dealer_hand) {
        return Outcome::Player;
    } else {
        return Outcome::Dealer;
    }
}
