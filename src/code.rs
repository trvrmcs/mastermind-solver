use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use strum::IntoEnumIterator;

use crate::Colour;
use std::{fmt, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code(pub Colour, pub Colour, pub Colour, pub Colour);

impl Index<usize> for Code {
    type Output = Colour;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(),
        }
    }
}

impl Code {
    pub fn all_codes() -> Vec<Code> {
        let mut out = vec![];

        for a in Colour::iter() {
            for b in Colour::iter() {
                for c in Colour::iter() {
                    for d in Colour::iter() {
                        let row: Code = Code(a, b, c, d);
                        out.push(row);
                    }
                }
            }
        }
        out
    }
}

impl Distribution<Code> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Code {
        Code(rng.random(), rng.random(), rng.random(), rng.random())
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| {:<6} | {:<6} | {:<6} | {:<6} |",
            self.0, self.1, self.2, self.3
        )
    }
}
#[cfg(test)]
mod test {
    use super::*;

    const RED: Colour = Colour::Red;

    #[test]
    fn test_01() {
        let _ = Code(RED, RED, RED, RED);
    }
}
