use crate::{clampedi8::*, guests::*, player::*};

pub struct Party {
    pub attendees: Vec<Guest>,
    pub capacity: ClampedI8,
    pub trouble_count: u8,
    pub chill_count: u8,
    pub narcs: bool,
    pub overflow: bool
}

pub fn init_party(cap: &ClampedI8) -> Party {
    Party {
        attendees: vec![],
        capacity: cap.clone(),
        trouble_count: 0,
        chill_count: 0,
        narcs: false,
        overflow: false
    }
}

pub fn do_partying(party: &mut Party, player: &mut Player) -> bool {
    false
}