use crate::Unit;
use derive_more::{
    Add, AddAssign, AsRef, Constructor, Div, DivAssign, From, Mul, MulAssign, Rem, Sub, SubAssign,
};
use std::f64::consts::PI;

pub trait Angle: Unit + Into<Radians> + Into<Degrees> {
    fn radians(self) -> Radians {
        self.into()
    }
    fn degrees(self) -> Degrees {
        self.into()
    }
    fn sin(self) -> f64 {
        <Self as Into<Radians>>::into(self).0.sin()
    }
    fn cos(self) -> f64 {
        <Self as Into<Radians>>::into(self).0.cos()
    }
    fn tan(self) -> f64 {
        <Self as Into<Radians>>::into(self).0.tan()
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
pub struct Degrees(f64);

impl Unit for Degrees {
    fn value(self) -> f64 {
        self.0
    }
}
impl Angle for Degrees {}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Degrees {
        Degrees::from(radians.0 * 180.0 / PI)
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
pub struct Radians(f64);

impl Unit for Radians {
    fn value(self) -> f64 {
        self.0
    }
}
impl Angle for Radians {}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Radians {
        Radians::new(degrees.0 * PI / 180.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angles() {
        let a = Degrees::from(90.0);
        let b = Radians::from(PI);
        assert_eq!(a * 2.0, b.into());
        assert_eq!(a * 4.0, (b * 2.0).into());
    }

    #[test]
    fn trigonometry() {
        let a = Degrees::from(30.0);
        let b = Radians::from(PI / 2.0);
        // Floating point maths is super weird, so answers won't be 100%
        // accurate. Instead, just check if the answers are *almost* correct.
        assert!(a.sin() - 0.5 < (10f64.powf(-10.0)));
        assert!((a * 2.0).cos() - 0.5 < (10f64.powf(-10.0)));
        assert!(b.sin() - 1.0 < (10f64.powf(-10.0)));
        assert!(b.cos() < (10f64.powf(-10.0)));
    }
}
