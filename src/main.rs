use std::io;

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

    let mut input = String::new();
    io:stdin().readline(&mut input).expect("Could not get input from stdin");

    input = input.trim();

    let mut choice = 0;

    match input.parse::<u32>() {
        Ok(i) => choice = i,
        Err(e) => println!("Error: {}", e)
    }

    match choice {
        1 => explain(),
        2 => play(),
        _ => println!("Invalid choic")
    }
}

fn explain() {
    println("Please choose a topic to see it's unique features in Rust and how it was implemented in our program:");

}

fn play() {
    let mut deck = makeDeck();

    let mut dealer = Player{name: "Dealer", hand: Vec<Card>::new()};
    let mut player = Player{name: "Player", hand: Vec<Card>::new()};
    let mut playerHold = false;
    let mut dealerHold = false;
    let mut playerBust = false;
    let mut dealerBust = false;

    loop {
        // player's turn
        if (!playerHold)
        println!("Current hand value: {}", getHandValue(player.hand));
        println!("If an Ace would cause you to go over 21, it will be automatically counted as a 1 instead of 11.");
        println!("Cards in hand: \n{}", printHand(player.hand));

        println!("What would you like to do?");
        println!("1) Draw a card");
        println!("2) Hold");

        let mut input = String::new();
        io:stdin().readline(&mut input).expect("Could not get input from stdin");

        input = input.trim();

        let mut choice = 0;

        match input.parse::<u32>() {
            Ok(i) => choice = i,
            Err(e) => println!("Error: {}", e)
        }

        match choice {
            1 => player.hand.push(deck.pop()),
            2 => playerHold = true,
            _ => println!("Invalid choice")
        }

        // daeler's turn
        if getHandValue(dealer.hand) < 17 {
            dealer.hand.push(deck.pop())
        }

        if (playerHold && dealerHold) || playerBust || dealerBust {
            break;
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

fn printHand(hand: Vec<Card>) -> String {
    let mut inHand: String = "";

    for card in hand {
        let mut curCard: String = format!("{} of {}", card.rank, card.suit);
        inHand = format!("{}{}\n", inHand, curCard);
    }

    inHand
}
