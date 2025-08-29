/*

    Goal: build a mastermind solver using information theory.

    Optionally build a GUI for it.

    10 rows
    6 colors

    4 columns
*/

mod code;
mod colour;
mod game;
mod guess;
mod response;

use code::Code;
use colour::Colour;
use guess::{Guess, Guesses};
use response::Response;

use game::GameState;

fn main() {
    println!("Picking a secret code...");
    let secret: Code = rand::random();
    let mut state = GameState::new(secret);

    println!("{}", state);

    for i in 1..10 {
        let candidate = state.guesses.best_candidate();

        println!("  Optimal guess:  {} ", candidate.code);
        println!(
            "  Expected information of guess: {} bits\n",
            -candidate.entropy
        );

        state.guess(candidate.code);

        println!("{}", state);

        if state.guesses.possible_secrets().len() <= 1 {
            let our_guess = state.guesses.possible_secrets()[0];
            assert_eq!(our_guess, secret);
            println!("I believe the secret is {our_guess}");
            println!("The actual secret is    {secret}");
            println!("Solution found in {i} iterations");

            break;
        }
    }
}
