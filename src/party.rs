use crate::{guests::*, player::*};

pub struct Party {
    pub attendees: Vec<Guest>,
    pub capacity: ClampedI8,
    pub trouble_count: u8,
    pub chill_count: u8,
    pub still_partying: bool,
    pub still_shopping: bool
}

pub fn init_party(cap: &ClampedI8) -> Party {
    Party {
        attendees: vec![],
        capacity: cap.clone(),
        trouble_count: 0,
        chill_count: 0,
        still_partying: true,
        still_shopping: true
    }
}

pub fn do_partying(party: &mut Party, player: &mut Player) -> bool {
    false
}