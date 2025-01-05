use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Sub, SubAssign, Deref, DerefMut},
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
impl Sub<i8> for ClampedI8 {
    type Output = Self;
    fn sub(mut self, rhs: i8) -> Self::Output {
        self.value = min(max(self.value - rhs, self.min), self.max);
        self
    }
}
impl SubAssign<i8> for ClampedI8 {
    fn sub_assign(&mut self, rhs: i8) {
        *self = self.clone() - rhs;
    }
}

impl Deref for ClampedI8 {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for ClampedI8 {
    fn deref_mut(&mut self) -> &mut i8 {
        &mut self.value
    }
}