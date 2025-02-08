use crate::{display::pause_for_enter, guest::{GuestType::*, *}, party::*, player::*};
use clearscreen::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{f32::INFINITY, io::stdin};

pub fn get_num_players() -> usize {
    loop {
        clear().unwrap();
        println!("Party House: CLI Edition - Copyright (C) 2025 JazzerThighs\nTo view the README credits and disclaimer, type \"show r\"\n");
        println!("Welcome to Party House: CLI Edition! Enter the number of players:");
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }
        match input.trim() {
            i if i.parse::<usize>().map_or(false, |n| n > 0) => {
                clear().unwrap();
                let num = i.parse::<usize>().unwrap();
                print!("{num} Player{} selected!\n\n", {
                    if num == 1 {
                        ""
                    } else {
                        "s"
                    }
                });
                return num;
            },
            "show r" => {
                println!("Disclaimer

I am not affiliated with Mossmouth or any of the developers of UFO 50. This project is a fan-made demake created for educational and recreational purposes.

All code in this project has been written from scratch by me, and no original art assets, sounds, or proprietary code from UFO 50 have been used. This is a terminal-based version, distinct from the original game.

Credits & Acknowledgment

Full credit for the original Party House minigame concept, design, and inspiration goes to Mossmouth, the developers of UFO 50.

If you're interested in the official version, check out UFO 50! Support the developers and their work.
https://50games.fun/

Purpose of This Project

This project serves as:

    A programming exercise to recreate gameplay mechanics in a different environment.
    A way to explore demaking games for minimalistic platforms.
    A tribute to the creativity of UFO 50 and its developers.

I encourage everyone to play the original game when possible and support the developers!");
                pause_for_enter("Press \"Enter\" to continue...");
            },
            _ => {},
        }
    }
}

pub fn init_players(num_players: usize) -> (Vec<Player>, usize) {
    let mut players = vec![];
    let star_guest_arrivals_for_win: usize = match num_players {
        0 => unreachable!(),
        1 => 4,
        2.. => 3,
    };
    let rolodex = {
        let (friends, _, _, _) = guest_lists();
        let mut rolodex = vec![friends[&GuestType::OLD_FRIEND].clone(); 4];
        rolodex.extend(vec![friends[&GuestType::RICH_PAL].clone(); 2]);
        rolodex.extend(vec![friends[&GuestType::WILD_BUDDY].clone(); 4]);
        for i in 0..rolodex.len() {
            rolodex[i].id = i;
        }
        rolodex
    };
    for i in 0..num_players {
        players.push(Player {
            rolodex: rolodex.clone(),
            id: i,
            ..Default::default()
        })
    }
    (players, star_guest_arrivals_for_win)
}

const fn get_num_stocks(num_players: usize) -> f32 {
    (4 + (2 * (num_players - 1))) as f32
}

