use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use strum::{Display, EnumCount, EnumIter, VariantArray};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount, VariantArray, Display)]
pub enum Colour {
    Black,
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

pub const COLUMNS: usize = 4;

impl Distribution<Colour> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colour {
        let index = rng.random_range(0..Colour::COUNT);

        Colour::VARIANTS[index]
    }
}
