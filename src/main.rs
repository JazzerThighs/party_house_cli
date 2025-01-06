#![allow(dead_code)]
mod clampedi8;
mod guests;
mod init;
mod party;
mod store;
use clearscreen::clear;
use {init::*, party::*, store::*};

fn main() {
    let num_players = get_num_players();
    let mut players = init_players(num_players);
    let mut store = init_scenerio(num_players);
    let mut day_count = 1;

    'gameloop: loop {
        clear().unwrap();

        for player in players.iter_mut() {
            println!("Player {}, throw a party!", player.id);
            let mut party = init_party(&player.capacity);

            while do_partying(&mut party, player) {}
            
            match (party.narcs, party.overflow, player.victory) {
                (true, _, _) => {}
                (_, true, _) => {}
                (_, _, false) => while do_shopping(&mut store, player) {},
                (_, _, _) => {}
            }
        }

        clear().unwrap();
        match num_players {
            0 => unreachable!(),
            1 => {
                if day_count == 25 || players[0].victory {
                    break 'gameloop;
                }
            }
            2.. => {
                if players.iter().any(|p| p.victory) {
                    break 'gameloop;
                }
            }
        }
        
        day_count += 1;
    }

    clear().unwrap();
    for player in players.iter() {
        match player.victory {
            true => println!("Player {} threw the Ultimate Party!", player.id),
            false => println!("Player {} loses!", player.id),
        }
    }
}
