use std::fmt;

use crate::code::Code;
use crate::response::Response;

#[derive(Debug, Clone)]
pub struct Guess {
    pub code: Code,
    pub response: Response,
}

impl Guess {
    pub fn could_match(&self, secret: Code) -> bool {
        Response::check(secret, self.code) == self.response
    }
}

#[derive(Clone)]
pub struct CodeAndEntropy {
    pub code: Code,
    pub entropy: f32,
}

#[derive(Debug, Clone)]
pub struct Guesses {
    guesses: Vec<Guess>,
    possible_secrets: Vec<Code>,
}

impl Guesses {
    pub fn possible_secrets(&self) -> &Vec<Code> {
        &self.possible_secrets
    }
    pub fn new() -> Self {
        let guesses = vec![];

        // initially any code could match
        let possible_secrets = Code::all_codes();

        Self {
            guesses,
            possible_secrets,
        }
    }

    pub fn push(&mut self, guess: Guess) {
        //add to guesses, filter possible secrets.

        self.possible_secrets
            .retain(|&code| guess.could_match(code));

        self.guesses.push(guess);
    }

    pub fn entropy_of_guess(&self, candidate: Code) -> f32 {
        /*
        For THIS guess, calculate the number of codes that will still be viable if the Code Master gives response i in return.

        Do this for all possible responses.

        Finally, calculate the entropy of THIS guess...
        */

        let n = self.possible_secrets.len();

        let mut sum = 0.0;

        for response in Response::all_responses() {
            // how many codes would be viable if this response is returned?

            let count = self
                .possible_secrets
                .iter()
                // This code could produce the results on all previous rows AND give `response` for `candidate`
                .filter(|&code| response.could_match(*code, candidate))
                .count();

            // probability of getting THIS response
            let p = count as f32 / n as f32;

            sum += if p == 0.0 { 0.0 } else { p * (1.0 / p).log2() };
        }

        -sum
    }

    pub fn codes_by_entropy(&self) -> Vec<CodeAndEntropy> {
        let mut pairs: Vec<CodeAndEntropy> = Code::all_codes()
            .iter()
            .map(|candidate| CodeAndEntropy {
                code: *candidate,
                entropy: self.entropy_of_guess(*candidate),
            })
            .collect();

        pairs.sort_by(|a, b| a.entropy.total_cmp(&b.entropy));
        pairs
    }

    pub fn best_candidate(&self) -> CodeAndEntropy {
        // Ideally we'd randomly choose ANY code from the best-ranking ones
        self.codes_by_entropy()[0].clone()
    }

    pub fn bits(&self) -> f32 {
        /*
           Equivalent to -H, where
           H = entropy = \Sigma {p(x) log (p(x))}

           or

           E [-log(p(x))]

        */
        let l = self.possible_secrets.len();

        (l as f32).log2() / (2f32).log2()
    }
}

impl fmt::Display for Guesses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for guess in &self.guesses {
            writeln!(
                f,
                "  {} \t[Right: {} | Wrong: {}]",
                guess.code, guess.response.right, guess.response.wrong
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::colour::*;
    pub const BLACK: Colour = Colour::Black;
    const BLUE: Colour = Colour::Blue;

    const RED: Colour = Colour::Red;
    const WHITE: Colour = Colour::White;
    const YELLOW: Colour = Colour::Yellow;

    #[test]
    fn test_01() {
        let guess = Guess {
            code: Code(WHITE, WHITE, WHITE, WHITE),
            response: Response { right: 1, wrong: 0 },
        };

        assert!(guess.could_match(Code(RED, RED, RED, WHITE)));
        assert!(guess.could_match(Code(RED, RED, WHITE, RED)));
        assert!(guess.could_match(Code(WHITE, BLUE, YELLOW, BLACK)));
        assert!(!guess.could_match(Code(RED, YELLOW, BLUE, BLACK)));
    }
}
