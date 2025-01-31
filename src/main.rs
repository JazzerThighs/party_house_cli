mod clampedi8;
mod display;
mod guest;
mod init;
mod party;
mod player;

use clearscreen::clear;
use guest::Guest;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    cmp::{min, Reverse},
    io::stdin,
};
use {
    display::*,
    guest::{AbilityType::*, GuestType::*},
    init::*,
    party::{PartyState::*, *},
    player::*,
};

fn main() {
    let num_players: usize = get_num_players();
    let (mut players, star_guest_arrivals_for_win): (Vec<Player>, usize) =
        init_players(num_players);
    let mut store: Vec<(Guest, f32)> = init_scenerio(num_players);
    let mut day_count: usize = 1;
    let mut victories: Vec<bool> = vec![false; num_players];
    let mut party: Party = Party::default();

    'game: loop {
        clear().unwrap();
        for player in players.iter_mut() {
            init_party(&mut party, player, star_guest_arrivals_for_win);
            let mut boxed_message: &str = "";
            macro_rules! refresh {
                (party $message:expr) => {
                    boxed_message = $message;
                    party_display(&party, player, &victories, day_count, &boxed_message.to_string());
                };
                (store $message:expr) => {
                    boxed_message = $message;
                    store_display(&store, player, &victories, day_count, &star_guest_arrivals_for_win, &boxed_message.to_string());
                };
            }
            'ongoing_party: loop {
                let mut rng = thread_rng();
                player.rolodex.shuffle(&mut rng);
                let (
                    house_is_full,
                    rolodex_is_empty,
                    available_full_house_abilities,
                    replenishes_available,
                ) = get_party_state(&party, player);
                refresh!(party boxed_message);
                match party.state {
                    IncomingGuest { mut amount, greet } if amount >= 1 => {
                        if player.rolodex.is_empty() {
                            party.state = IncomingGuest {
                                amount: 0,
                                greet: false,
                            };
                            continue 'ongoing_party;
                        } else {
                            if let Some(next_in_line) = party.peek_slot.take() {
                                party.attendees.push(next_in_line);
                            } else {
                                party.attendees.push(player.rolodex.pop().unwrap())
                            };
                            let newest_guest = party.attendees.len() - 1;
                            if greet {
                                player.add_pop_from_guest(*party.attendees[newest_guest].popularity);
                                if *party.attendees[newest_guest].cash >= 0 {
                                    player.add_cash_from_guest(*party.attendees[newest_guest].cash);
                                }
                                player.add_pop_from_guest((party.attendees[newest_guest].bonus_pop)(&party));
                                if party.attendees[newest_guest].guest_type == DANCER {
                                    player.add_pop_from_guest(min(16, party.attendees.iter().filter(|a| a.guest_type == DANCER).count().pow(2) as i8,))
                                };
                                player.add_cash_from_guest((party.attendees[newest_guest].bonus_cash)(&party));
                                if *party.attendees[newest_guest].cash < 0 {
                                    player.add_cash_from_guest(*party.attendees[newest_guest].cash);
                                }
                            }
                            (party.attendees[newest_guest].entrance_effect)(
                                &mut party.attendees[newest_guest],
                            );
                            amount += party.attendees[newest_guest].tagalongs;
                            party.state = match amount {
                                1 => IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                },
                                2.. => IncomingGuest {
                                    amount: amount - 1,
                                    greet,
                                },
                                0 => unreachable!(),
                            };
                            if check_for_party_end_conditions(
                                &mut party,
                                house_is_full,
                                rolodex_is_empty,
                                available_full_house_abilities,
                                replenishes_available,
                            ) {
                                refresh!(party boxed_message);
                                break 'ongoing_party;
                            }
                            continue 'ongoing_party;
                        }
                    }
                    _ => {}
                }

                if (house_is_full || rolodex_is_empty)
                    && (available_full_house_abilities || replenishes_available)
                {
                    boxed_message = "Party is full, but you still can use some abilities!";
                }
                if check_for_party_end_conditions(
                    &mut party,
                    house_is_full,
                    rolodex_is_empty,
                    available_full_house_abilities,
                    replenishes_available,
                ) {
                    break 'ongoing_party;
                }

                'party_input: loop {
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'party_input;
                    }
                    match input.trim() {
                        "h" => {
                            boxed_message = "";
                            party.state = IncomingGuest {
                                amount: 1,
                                greet: false,
                            };
                            break 'party_input;
                        }
                        "r" => {
                            let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                            let mut attendees_view: Vec<&Guest> = party.attendees.iter().collect();
                            let mut booted_view: Vec<&Guest> = player.booted.iter().collect();
                            if let Some(peek) = &party.peek_slot {
                                rolodex_view.push(&peek)
                            };
                            if let Some(banned) = &player.banned.guest {
                                booted_view.push(&banned)
                            };
                            rolodex_view.sort_by_key(|guest| {
                                (
                                    guest.sort_value,
                                    Reverse(*guest.popularity),
                                    Reverse(*guest.cash),
                                )
                            });
                            attendees_view.sort_by_key(|guest| {
                                (
                                    guest.sort_value,
                                    Reverse(*guest.popularity),
                                    Reverse(*guest.cash),
                                )
                            });
                            booted_view.sort_by_key(|guest| {
                                (
                                    guest.sort_value,
                                    Reverse(*guest.popularity),
                                    Reverse(*guest.cash),
                                )
                            });
                            clear().unwrap();
                            println!("Player {}", player.id + 1);
                            let mut i = 1;
                            println!("The following contacts can still show up to the party:");
                            for r in rolodex_view {
                                println!("{i:>2}) {}", display_guest(r));
                                i += 1;
                            }
                            if attendees_view.len() > 0 {
                                println!("\nThe following contacts have already showed up to the party:");
                                for a in attendees_view {
                                    println!("{i:>2}) {}", display_guest(a));
                                    i += 1;
                                }
                            }
                            if booted_view.len() > 0 {
                                println!("\nThe following contacts cannot show up to the party today:");
                                for b in booted_view {
                                    println!("{i:>2}) {}", display_guest(b));
                                    i += 1;
                                }
                            }
                            pause_for_enter("\nPress \"Enter\" to go back to the party...");
                            break 'party_input;
                        }
                        "e" => {
                            if party.attendees.len() > 0
                                || (rolodex_is_empty && party.peek_slot.is_none())
                            {
                                party.state = EndedSuccessfully;
                                break 'party_input;
                            } else {
                                refresh!(party "Don't end the party yet! This place is dead!");
                                continue 'party_input;
                            }
                        }
                        "b" => {
                            if party.peek_slot.is_some() {
                                player.booted.push(party.peek_slot.take().unwrap());
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                break 'party_input;
                            } else {
                                refresh!(party "No one is at the front door!");
                                continue 'party_input;
                            }
                        }
                        i if i.parse::<usize>().map_or(false, |n| {
                            (1..=34).contains(&n)
                                && n <= *party.capacity as usize
                                && n <= party.attendees.len()
                        }) =>
                        {
                            let idx = i.parse::<usize>().unwrap() - 1;
                            if party.attendees[idx].ability_type == NoAbility {
                                refresh!(party "This guest does not have an ability. Please select a different guest to use their ability.");
                                continue 'party_input;
                            }
                            if party.attendees[idx].ability_stock < 1 {
                                refresh!(party "This guest's ability isn't available. Please select a different guest to use their ability.");
                                continue 'party_input;
                            }
                            party.ability_state = true;
                            match party.attendees[idx].ability_type {
                                Evac => {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    player.rolodex.extend(party.attendees.drain(0..));
                                    if let Some(peek) = party.peek_slot.take() {
                                        player.rolodex.push(peek);
                                    }
                                    let mut rng = thread_rng();
                                    player.rolodex.shuffle(&mut rng);
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    break 'party_input;
                                }
                                Cheer => {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    for p in party
                                        .attendees
                                        .iter_mut()
                                        .filter(|g| g.ability_type != Cheer)
                                    {
                                        p.ability_stock = p.ability_base;
                                    }
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    break 'party_input;
                                }
                                Quench => {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    for p in party.attendees.iter_mut() {
                                        p.trouble = false;
                                    }
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    break 'party_input;
                                }
                                Peek => match (&party.peek_slot, rolodex_is_empty) {
                                    (Some(_), _) => {
                                        "Someone is already at the front door!";
                                        continue 'party_input;
                                    }
                                    (None, true) => {
                                        refresh!(party "Rolodex is empty, no one left to invite!");
                                        continue 'party_input;
                                    }
                                    (None, false) => {
                                        party.attendees[idx].ability_stock -= 1;
                                        party.ability_state = false;
                                        party.peek_slot = Some(player.rolodex.pop().unwrap());
                                        break 'party_input;
                                    }
                                },
                                Shutter(mut target) => {
                                    refresh!(party "Select a guest to Photograph.");
                                    'shutter_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n <= party.attendees.len()
                                            }) =>
                                            {
                                                party.attendees[idx].ability_stock -= 1;
                                                party.ability_state = false;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                player.add_pop_from_guest(
                                                    *party.attendees[target].popularity,
                                                );
                                                player.add_cash_from_guest(
                                                    *party.attendees[target].cash,
                                                );
                                                player.add_pop_from_guest((party.attendees
                                                    [target]
                                                    .bonus_pop)(
                                                    &party
                                                ));
                                                if party.attendees[target].guest_type == DANCER {
                                                    player.add_pop_from_guest(min(
                                                        16,
                                                        party
                                                            .attendees
                                                            .iter()
                                                            .filter(|a| a.guest_type == DANCER)
                                                            .count()
                                                            .pow(2)
                                                            as i8,
                                                    ))
                                                };
                                                player.add_cash_from_guest((party.attendees
                                                    [target]
                                                    .bonus_cash)(
                                                    &party
                                                ));
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.ability_state = false;
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'shutter_input;
                                            }
                                        }
                                    }
                                }
                                Style(mut target) => {
                                    refresh!(party "Select a guest to Style.");
                                    'style_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n <= party.attendees.len()
                                            }) =>
                                            {
                                                party.attendees[idx].ability_stock -= 1;
                                                party.ability_state = false;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                party.attendees[target].popularity += 1;
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.ability_state = false;
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'style_input;
                                            }
                                        }
                                    }
                                }
                                Boot(mut target) => {
                                    refresh!(party "Select a guest to Kick from the party.");
                                    'boot_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n <= party.attendees.len()
                                            }) =>
                                            {
                                                party.attendees[idx].ability_stock -= 1;
                                                party.ability_state = false;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                player.booted.push(party.attendees[target].clone());
                                                party.attendees.remove(target);
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.ability_state = false;
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'boot_input;
                                            }
                                        }
                                    }
                                }
                                StarSwap(mut target) => match (
                                    party.attendees.iter().filter(|a| *a.stars > 0).count() > 0,
                                    player.rolodex.iter().filter(|a| *a.stars > 0).count() > 0,
                                ) {
                                    (true, _) | (_, true) => {
                                        refresh!(party "Select a guest to Swap Out.");
                                        'star_swap_input: loop {
                                            let mut input = String::new();
                                            if let Err(e) = stdin().read_line(&mut input) {
                                                eprintln!("Error reading input: {}", e);
                                                continue 'party_input;
                                            }
                                            match input.trim() {
                                                i if i.parse::<usize>().map_or(false, |n| {
                                                    (1..=34).contains(&n)
                                                        && n <= party.attendees.len()
                                                }) =>
                                                {
                                                    party.attendees[idx].ability_stock -= 1;
                                                    target = i.parse::<usize>().unwrap() - 1;
                                                    let mut replacement: Guest = Guest::default();
                                                    let goes_away: Guest;
                                                    for r in 0..player.rolodex.len() {
                                                        if *party.attendees[target].stars == 0 {
                                                            if player
                                                                .rolodex
                                                                .iter()
                                                                .filter(|g| *g.stars != 0)
                                                                .count()
                                                                == 0
                                                            {
                                                                refresh!(party "Cannot swap star guest for non-star guest because your rolodex has no available non-star guests.");
                                                                continue 'star_swap_input;
                                                            }
                                                            if *player.rolodex[r].stars != 0 {
                                                                replacement =
                                                                    player.rolodex[r].clone();
                                                                player.rolodex.remove(r);
                                                                break;
                                                            }
                                                        } else {
                                                            if player
                                                                .rolodex
                                                                .iter()
                                                                .filter(|g| *g.stars == 0)
                                                                .count()
                                                                == 0
                                                            {
                                                                refresh!(party "Cannot swap non-star guest for star guest because your rolodex has no available star guests.");
                                                                continue 'star_swap_input;
                                                            }
                                                            if *player.rolodex[r].stars == 0 {
                                                                replacement =
                                                                    player.rolodex[r].clone();
                                                                player.rolodex.remove(r);
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    goes_away = party.attendees[target].clone();
                                                    party.attendees.remove(target);
                                                    party.attendees.insert(target, replacement);
                                                    player.rolodex.push(goes_away);
                                                    party.state = IncomingGuest {
                                                        amount: party.attendees[target].tagalongs,
                                                        greet: false,
                                                    };
                                                    party.ability_state = false;
                                                    boxed_message = "";
                                                    break 'party_input;
                                                }
                                                "n" => {
                                                    party.state = IncomingGuest {
                                                        amount: 0,
                                                        greet: false,
                                                    };
                                                    party.ability_state = false;
                                                    boxed_message = "";
                                                    break 'party_input;
                                                }
                                                _ => {
                                                    println!("Invalid input.");
                                                    continue 'star_swap_input;
                                                }
                                            }
                                        }
                                    }
                                    (false, false) => {
                                        refresh!(party "There are neither any star guests in the rolodex nor the party.");
                                        party.ability_state = false;
                                        continue 'party_input;
                                    }
                                },
                                LoveArrow(mut target) => {
                                    if party.attendees.len() < 2 {
                                        refresh!(party "At least 2 partygoers need to be paired up by a Cupid's Arrow.");
                                        continue 'party_input;
                                    }
                                    refresh!(party "Select a pair of people who are directly next to each other and will leave together by selecting the person with the lower of the 2 positions.");
                                    'love_arrow_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n <= party.attendees.len() - 1
                                            }) =>
                                            {
                                                party.attendees[idx].ability_stock -= 1;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                player.booted.push(party.attendees[target].clone());
                                                player
                                                    .booted
                                                    .push(party.attendees[target + 1].clone());
                                                party.attendees.remove(target);
                                                party.attendees.remove(target);
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                party.ability_state = false;
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                party.ability_state = false;
                                                boxed_message = "";
                                                break 'party_input;
                                            }
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                n == *party.capacity as usize
                                            }) =>
                                            {
                                                refresh!(party "If you want to pair-kick the two guests at the back, select the guest with the lower of the 2 positional numbers.");
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'love_arrow_input;
                                            }
                                        }
                                    }
                                }
                                Greet => match (house_is_full, rolodex_is_empty) {
                                    (true, _) => {
                                        refresh!(party "The house is full, cannot invite a new guest!");
                                        party.ability_state = false;
                                        continue 'party_input;
                                    }
                                    (_, true) => {
                                        refresh!(party "Rolodex is empty, no one left to invite!");
                                        party.ability_state = false;
                                        continue 'party_input;
                                    }
                                    (false, false) => {
                                        party.attendees[idx].ability_stock -= 1;
                                        party.state = IncomingGuest {
                                            amount: 1,
                                            greet: true,
                                        };
                                        party.ability_state = false;
                                        break 'party_input;
                                    }
                                },
                                Summoning => match (house_is_full, rolodex_is_empty) {
                                    (true, _) => {
                                        refresh!(party "The house is full, cannot invite a new guest!");
                                        party.ability_state = false;
                                        continue 'party_input;
                                    }
                                    (_, true) => {
                                        refresh!(party "Rolodex is empty, no one left to invite!");
                                        party.ability_state = false;
                                        continue 'party_input;
                                    }
                                    (false, false) => {
                                        refresh!(party "Select a contact to invite to the party.");
                                        'summoning_input: loop {
                                            player.rolodex.sort_by_key(|guest| {
                                                (
                                                    guest.sort_value,
                                                    Reverse(*guest.popularity),
                                                    Reverse(*guest.cash),
                                                )
                                            });
                                            println!("\nRolodex:\n");
                                            for contact_num in 0..player.rolodex.len() {
                                                println!(
                                                    "{:>2}) {}",
                                                    contact_num + 1,
                                                    display_guest(&player.rolodex[contact_num])
                                                );
                                            }
                                            let mut input = String::new();
                                            if let Err(e) = stdin().read_line(&mut input) {
                                                eprintln!("Error reading input: {}", e);
                                                continue 'party_input;
                                            }
                                            match input.trim() {
                                                i if i.parse::<usize>().map_or(false, |n| {
                                                    (1..=player.rolodex.len()).contains(&n)
                                                }) =>
                                                {
                                                    party.attendees[idx].ability_stock -= 1;
                                                    let target = i.parse::<usize>().unwrap() - 1;
                                                    party
                                                        .attendees
                                                        .push(player.rolodex[target].clone());
                                                    player.rolodex.remove(target);
                                                    let mut rng = thread_rng();
                                                    player.rolodex.shuffle(&mut rng);
                                                    party.state = IncomingGuest {
                                                        amount: party.attendees
                                                            [party.attendees.len() - 1]
                                                            .tagalongs,
                                                        greet: false,
                                                    };
                                                    party.ability_state = false;
                                                    boxed_message = "";
                                                    break 'party_input;
                                                }
                                                "n" => {
                                                    let mut rng = thread_rng();
                                                    player.rolodex.shuffle(&mut rng);
                                                    party.state = IncomingGuest {
                                                        amount: 0,
                                                        greet: false,
                                                    };
                                                    party.ability_state = false;
                                                    boxed_message = "";
                                                    break 'party_input;
                                                }
                                                _ => {
                                                    println!("Invalid input.");
                                                    continue 'summoning_input;
                                                }
                                            }
                                        }
                                    }
                                },
                                NoAbility => unreachable!(),
                            }
                        }
                        _ => {
                            println!("Invalid Input {}", input.trim());
                            continue 'party_input;
                        }
                    }
                }
            }

            match party.state {
                TooMuchTrouble => {
                    refresh!(party "Oh no! The cops have shown up! Select a guest to take the blame!");
                    ban_guest(player, &mut party);
                }
                Overcrowded => {
                    refresh!(party "Oh no! The fire marshal has shown up! Select a guest to take the blame!");
                    ban_guest(player, &mut party);
                }
                EndedSuccessfully => {
                    player.end_of_party_score_guests(&party);
                    if party.attendees.iter().filter(|a| *a.stars == 1).count()
                        - party.attendees.iter().filter(|a| *a.stars == -1).count()
                        >= star_guest_arrivals_for_win
                    {
                        victories[player.id] = true;
                        boxed_message = "You threw the Ultimate Party! Win!"
                    } else {
                        boxed_message = "Party Ended Successfully! Press \"Enter\" to continue..."
                    }
                    refresh!(party boxed_message);
                    pause_for_enter("");
                }
                _ => unreachable!(),
            }
            player.rolodex.extend(party.attendees.drain(0..));
            if player.banned.guest.is_some() && player.banned.already_served_time {
                player.rolodex.push(player.banned.guest.take().unwrap());
            }
            player.banned.already_served_time = true;
            if let Some(peek) = party.peek_slot.take() {
                player.rolodex.push(peek);
            }

            'store: {
                if victories[0..=player.id].iter().any(|v| *v) {
                    break 'store;
                }
                let mut boxed_message: &str = "";
                'store_input: loop {
                    refresh!(store boxed_message);
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'store_input;
                    }
                    match input.trim() {
                        "r" => {
                            let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                            
                            rolodex_view.sort_by_key(|guest| {
                                (
                                    guest.sort_value,
                                    Reverse(*guest.popularity),
                                    Reverse(*guest.cash),
                                )
                            });
                            clear().unwrap();
                            println!("Player {}", player.id + 1);
                            let mut i = 1;
                            println!("The following contacts can show up to the party tomorrow:");
                            for r in rolodex_view {
                                println!("{i:>2}) {}", display_guest(r));
                                i += 1;
                            }
                            if let Some(b) = &player.banned.guest {
                                println!("\nThe following contacts cannot show up to the party tomorrow:");
                                println!("{i:>2}) {}", display_guest(b));
                            }
                            pause_for_enter("\nPress \"Enter\" to go back to the store...");
                            continue 'store_input;
                        }
                        "c" => {
                            let cost_of_expansion = match *player.capacity {
                                5..=15 => *player.capacity - 3,
                                16..=33 => 12,
                                34.. => 0,
                                ..=4 => unreachable!(),
                            };
                            if cost_of_expansion == 0 {
                                boxed_message = "Player's capacity is maxed out!";
                            } else if *player.cash >= cost_of_expansion {
                                player.cash -= cost_of_expansion;
                                player.capacity += 1;
                                boxed_message = "";
                            } else {
                                boxed_message = "Not enough cash to upgrade capacity!";
                            }
                            continue 'store_input;
                        }
                        "e" => break 'store,
                        i if i
                            .parse::<usize>()
                            .map_or(false, |n| (1..=store.len()).contains(&n)) =>
                        {
                            let idx = i.parse::<usize>().unwrap() - 1;
                            if store[idx].1 == 0.0 {
                                boxed_message = "This guest is no longer available.";
                                continue 'store_input;
                            } else if *player.popularity < store[idx].0.cost as i8 {
                                boxed_message = "Cannot afford this guest.";
                                continue 'store_input;
                            } else {
                                player.popularity -= store[idx].0.cost as i8;
                                store[idx].1 -= 1.0;
                                player.rolodex.push(store[idx].0.clone());
                                boxed_message = "";
                                continue 'store_input;
                            }
                        }
                        _ => println!("Invalid Input."),
                    }
                }
            }
        }

        match num_players {
            0 => unreachable!(),
            1 => {
                if day_count == 25 || victories[0] {
                    break 'game;
                }
            }
            2.. => {
                if day_count == 100 || victories.iter().any(|v| *v) {
                    break 'game;
                }
            }
        }

        day_count += 1;
    }

    clear().unwrap();

    match victories.len() {
        1 => match victories[0] {
            true => println!("You threw the Ultimate Party! Win!"),
            false => println!("You lose!"),
        },
        2.. => {
            if victories.iter().filter(|v| **v).count() > 1 {
                for i in 0..victories.len() {
                    match victories[i] {
                        true => println!("Player {} threw the Ultimate Party! Win!", i + 1),
                        false => println!("Player {} loses!", i + 1),
                    }
                }
            } else {
                for i in 0..victories.len() {
                    match victories[i] {
                        true => println!("Player {} is the Party Master! Win!", i + 1),
                        false => {}
                    }
                    println!("Everyone else loses! All of their vibes were way off!")
                }
            }
        },
        0 => unreachable!()
    }
    println!();
}
