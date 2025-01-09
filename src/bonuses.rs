use crate::{guests::GuestType, init::*};
use std::cmp::max;

pub fn comedian_bonus(party: &Party) -> i8 {
    if party.attendees.len() as i8 == *party.capacity { 5 } else { 0 }
}
pub fn introvert_bonus(party: &Party) -> i8 {
    max(0, *party.capacity - party.attendees.len() as i8)
}
pub fn dancer_bonus(party: &Party) -> i8 {
    max(16, party.attendees.iter().filter(|guest| guest.guest == GuestType::DANCER).count().pow(2) as i8)
}
pub fn mascot_bonus(party: &Party) -> i8 {
    party.attendees.iter().filter(|guest| guest.guest == GuestType::OLD_FRIEND).count() as i8
}
pub fn writer_bonus(party: &Party) -> i8 {
    2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8
}
pub fn bartender_bonus(party: &Party) -> i8 {
    2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8
}