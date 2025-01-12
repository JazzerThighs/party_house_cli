#![allow(dead_code)]
mod clampedi8;
mod guest;
mod init;
mod player;
mod party;
mod store;
use clearscreen::clear;
use {init::*, party::*, store::*};

fn main() {
    let num_players = get_num_players();
    let (mut players, star_guest_arrivals_for_win) = init_players(num_players);
    let mut store = init_scenerio(num_players);
    let mut day_count = 1;
    let mut victories = vec![false; num_players + 1];

    'gameloop: loop {
        clear().unwrap();

        for player in players.iter_mut() {
            let mut party = init_party(&player.capacity, star_guest_arrivals_for_win);
            do_partying(&mut party, player, &mut victories);
            /*DEV_MARKER*/victories[player.id] = true;
            if !victories[player.id] {
                do_shopping(&mut store, player)
            }
            clear().unwrap();
        }

        clear().unwrap();
        match num_players {
            0 => unreachable!(),
            1 => {
                if day_count == 25 || victories[0] {
                    break 'gameloop;
                }
            }
            2.. => {
                if day_count == 100 || victories.iter().any(|v| *v) {
                    break 'gameloop;
                }
            }
        }
        
        day_count += 1;
    }

    clear().unwrap();
    for i in 0..victories.len() {
        match victories[i] {
            true => println!("Player {} threw the Ultimate Party!", i + 1),
            false => println!("Player {} loses!", i + 1),
        }
    }
    println!();
}
