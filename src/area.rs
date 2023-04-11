use crate::length::Meters;
use crate::Unit;
use derive_more::{Add, AsRef, Constructor, Div, From, Mul, Rem, Sub};

pub trait Area: Unit + Into<SquareMeters> {
    fn square_meters(self) -> SquareMeters {
        self.into()
    }
}

#[derive(
    Constructor, Clone, Copy, Debug, PartialEq, PartialOrd, Add, Sub, Mul, Div, Rem, AsRef, From,
)]
pub struct SquareMeters(f64);

impl Unit for SquareMeters {
    fn value(self) -> f64 {
        self.0
    }
}
impl Area for SquareMeters {}

impl std::ops::Mul<Meters> for Meters {
    type Output = SquareMeters;
    fn mul(self, other: Meters) -> Self::Output {
        Self::Output::new(self.value() * other.value())
    }
}

impl std::ops::Div<Meters> for SquareMeters {
    type Output = Meters;
    fn div(self, other: Meters) -> Self::Output {
        Self::Output::new(self.value() / other.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let a = Meters::from(2.0);
        let b = Meters::from(3.0);
        let c = SquareMeters::from(6.0);
        assert_eq!(a * b, c);
        assert_eq!(c / a, b);
        assert_eq!(c / b, a);
    }
}
