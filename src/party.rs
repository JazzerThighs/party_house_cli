use clearscreen::clear;
use crate::init::*;

pub fn do_partying(party: &mut Party, player: &mut Player, victories: &mut Vec<bool>) -> bool {
    let still_partying: bool = true;
    println!("Player {}, throw a party!", player.id);
    if victories.iter().any(|v| *v) {
        for i in 0..victories.len() {
            if victories[i] {
                println!("Player {} won today!", i + 1)
            };
        }
        println!("Last Chance!\n");
    }
    clear().unwrap();
    still_partying
}