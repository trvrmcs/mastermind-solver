use crate::Code;

use crate::colour::{COLUMNS, Colour};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    pub right: u8, // right color, right position
    pub wrong: u8, // right color, wrong position
}

impl Response {
    pub fn new(right: u8, wrong: u8) -> Self {
        assert!(right + wrong <= 4);
        Self { right, wrong }
    }

    pub fn could_match(&self, secret: Code, guess: Code) -> bool {
        Response::check(secret, guess) == *self
    }

    pub fn all_responses() -> Vec<Response> {
        // I'd prefer an internal representation that makes this
        // unavoidable!
        let mut out = vec![];
        for right in 0..5 {
            for wrong in 0..5 {
                if right + wrong <= 4 {
                    out.push(Response::new(right, wrong))
                }
            }
        }
        out
    }
}
impl Response {
    pub fn check(secret: Code, guess: Code) -> Self {
        let mut right: u8 = 0;
        let mut wrong: u8 = 0;

        let mut secret: [Option<Colour>; COLUMNS] = [
            Some(secret.0),
            Some(secret.1),
            Some(secret.2),
            Some(secret.3),
        ];
        let mut guess: [Option<Colour>; COLUMNS] =
            [Some(guess.0), Some(guess.1), Some(guess.2), Some(guess.3)];

        // Color correct, location correct
        for i in 0..COLUMNS {
            if let Some(secret_color) = secret[i] {
                if let Some(guess_color) = guess[i] {
                    if secret_color == guess_color {
                        right += 1;
                        guess[i] = None;
                        secret[i] = None;
                    }
                }
            }
        }

        // Color correct, location wrong
        for (s_index, s) in secret.iter_mut().enumerate() {
            if let Some(secret_color) = s {
                for (g_index, g) in guess.iter_mut().enumerate() {
                    if let Some(guess_color) = g {
                        if secret_color == guess_color {
                            wrong += 1;
                            assert!(s_index != g_index);
                            *g = None;
                            *s = None;
                            break;
                        }
                    }
                }
            }
        }
        assert!(right + wrong <= 4);
        Response { right, wrong }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::colour::*;

    const BLACK: Colour = Colour::Black;
    const BLUE: Colour = Colour::Blue;
    const GREEN: Colour = Colour::Green;
    const RED: Colour = Colour::Red;
    const WHITE: Colour = Colour::White;
    const YELLOW: Colour = Colour::Yellow;

    #[test]
    fn test_response_01() {
        let secret: Code = Code(RED, RED, RED, RED);
        let guess: Code = Code(RED, RED, RED, RED);

        assert_eq!(
            Response::check(secret, guess),
            Response { right: 4, wrong: 0 }
        );
    }

    #[test]
    fn test_02a() {
        let secret: Code = Code(RED, BLACK, BLACK, BLACK);
        let guess: Code = Code(WHITE, RED, WHITE, RED);

        assert_eq!(
            Response::check(secret, guess),
            Response { right: 0, wrong: 1 }
        );
    }
    #[test]
    fn test_03() {
        let secret: Code = Code(RED, BLACK, GREEN, YELLOW);
        let guess: Code = Code(YELLOW, RED, BLACK, GREEN);

        assert_eq!(
            Response::check(secret, guess),
            Response { right: 0, wrong: 4 }
        );
    }
    #[test]
    fn test_04() {
        let secret: Code = Code(RED, BLACK, GREEN, YELLOW);
        let guess: Code = Code(YELLOW, RED, BLACK, RED);

        assert_eq!(
            Response::check(secret, guess),
            Response { right: 0, wrong: 3 }
        );
    }
    #[test]
    fn test_response_05() {
        let secret: Code = Code(RED, RED, RED, WHITE);
        let guess: Code = Code(WHITE, BLUE, BLACK, WHITE);
        assert_eq!(
            Response::check(secret, guess),
            Response { right: 1, wrong: 0 }
        );
    }

    #[test]
    fn test_06() {
        let a = Response::all_responses();

        println!("{:?}", a);
        assert_eq!(a.len(), 15)
    }

    #[test]
    fn test_07() {
        let secret = Code(WHITE, BLUE, YELLOW, BLUE);
        let guess = Code(RED, BLUE, YELLOW, YELLOW);

        assert_eq!(
            Response::check(secret, guess),
            Response { right: 2, wrong: 0 }
        );
    }
}
