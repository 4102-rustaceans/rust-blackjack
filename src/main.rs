struct Card {
    suit: String,
    rank: String
}

struct Player {
    name: String,
    hand: Vec<Card>
}

fn main() {
    println!("Rust Sample Project");
    println!("by the ITCS 4102 Rustaceans: Richard Duane, Luke Frazier, and Seth Morris");
    println!("");
    println!("Demonstrating I/O, Control Structures, and Data Structures in Rust");
}

fn explain() {
    println("Please choose a topic to see it's unique features in Rust and how it was implemented in our program:");

}

fn play() {
    let mut deck = makeDeck();

    let mut dealer = Player{name: "Dealer", hand: Vec<Card>::new()};
    let mut player = Player{name: "Player", hand: Vec<Card>::new()};

    loop {
        // player's turn


        // daeler's turn
        if getHandValue(dealer.hand) < 17 {
            // don't draw another card, close enough
        } else {
            // draw a card
        }
    }
}

fn createDeck() -> Vec<Card> {
    let mut deck = Vec<Card>::new();
    let suits = Vec!["Clubs", "Diamonds", "Hearts", "Spades"];
    let ranks = Vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    for suit in suits {
        for rank in ranks {
            deck.push(Card{suit: suit, rank: rank});
        }
    }

    deck
}

fn getHandValue(hand: Vec<Card>) -> u32 {
    let mut aces = 0;

    let mut value = 0;

    for card in hand {
        match &card.rank {
            "Ace" => aces += 1,
            "Two" => value += 2,
            "Three" => value += 3,
            "Four" => value += 4,
            "Five" => value += 5,
            "Six" => value += 6,
            "Seven" => value += 7,
            "Eight" => value += 8,
            "Nine" => value += 9,
            _ => value += 10
        }
    }

    loop {
        value += if (value + 11) > 21 {
            1
        } else {
            11
        };

        aces -= 1;
        if aces == 0 {
            break;
        }
    }

    value
}
