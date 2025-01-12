use clearscreen::clear;
use crate::{guest::*, player::*};

#[derive(Debug, Clone)]
pub struct Store {
    pub stock: Vec<(Guest, f32)>,
    pub still_shopping: bool,
}
impl Store {
        const fn cost_of_expansion(capacity: i8) -> i8 {
        match capacity {
            ..=4 => unreachable!(),
            5..=15 => capacity - 3,
            16..=34 => 12,
            35.. => 0
        }
    }
}

pub fn do_shopping(store: &mut Store, player: &mut Player) {
    if !store.still_shopping { return }
    println!("Player {}, spend Pop to add guests to your rolodex; Spend Cash to expand the capacity of your house:\n", player.id + 1);
    
    
    
    clear().unwrap();
}
