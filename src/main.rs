#![allow(dead_code)]
mod guests;
mod init;
mod party;
mod player;
mod store;
use clearscreen::clear;
use init::*;
use party::*;
use player::*;
use store::*;

fn main() {
    let num_players = get_num_players();
    let mut store = create_scenerio(num_players);
    let mut players: Vec<Player> = vec![init_player().clone(); num_players];
    let mut day_count: u8 = 1;
    'gameloop: loop {
        for player in players.iter_mut() {
            let mut party = init_party(&player.capacity);
            while do_partying(&mut party, player) {}
            if !player.victory {
                while do_shopping(&mut store, player) {}
            }
        }
        day_count += 1;
        match players.len() {
            0 => unreachable!(),
            1 => {
                if day_count == 25 {
                    break 'gameloop;
                }
            }
            2.. => {
                if players.iter().any(|p| p.victory) {
                    break 'gameloop;
                }
            }
        }
    }
    clear().unwrap();
    for i in 0..players.len() {
        match players[i].victory {
            true => println!("Player {i} threw the Ultimate Party!"),
            false => println!("Player {i} loses!"),
        }
    }
}
