use crate::area::SquareMeters;
use crate::length::Meters;
use crate::Unit;
use derive_more::{
    Add, AddAssign, AsRef, Constructor, Div, DivAssign, From, Mul, MulAssign, Rem, Sub, SubAssign,
};

pub trait Volume: Unit + Into<CubicMeters> {
    fn square_meters(self) -> CubicMeters {
        self.into()
    }
}

#[derive(
    Constructor,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Rem,
    AsRef,
    From,
)]
pub struct CubicMeters(f64);

impl Unit for CubicMeters {
    fn value(self) -> f64 {
        self.0
    }
}
impl Volume for CubicMeters {}

impl std::ops::Mul<Meters> for SquareMeters {
    type Output = CubicMeters;
    fn mul(self, other: Meters) -> Self::Output {
        Self::Output::new(self.value() * other.value())
    }
}

impl std::ops::Mul<SquareMeters> for Meters {
    type Output = CubicMeters;
    fn mul(self, other: SquareMeters) -> Self::Output {
        Self::Output::new(self.value() * other.value())
    }
}

impl std::ops::Div<Meters> for CubicMeters {
    type Output = SquareMeters;
    fn div(self, other: Meters) -> Self::Output {
        Self::Output::new(self.value() / other.value())
    }
}

impl std::ops::Div<SquareMeters> for CubicMeters {
    type Output = Meters;
    fn div(self, other: SquareMeters) -> Self::Output {
        Self::Output::new(self.value() / other.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume() {
        let a = Meters::from(2.0);
        let b = SquareMeters::from(4.0);
        let c = CubicMeters::from(8.0);
        assert_eq!(a * a, b);
        assert_eq!(b * a, c);
        assert_eq!(a * b, c);
        assert_eq!(c / a, b);
        assert_eq!(c / b, a);
    }
}
