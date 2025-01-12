use clearscreen::clear;
use crate::{init::*, guest::*, clampedi8::*, player::*};

pub fn do_shopping(store: &mut Store, player: &mut Player) {
    if !store.still_shopping { return }
    println!("Player {}, spend Pop to add guests to your rolodex; Spend Cash to expand the capacity of your house:\n", player.id + 1);
    
    
    
    clear().unwrap();
}
