#![doc = include_str!("../README.md")]

pub mod angle;
pub mod area;
pub mod length;
pub mod volume;

pub mod prelude {
    pub use crate::angle::Angle;
    pub use crate::area::Area;
    pub use crate::length::Length;
    pub use crate::volume::Volume;
}

pub trait Unit: Copy + From<f64> {
    fn value(self) -> f64;
}