pub fn init_scenerio(num_players: usize) -> Vec<(Guest, f32)> {
    let (friends, randos, star_guests, _) = guest_lists();
    let num_stocks = get_num_stocks(num_players);
    macro_rules! place {
        (star_guests, $gt: ident) => {
            (star_guests[&$gt].clone(), INFINITY)
        };
        ($pile: ident, $gt: ident) => {
            ($pile[&$gt].clone(), num_stocks)
        };
    }
    let mut store = vec![
        place!(friends, OLD_FRIEND),
        place!(friends, RICH_PAL),
    ];
    let alien_invitation = vec![
        place!(randos, DRIVER),
        place!(randos, MONKEY),
        place!(randos, SECURITY),
        place!(randos, TICKET_TKR),
        place!(randos, WATCH_DOG),
        place!(randos, HIPPY),
        place!(randos, ROCK_STAR),
        place!(randos, COMEDIAN),
        place!(randos, CATERER),
        place!(randos, MR_POPULAR),
        place!(randos, DANCER),
        place!(randos, AUCTIONEER),
        place!(star_guests, ALIEN),
    ];
    let high_or_low = vec![
        place!(randos, PRIVATE_I),
        place!(randos, INTROVERT),
        place!(randos, GRILLMASTR),
        place!(randos, MASCOT),
        place!(randos, GANGSTER),
        place!(randos, CUTE_DOG),
        place!(randos, GAMBLER),
        place!(randos, SPY),
        place!(randos, WRITER),
        place!(randos, WRESTLER),
        place!(randos, CLIMBER),
        place!(star_guests, MERMAID),
        place!(star_guests, SUPERHERO),
    ];
    let best_wishes = vec![
        place!(randos, MONKEY),
        place!(randos, HIPPY),
        place!(randos, PHOTOGRPHR),
        place!(randos, CHEERLEADR),
        place!(randos, ROCK_STAR),
        place!(randos, ATHLETE),
        place!(randos, STYLIST),
        place!(randos, COUNSELOR),
        place!(randos, WRESTLER),
        place!(randos, CELEBRITY),
        place!(randos, BARTENDER),
        place!(star_guests, DINOSAUR),
        place!(star_guests, GENIE),
    ];
    let money_management = vec![
        place!(randos, PRIVATE_I),
        place!(randos, TICKET_TKR),
        place!(randos, SECURITY),
        place!(randos, PHOTOGRPHR),
        place!(randos, COMEDIAN),
        place!(randos, CATERER),
        place!(randos, GANGSTER),
        place!(randos, ATHLETE),
        place!(randos, STYLIST),
        place!(randos, CUTE_DOG),
        place!(randos, SPY),
        place!(star_guests, DRAGON),
        place!(star_guests, LEPRECHAUN),
    ];
    let a_magical_night = vec![
        place!(randos, INTROVERT),
        place!(randos, WATCH_DOG),
        place!(randos, WAREWOLF),
        place!(randos, GREETER),
        place!(randos, MAGICIAN),
        place!(randos, GAMBLER),
        place!(randos, DANCER),
        place!(randos, CUPID),
        place!(randos, AUCTIONEER),
        place!(randos, CELEBRITY),
        place!(randos, CLIMBER),
        place!(star_guests, UNICORN),
        place!(star_guests, GHOST),
    ];
    print!("Select desired Party Scenerio:\n\n");
    print!("1 -> Alien Invitation");
    for i in 0..alien_invitation.len() {
        print!("{} ", alien_invitation[i].0.emoji);
    }
    print!("\n\n");
    print!("2 -> High Or Low");
    for i in 0..high_or_low.len() {
        print!("{} ", high_or_low[i].0.emoji);
    }
    print!("\n\n");
    print!("3 -> Best Wishes");
    for i in 0..best_wishes.len() {
        print!("{} ", best_wishes[i].0.emoji);
    }
    print!("\n\n");
    print!("4 -> Money Management");
    for i in 0..money_management.len() {
        print!("{} ", money_management[i].0.emoji);
    }
    print!("\n\n");
    print!("5 -> A Magical Night");
    for i in 0..a_magical_night.len() {
        print!("{} ", a_magical_night[i].0.emoji);
    }
    print!("\n\n");
    print!("6 -> Randomized Scenerio\n");
    loop {
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(num) if num == 1 => {
                store.extend(alien_invitation);
                break;
            }
            Ok(num) if num == 2 => {
                store.extend(high_or_low);
                break;
            }
            Ok(num) if num == 3 => {
                store.extend(best_wishes);
                break;
            }
            Ok(num) if num == 4 => {
                store.extend(money_management);
                break;
            }
            Ok(num) if num == 5 => {
                store.extend(a_magical_night);
                break;
            }
            Ok(num) if num == 6 => {
                let mut randos_keys: Vec<GuestType> = randos.keys().cloned().collect();
                let mut rng = thread_rng();
                randos_keys.shuffle(&mut rng);
                let chosen_randos = &randos_keys[..11];
                for guest in chosen_randos {
                    store.extend(vec![(randos[&guest].clone(), num_stocks)]);
                }

                let mut star_keys: Vec<GuestType> = star_guests.keys().cloned().collect();
                let mut rng = thread_rng();
                star_keys.shuffle(&mut rng);
                let chosen_stars = &star_keys[..2];
                for guest in chosen_stars {
                    store.extend(vec![(star_guests[&guest].clone(), f32::INFINITY)]);
                }
                break;
            }
            Ok(_) | Err(_) => println!("Invalid input. Please enter a valid number as displayed."),
        }
    }
    clear().unwrap();
    for g in 0..store.len() {
        store[g].0.trouble = store[g].0.trouble_base;
        store[g].0.chill = store[g].0.chill_base;
        store[g].0.ability_stock = store[g].0.ability_base;
    }
    store.sort_by(|a, b| a.0.sort_value.cmp(&b.0.sort_value));
    
    store
}

pub fn handle_end_of_party(player: &mut Player, party: &mut Party) {
    player.rolodex.extend(party.attendees.drain(0..));
    player.rolodex.extend(player.booted.drain(0..));
    if player.banned.guest.is_some() && player.banned.already_served_time {
        player.rolodex.push(player.banned.guest.take().unwrap());
    }
    player.banned.already_served_time = true;
    if let Some(peek) = party.peek_slot.take() {
        player.rolodex.push(peek);
    }
    for guest in player.rolodex.iter_mut() {
        guest.trouble = guest.trouble_base;
        guest.chill = guest.chill_base;
        guest.ability_stock = guest.ability_base;
    }
    if let Some(banned) = &mut player.banned.guest {
        banned.trouble = banned.trouble_base;
        banned.chill = banned.chill_base;
        banned.ability_stock = banned.ability_base;
    }
}
