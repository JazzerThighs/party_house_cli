use crate::guests::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use clearscreen::*;
use std::io;

pub fn get_num_players() -> usize {
    let num_players: usize;
    loop {
        println!("Welcome to Party House: CLI Edition! Enter the number of players:");
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => {
                num_players = num;
                break;
            }
            Ok(_) => eprintln!("The number of players must be at least 1. Please try again."),
            Err(_) => eprintln!("Invalid input. Please enter a valid positive number.")
        }
    }
    clear().unwrap();
    num_players
}

pub fn init_scenerio(num_players: usize) -> Vec<(Guest, f32)> {
    let (friends, randos, star_guests) = guest_lists();

    let num_stocks = 4 + (2 * (num_players - 1));

    let mut store = vec![(friends[&GuestType::OLD_FRIEND].clone(), num_stocks as f32)];
    store.extend(vec![(friends[&GuestType::RICH_PAL].clone(), num_stocks as f32)]);

    let flip: f64 = thread_rng().gen();
    let (randos_num, star_guests_num) = if flip < 0.5 { (11, 2) } else { (12, 1) };

    let mut randos_keys: Vec<GuestType> = randos.keys().cloned().collect();
    let mut rng = thread_rng();
    randos_keys.shuffle(&mut rng);
    let chosen_randos = &randos_keys[..randos_num];
    for guest in chosen_randos {
        store.extend(vec![(randos[&guest].clone(), num_stocks as f32)]);
    }

    let mut star_keys: Vec<GuestType> = star_guests.keys().cloned().collect();
    let mut rng = thread_rng();
    star_keys.shuffle(&mut rng);
    let chosen_stars = &star_keys[..star_guests_num];
    for guest in chosen_stars {
        store.extend(vec![(star_guests[&guest].clone(), f32::INFINITY)]);
    }
    store
}
