use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Deref},
};

#[derive(Debug, Clone)]
pub struct ClampedI8 {
    pub value: i8,
    pub min: i8,
    pub max: i8,
}

impl Add<i8> for ClampedI8 {
    type Output = Self;
    fn add(mut self, rhs: i8) -> Self::Output {
        self.value = min(max(self.value + rhs, self.min), self.max);
        self
    }
}
impl AddAssign<i8> for ClampedI8 {
    fn add_assign(&mut self, rhs: i8) {
        *self = self.clone() + rhs;
    }
}

impl Deref for ClampedI8 {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<(i8, i8, i8)> for ClampedI8 {
    fn from(tuple: (i8, i8, i8)) -> Self {
        ClampedI8 {
            value: tuple.1,
            min: tuple.1,
            max: tuple.2
        }
    }
}

impl ClampedI8 {
    pub fn pop_cash(value: i8) -> Self {
        ClampedI8::from((value, -9, 9))
    }
    pub fn stars(value: i8) -> Self {
        ClampedI8::from((value, -1, 1))
    }
    pub fn capacity() -> Self {
        ClampedI8::from((5, 5, 34))
    } 
}