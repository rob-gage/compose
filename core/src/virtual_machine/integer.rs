// Copyright Rob Gage 2025

use rug::Integer as RugInteger;
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
pub struct Integer (RugInteger);

impl Integer {

    /// Create an `Integer` from a string slice
    pub fn from_string(string: &str) -> Option<Self> {
        RugInteger::from_str(string) // parse rug integer from string
            .ok() // convert result to option
            .map(Self) // apply `Integer` constructor
    }

    /// Returns the `Integer` as a `String`
    pub fn to_string(&self) -> String {
        self.0.to_string()
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