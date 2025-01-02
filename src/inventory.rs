use crate::init::*;
use card_deck::Deck;
use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

#[derive(Debug, Clone)]
pub struct Wallet(i8);

impl Add for Wallet {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 = min(max(self.0 + rhs.0, 0), 65);
        self
    }
}
impl AddAssign for Wallet {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}
impl Sub for Wallet {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 = min(max(self.0 - rhs.0, 0), 65);
        self
    }
}
impl SubAssign for Wallet {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}
impl Deref for Wallet {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Wallet {
    fn deref_mut(&mut self) -> &mut i8 {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub rolodex: Deck<crate::guests::Guest>,
    pub popularity: Wallet,
    pub cash: Wallet,
    pub capacity: Wallet,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            rolodex: init_rolodex(),
            popularity: Wallet(0),
            cash: Wallet(0),
            capacity: Wallet(0),
        }
    }
}
