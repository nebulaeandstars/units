mod angle;
mod area;
mod length;
mod volume;

pub use angle::*;
pub use length::*;

pub trait Unit: Copy {
    fn value(self) -> f64;
}
