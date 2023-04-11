# units

Force proper unit handling at compile time using rust's type system.

This crate contains types such as [`Meters`] and
[`Radians`], and will force you to use them correctly. It
aims to handle unit conversion for you, so that you no longer have to worry
about [crashing your probe into
Mars](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter#Cause_of_failure), or
waste time wondering whether you should be using radians or degrees for that one
trig function.

Arithmetic operations using these types (with the exception of exponents) should
work as normal. For example:

```rust
use units::length::{Meters, Kilometers};
use units::area::SquareMeters;

let a = Meters::new(150.0);
let b = Meters::new(600.0);

assert_eq!(a + b, Meters::new(750.0));
assert_eq!(a * 2.0, Meters::new(300.0));
assert_eq!(a * b, SquareMeters::new(90_000.0));

assert_eq!(Kilometers::from(a + b), Kilometers::new(0.75));
```

Similarly, units can all be cast between one-another (if applicable):

```rust
use units::prelude::*;
use units::angle::{Degrees, Radians};
use std::f64::consts::PI;

fn foo(theta: Degrees) {
    assert_eq!(theta, 90.0.into());
}

let a: Radians = (PI / 2.0).into();

foo(a.into());

// If using traits from the prelude, you can also cast explicitly:
foo(a.degrees())
```

[`Meters`]: crate::length::Meters
[`Radians`]: crate::angle::Radians
