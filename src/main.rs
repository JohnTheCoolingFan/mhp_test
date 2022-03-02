use rand::prelude::*;

const TRIES: u64 = 20_000_000;
const DOORS: u64 = 3;

fn main() {
    let mut wins_noswitch: u64 = 0;
    let mut losses_noswitch: u64 = 0;
    let mut wins_switch: u64 = 0;
    let mut losses_switch: u64 = 0;
    let mut prize: u64;
    let mut guess: u64;
    let mut unopened: u64;

    let mut rng = rand::thread_rng();

    println!("Total amount of tries: {}\nTotal amount of doors: {}\n", TRIES, DOORS);

    for i in 1..=TRIES {
        print!("Try #{}  ", i);

        prize = rng.gen::<u64>() % DOORS;
        guess = rng.gen::<u64>() % DOORS;
        unopened = rng.gen::<u64>() % DOORS;

        while unopened == guess {
            unopened = rng.gen::<u64>() % DOORS;
        }

        if prize == guess {
            wins_noswitch += 1;
            losses_switch += 1;
            println!("Switch: Loss, No switch: Win");
        } else {
            wins_switch += 1;
            losses_noswitch += 1;
            println!("Switch: Win,  No switch: Loss");
        }
    }

    let switch_winratio: f64 = wins_switch as f64 / TRIES as f64;
    let noswitch_winratio: f64 = wins_noswitch as f64 / TRIES as f64;

    println!("\n\nResults:\nSwitch: {} losses, {} wins, win ratio: {:.3}%", losses_switch, wins_switch, switch_winratio * 100.0);
    println!("No switch: {} losses, {} wins, win ratio: {:.3}%", losses_noswitch, wins_noswitch, noswitch_winratio * 100.0);

    println!("Winner is: {}", {
        if wins_switch > wins_noswitch {
            "Switch"
        } else if wins_switch < wins_noswitch {
            "No switch"
        } else {
            "stalemate"
        }
    })
}
