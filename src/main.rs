extern crate rand;

use std::io;
use rand::{thread_rng, Rng};

struct Card {
    suit: String,
    rank: String
}

struct Player {
    name: String,
    hand: Vec<Card>
}

fn main() {
    // introduction text
    println!("Rust Sample Project");
    println!("by the ITCS 4102 Rustaceans: Richard Duane, Luke Frazier, and Seth Morris\n");
    println!("Demonstrating I/O, Control Structures, and Data Structures in Rust");

    // ask the user which function they want to run
    println!("What would you like to do?");
    println!("1) Explain I/O, CS, and DS");
    println!("2) Play the game!");

    // create a string to store user's input
    let mut input = String::new();

    // reads input from user and stores it in the input variable
    // read_line returns an Option, so expect unwraps the option to get the actual value
    // if that fails, it panics with the error message
    io::stdin().read_line(&mut input).expect("Could not get input from stdin");

    // trim removes whitespace, but returns &str, so we have to convert it back to a String
    input = input.trim().to_string();

    // create a variable to store the user's choice if it parses
    let mut choice = 0;

    // parse returns a Result, so while we could use expect again, we'll use a match to be more explicit
    // parsing as an unsigned 32-bit integer, although we could lower the size later
    match input.parse::<u32>() {
        // if the parse is successful, store the parsed value in choice
        Ok(i) => choice = i,
        // if the parse fails, print out the error
        Err(e) => println!("Error: {}", e)
    }

    // call our functions based on user input
    match choice {
        1 => explain(),
        2 => play(),
        // the user put in a u32, but it wasn't one we asked for
        // (a _ in a match statement is for the default choice)
        _ => println!("Invalid choice")
    }
}

fn explain() {
    println!("I/O: Input and output is handled through the std::io module. This is \
              implemented in two ways: explicitly, using the io::stdin() and read_line() \
              functions to get user input, as well as through the println!() macro. \
              Macros in rust are a form of 'metaprogramming', and are expanded to larger \
              blocks of code at compile time.\n");

    println!("Control Structures: This programs utilizes a few common control structures. \
              If statements work exactly like you'd expect, but in rust you can leave \
              off the parenthesis (in fact, the compiler recommends you do that for proper \
              Rust style). The loop statement works like a while(true), in that it doesn't \
              end until you break it. The for-in loop works like you'd expect, and the \
              match statement is similar to switch-case.\n");

    println!("Data Structures: Vectors are used frequently in those program. These are \
              lists/arrays that contain a single data type but are resizable and implement \
              functions like pop() and push(). Rust also has arrays, which contain a single \
              data type and have a fixed size, and lists, which are resizable and can \
              contain any combination of data types. There are also two custom structs \
              for handling Players and Cards.\n");

    println!("If you want more information, we've added detailed comments to the main.rs \
              source!");
}

fn play() {
    // create a 52-card deck using our createDeck function
    let mut deck = createDeck();

    // create a random number generator in this thread
    let mut rng = thread_rng();

    // shuffle the deck using the built in Fisher-Yates shuffle
    rng.shuffle(&mut deck);

    // create dealer and player from the Player struct
    let mut dealer = Player{name: "Dealer".to_string(), hand: Vec::new()};
    let mut player = Player{name: "Player".to_string(), hand: Vec::new()};

    // create booleans will we need to determine when to end the game
    let mut playerHold = false;
    let mut dealerHold = false;
    let mut playerBust = false;
    let mut dealerBust = false;

    // create a loop to start the game
    loop {
        if getHandValue(&player.hand) > 21 {
            playerHold = true;
            playerBust = true;
        }

        // if the player isn't holding, let them take their turn
        if !playerHold {
            // tell the player about their current state in the game
            // note that the values passed to the function include &,
            // this indicates that the functions will be immutably borrowing
            // the references, otherwise the functions would get ownershio
            // and we wouldn't be able to use them in this loop anymore
            println!("Current hand value: {}", getHandValue(&player.hand));
            println!("If an Ace would cause you to go over 21, it will be automatically counted as a 1 instead of 11.");
            println!("Cards in hand: \n{}", printHand(&player.hand));

            // using techniques discussed in main(), give user some choices and parse their input
            println!("What would you like to do?");
            println!("1) Draw a card");
            println!("2) Hold");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Could not get input from stdin");
            input = input.trim().to_string();
            let mut choice = 0;
            match input.parse::<u32>() {
                Ok(i) => choice = i,
                Err(e) => println!("Error: {}", e)
            }
            match choice {
                // take a card off the deck and put it in the player's hand
                // pop() returns an Option, so we need to use expect() again
                1 => player.hand.push(deck.pop().expect("No cards in deck")),
                2 => playerHold = true,
                _ => println!("Invalid choice")
            }
        }

        // daeler's turn
        if !dealerHold {
            // the dealer chickens out if they get to 17 or higher

            if getHandValue(&dealer.hand) > 21 {
                dealerBust = true;
            } else if getHandValue(&dealer.hand) < 17 {
                dealer.hand.push(deck.pop().expect("No cards in deck"))
            } else {
                dealerHold = true;
            }
        }

        // break the loop if the game is over
        if (playerHold && dealerHold) || playerBust || dealerBust {
            break;
        }
    }

    // let the player know who won
    if dealerBust && playerBust {
        println!("You both busted!")
    } else if playerBust {
        println!("You busted!");
    } else if dealerBust {
        println!("Dealer busted!");
    } else {
        let mut dealerScore = getHandValue(&dealer.hand);
        let mut playerScore = getHandValue(&player.hand);

        println!("You got {} and the dealer got {}.", playerScore, dealerScore);

        if playerScore == dealerScore {
            println!("You tied!");
        } else if playerScore > dealerScore {
            println!("You win!");
        } else {
            println!("You lost!");
        }
    }
}

// return types are specified using -> in the function declaration
fn createDeck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    let suits = vec!["Clubs", "Diamonds", "Hearts", "Spades"];
    let ranks = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    for suit in suits {
        for rank in &ranks {
            deck.push(Card{suit: suit.to_string(), rank: rank.to_string()});
        }
    }

    deck
}

fn getHandValue(hand: &Vec<Card>) -> u32 {
    let mut aces = 0;
    let mut value = 0;

    for card in hand {
        // literal strings are &str, not String, so we need coerce the Strings to &str
        match &card.rank as &str {
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

    // handle aces in typical blackjack fashion
    while aces > 0 {
        value += if (value + 11) > 21 {
            1
        } else {
            11
        };

        aces -= 1;
    }

    // variables at the end of a function with no ';' are returned
    value
}

fn printHand(hand: &Vec<Card>) -> String {
    let mut inHand: String = String::new();

    for card in hand {
        let mut curCard: String = format!("{} of {}", card.rank, card.suit);
        inHand = format!("{}{}\n", inHand, curCard);
    }

    inHand
}
