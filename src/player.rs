use crate::guests::*;
use card_deck::Deck;
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

#[derive(Debug, Clone)]
pub struct Player {
    pub rolodex: Deck<crate::guests::Guest>,
    pub popularity: ClampedI8,
    pub cash: ClampedI8,
    pub capacity: ClampedI8,
    pub victory: bool
}

pub fn init_player() -> Player {
    Player {
        rolodex: {
            let (friends, _, _) = guest_lists();
            let mut rolodex = vec![friends[&GuestType::OLD_FRIEND].clone(); 4];
            rolodex.extend(vec![friends[&GuestType::RICH_PAL].clone(); 2]);
            rolodex.extend(vec![friends[&GuestType::WILD_BUDDY].clone(); 4]);
            for i in 0..rolodex.len() {
                rolodex[i].id = i;
            }
            Deck::new(rolodex)
        },
        popularity: ClampedI8 {
            value: 0,
            min: 0,
            max: 65,
        },
        cash: ClampedI8 {
            value: 0,
            min: 0,
            max: 30,
        },
        capacity: ClampedI8 {
            value: 5,
            min: 5,
            max: 34,
        },
        victory: false
    }
}
