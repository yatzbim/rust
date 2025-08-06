// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    if divisor == 0 {
        panic!("Cannot divide by zero!");
    }

    let is_negative = (dividend < 0 || divisor < 0) && !(dividend < 0 && divisor < 0);

    let mut quotient = 0;
    let mut remainder = dividend;
    while remainder >= divisor {
        remainder -= divisor;
        quotient += 1;
    }

    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.

    let mut i = 0;
    iter.filter(move |v| {
        let res = i % 2 == 0;
        i += 1;
        res
    })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}
