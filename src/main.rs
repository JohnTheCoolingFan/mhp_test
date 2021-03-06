use rand::prelude::*;
use std::cmp::Ordering;

const TRIES: u64 = 20_000_000; // Amount of tries
const DOORS: u64 = 3;          // Amoutn of doors to choose from

fn main() {
    // Global score-keeping
    let mut wins_noswitch: u64 = 0;   // Wins with "no switching" strategy
    let mut losses_noswitch: u64 = 0; // Losses with "no switching" strategy
    let mut wins_switch: u64 = 0;     // Wins with "switching" strategy
    let mut losses_switch: u64 = 0;   // Losses with "switching" strategy

    // Loop variables
    let mut prize: u64;    // Number of a door with a prize
    let mut guess: u64;    // "Player"'s guess
    let mut unopened: u64; // Number of a door "Monty Hall" left unopened (aside from player's guess)

    // Initialize random number generator
    let mut rng = rdrand::RdRand::new().unwrap();

    println!("Total amount of tries: {}\nTotal amount of doors: {}\n", TRIES, DOORS);

    for i in 1..=TRIES {
        print!("Try #{}  ", i);

        // Select random doors
        prize = rng.gen::<u64>() % DOORS;
        guess = rng.gen::<u64>() % DOORS;
        unopened = rng.gen::<u64>() % DOORS;

        // Make sure that there are two unopened doors and that door wit ha prize wasn't opened
        // Second check doesn't affect probability for some reason...
        while unopened == guess || (unopened != prize && guess != prize) {
            unopened = rng.gen::<u64>() % DOORS;
        }

        if prize == guess {            // Switch loss, No switch win
            wins_noswitch += 1;
            losses_switch += 1;
            println!("Switch: Loss, No switch: Win");
        } else if prize == unopened {  // No switch loss, Switch win
            wins_switch += 1;
            losses_noswitch += 1;
            println!("Switch: Win,  No switch: Loss");
        } else {                       // Both lose, Monty Hall opened a door with a prize, someone gets fired
            losses_switch += 1;
            losses_noswitch += 1;
            println!("Switch: Loss, No switch: Loss");
        }
    }

    // Calculate win ratios
    let switch_winratio: f64 = wins_switch as f64 / TRIES as f64;
    let noswitch_winratio: f64 = wins_noswitch as f64 / TRIES as f64;

    // Print results
    println!("\n\nResults:\nSwitch: {} losses, {} wins, win ratio: {:.3}%", losses_switch, wins_switch, switch_winratio * 100.0);
    println!("No switch: {} losses, {} wins, win ratio: {:.3}%", losses_noswitch, wins_noswitch, noswitch_winratio * 100.0);

    println!("Winner is: {}", {
        match wins_switch.cmp(&wins_noswitch) {
            Ordering::Greater => "Switch",
            Ordering::Less => "No switch",
            Ordering::Equal => "Stalemate???"
        }
    })
}
