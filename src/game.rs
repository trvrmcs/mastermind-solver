use crate::{Code, Guess, Guesses, Response};
use std::fmt;
#[derive(Debug)]
pub struct GameState {
    secret: Code,
    pub guesses: Guesses,
}

impl GameState {
    pub fn new(secret: Code) -> Self {
        Self {
            secret,
            guesses: Guesses::new(),
        }
    }

    pub fn guess(&mut self, code: Code) {
        let guess = Guess {
            code,
            response: Response::check(self.secret, code),
        };
        self.guesses.push(guess);
    }
}

impl fmt::Display for GameState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "===========")?;
        writeln!(f, "GAME BOARD:")?;

        writeln!(f, "{}", self.guesses)?;

        let e = self.guesses.bits();
        writeln!(
            f,
            "Remaining possible codes: {}",
            self.guesses.possible_secrets().len()
        )?;
        writeln!(f, "Unknown information: {e} bits")?;

        Ok(())
    }
}
