use crate::Unit;
use derive_more::{Add, AsRef, Constructor, Div, From, Mul, Rem, Sub};

pub trait Length: Unit + Into<Meters> + Into<Kilometers> + Into<Miles> {
    fn meters(self) -> Meters {
        self.into()
    }
    fn kilometers(self) -> Kilometers {
        self.into()
    }
    fn miles(self) -> Miles {
        self.into()
    }
}

#[derive(
    Constructor, Clone, Copy, Debug, PartialEq, PartialOrd, Add, Sub, Mul, Div, Rem, AsRef, From,
)]
pub struct Meters(f64);

impl Unit for Meters {
    fn value(self) -> f64 {
        self.0
    }
}
impl Length for Meters {}

impl From<Kilometers> for Meters {
    fn from(kilometers: Kilometers) -> Self {
        Meters::from(kilometers.0 * 1000.0)
    }
}

impl From<Miles> for Meters {
    fn from(miles: Miles) -> Self {
        Meters::from(miles.0 * 1609.344)
    }
}

#[derive(
    Constructor, Clone, Copy, Debug, PartialEq, PartialOrd, Add, Sub, Mul, Div, Rem, AsRef, From,
)]
pub struct Kilometers(f64);

impl Unit for Kilometers {
    fn value(self) -> f64 {
        self.0
    }
}
impl Length for Kilometers {}

impl From<Meters> for Kilometers {
    fn from(meters: Meters) -> Self {
        Kilometers::from(meters.0 / 1000.0)
    }
}

impl From<Miles> for Kilometers {
    fn from(miles: Miles) -> Self {
        Kilometers::from(miles.0 * 1.609344)
    }
}

#[derive(
    Constructor, Clone, Copy, Debug, PartialEq, PartialOrd, Add, Sub, Mul, Div, Rem, AsRef, From,
)]
pub struct Miles(f64);

impl Unit for Miles {
    fn value(self) -> f64 {
        self.0
    }
}
impl Length for Miles {}

impl From<Meters> for Miles {
    fn from(meters: Meters) -> Self {
        Miles::from(meters.0 / 1609.344)
    }
}

impl From<Kilometers> for Miles {
    fn from(kilometers: Kilometers) -> Self {
        Miles::from(kilometers.0 / 1.609344)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison() {
        let a: Meters = 1400.0.into();
        let b: Kilometers = 700.0.into();
        assert_eq!(Kilometers::from(1.4), a.into());
        assert_eq!(Meters::from(700000.0), b.into());
        assert!(a < b.into());
        assert!(b > a.into());
    }

    #[test]
    fn arithmetic() {
        let a = Meters::from(2.0);
        let b = Meters::from(3.0);
        assert_eq!(a + b, Meters::from(5.0));
        assert_eq!(b - a, Meters::from(1.0));
        assert_eq!(a * 3.0, Meters::from(6.0));
        assert_eq!(b / 2.0, Meters::from(1.5));
    }

    #[test]
    fn conversion() {
        let a = Meters::from(200.0);
        let b = Kilometers::from(2.0);
        assert_eq!(a + b.into(), Meters::from(2200.0));
        assert_eq!(b - a.into(), Kilometers::from(1.8));
    }

    #[test]
    fn miles() {
        let a = Meters::from(4023.36);
        let b = Miles::from(3.0);
        assert_eq!(Miles::from(2.5), a.into());
        assert_eq!(Meters::from(4828.032), b.into());
        assert_eq!(Kilometers::from(4.828032), b.into());
    }
}
