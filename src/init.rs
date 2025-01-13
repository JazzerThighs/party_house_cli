use crate::{clampedi8::ClampedI8, guest::*, party::*, player::*, store::*};
use clearscreen::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{cmp::max, io::stdin};

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
pub fn init_scenerio(num_players: usize) -> Store {
    let (friends, randos, star_guests) = guest_lists();
    let num_stocks = get_num_stocks(num_players);
    use GuestType::*;
    let mut store = vec![
        (friends[&OLD_FRIEND].clone(), num_stocks),
        (friends[&RICH_PAL].clone(), num_stocks),
    ];
    let alien_invitation = vec![
        (randos[&DRIVER].clone(), num_stocks),
        (randos[&MONKEY].clone(), num_stocks),
        (randos[&SECURITY].clone(), num_stocks),
        (randos[&TICKET_TKR].clone(), num_stocks),
        (randos[&WATCH_DOG].clone(), num_stocks),
        (randos[&HIPPY].clone(), num_stocks),
        (randos[&ROCK_STAR].clone(), num_stocks),
        (randos[&COMEDIAN].clone(), num_stocks),
        (randos[&CATERER].clone(), num_stocks),
        (randos[&MR_POPULAR].clone(), num_stocks),
        (randos[&DANCER].clone(), num_stocks),
        (randos[&AUCTIONEER].clone(), num_stocks),
        (star_guests[&ALIEN].clone(), f32::INFINITY),
    ];
    let high_or_low = vec![
        (randos[&PRIVATE_I].clone(), num_stocks),
        (randos[&INTROVERT].clone(), num_stocks),
        (randos[&GRILLMASTR].clone(), num_stocks),
        (randos[&MASCOT].clone(), num_stocks),
        (randos[&GANGSTER].clone(), num_stocks),
        (randos[&CUTE_DOG].clone(), num_stocks),
        (randos[&GAMBLER].clone(), num_stocks),
        (randos[&SPY].clone(), num_stocks),
        (randos[&WRITER].clone(), num_stocks),
        (randos[&WRESTLER].clone(), num_stocks),
        (randos[&CLIMBER].clone(), num_stocks),
        (star_guests[&MERMAID].clone(), f32::INFINITY),
        (star_guests[&SUPERHERO].clone(), f32::INFINITY),
    ];
    let best_wishes = vec![
        (randos[&MONKEY].clone(), num_stocks),
        (randos[&HIPPY].clone(), num_stocks),
        (randos[&PHOTOGRPHR].clone(), num_stocks),
        (randos[&CHEERLEADR].clone(), num_stocks),
        (randos[&ROCK_STAR].clone(), num_stocks),
        (randos[&ATHLETE].clone(), num_stocks),
        (randos[&STYLIST].clone(), num_stocks),
        (randos[&COUNSELOR].clone(), num_stocks),
        (randos[&WRESTLER].clone(), num_stocks),
        (randos[&CELEBRITY].clone(), num_stocks),
        (randos[&BARTENDER].clone(), num_stocks),
        (star_guests[&DINOSAUR].clone(), f32::INFINITY),
        (star_guests[&GENIE].clone(), f32::INFINITY),
    ];
    let money_management = vec![
        (randos[&PRIVATE_I].clone(), num_stocks),
        (randos[&TICKET_TKR].clone(), num_stocks),
        (randos[&SECURITY].clone(), num_stocks),
        (randos[&PHOTOGRPHR].clone(), num_stocks),
        (randos[&COMEDIAN].clone(), num_stocks),
        (randos[&CATERER].clone(), num_stocks),
        (randos[&GANGSTER].clone(), num_stocks),
        (randos[&ATHLETE].clone(), num_stocks),
        (randos[&STYLIST].clone(), num_stocks),
        (randos[&CUTE_DOG].clone(), num_stocks),
        (randos[&SPY].clone(), num_stocks),
        (star_guests[&DRAGON].clone(), f32::INFINITY),
        (star_guests[&LEPRECHAUN].clone(), f32::INFINITY),
    ];
    let a_magical_night = vec![
        (randos[&INTROVERT].clone(), num_stocks),
        (randos[&WATCH_DOG].clone(), num_stocks),
        (randos[&WAREWOLF].clone(), num_stocks),
        (randos[&GREETER].clone(), num_stocks),
        (randos[&MAGICIAN].clone(), num_stocks),
        (randos[&GAMBLER].clone(), num_stocks),
        (randos[&DANCER].clone(), num_stocks),
        (randos[&CUPID].clone(), num_stocks),
        (randos[&AUCTIONEER].clone(), num_stocks),
        (randos[&CELEBRITY].clone(), num_stocks),
        (randos[&CLIMBER].clone(), num_stocks),
        (star_guests[&UNICORN].clone(), f32::INFINITY),
        (star_guests[&GHOST].clone(), f32::INFINITY),
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
    print!("6 -> Randomized Scenerios\n");
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
                let flip: f64 = thread_rng().gen();
                let (randos_num, star_guests_num) = if flip < 0.2 { (11, 2) } else { (12, 1) };

                let mut randos_keys: Vec<GuestType> = randos.keys().cloned().collect();
                let mut rng = thread_rng();
                randos_keys.shuffle(&mut rng);
                let chosen_randos = &randos_keys[..randos_num];
                for guest in chosen_randos {
                    store.extend(vec![(randos[&guest].clone(), num_stocks)]);
                }

                let mut star_keys: Vec<GuestType> = star_guests.keys().cloned().collect();
                let mut rng = thread_rng();
                star_keys.shuffle(&mut rng);
                let chosen_stars = &star_keys[..star_guests_num];
                for guest in chosen_stars {
                    store.extend(vec![(star_guests[&guest].clone(), f32::INFINITY)]);
                }
                break;
            }
            Ok(_) => eprintln!("Invalid input. Please enter a valid number as displayed."),
            Err(_) => eprintln!("Invalid input. Please enter a valid number as displayed."),
        }
    }
    clear().unwrap();
    
    store.sort_by(|a, b| a.0.sort_value.cmp(&b.0.sort_value));
    
    Store {
        stock: store,
        done_shopping: true,
    }
}

pub fn init_party(cap: &ClampedI8, star_guest_arrivals_for_win: usize) -> Party {
    Party {
        capacity: cap.clone(),
        star_guest_arrivals_for_win,
        ..Default::default()
    }
}
