use crate::{guests::*, player::*};

pub fn cost_of_expansion(capacity: i8) -> i8 {
    match capacity {
        ..=4 => unreachable!(),
        5..=15 => capacity - 3,
        16.. => 12,
        
    }
}

pub fn do_shopping(store: &mut Vec<Guest>, player: &mut Player) -> bool {
    false
}