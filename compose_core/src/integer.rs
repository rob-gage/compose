// Copyright Rob Gage 2025

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::{
    ops::{
        Add,
        Div,
        Mul,
        Rem,
        Sub
    },
    str::FromStr
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Integer (BigInt);

impl Integer {

    /// Create an `Integer` from a string slice
    pub fn from_string(string: &str) -> Option<Self> {
        BigInt::from_str(string) // parse rug integer from string
            .ok() // convert result to option
            .map(Self) // apply `Integer` constructor
    }

    /// Creates a new `Integer` from a `usize`
    pub fn from_usize(usize: usize) -> Self { Self (BigInt::from(usize)) }

    /// Returns the `Integer` as a `String`
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Returns this `Integer` as a `usize` wrapping index for an item in a stack or list with a
    /// given size, returning `usize::MAX` as a sentinel value indicating an empty space
    pub fn as_wrapping_index(&self, stack_size: usize) -> usize {
        if stack_size == 0 { return usize::MAX; }
        let size: BigInt = BigInt::from(stack_size);
        let index: BigInt = (&self.0 % &size + &size) % &size;
        index.to_usize().unwrap_or(usize::MAX)
    }

}

impl Add for Integer {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self (self.0 + other.0)
    }
}

impl Div for Integer {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self (self.0 / other.0)
    }
}

impl Mul for Integer {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self (self.0 * other.0)
    }
}

impl Rem for Integer {
    type Output = Self;
    #[inline]
    fn rem(self, other: Self) -> Self {
        Self (self.0 % other.0)
    }
}

impl Sub for Integer {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self (self.0 - other.0)
    }
}