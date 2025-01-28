use crate::{guest::{GuestType::*, *}, party::*, player::*};
use clearscreen::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{f32::INFINITY, cmp::max, io::stdin};

pub fn get_num_players() -> usize {
    loop {
        clear().unwrap();
        println!("Welcome to Party House: CLI Edition! Enter the number of players:");
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(num) => {
                clear().unwrap();
                print!("{} Player{} selected!\n\n", max(num, 1), {
                    if num == 1 {
                        ""
                    } else {
                        "s"
                    }
                });
                return max(num, 1);
            }
            Err(_) => eprintln!("Invalid input. Please enter a valid positive number."),
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
        let (friends, _, _) = guest_lists();
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
    let (friends, randos, star_guests) = guest_lists();
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

pub fn init_party(party: &mut Party, player: &mut Player, stars_to_win: usize) {
    player.rolodex.extend(player.booted.drain(0..));
    for guest in player.rolodex.iter_mut() {
        guest.trouble = guest.trouble_base;
        guest.chill = guest.chill_base;
        guest.ability_stock = guest.ability_base;
    }
    *party = Party {
        capacity: player.capacity.clone(),
        stars_to_win,
        ..Default::default()
    };
    let mut rng = thread_rng();
    player.rolodex.shuffle(&mut rng);
}
