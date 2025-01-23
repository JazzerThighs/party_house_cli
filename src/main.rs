#![allow(
    dead_code,
    unused_mut,
    unused_imports,
    unused_variables,
    unused_labels,
    unused_assignments,
    unreachable_code
)]
mod clampedi8;
mod display;
mod guest;
mod init;
mod party;
mod player;

use clearscreen::clear;
use guest::Guest;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{
    cmp::{min, Reverse},
    io::stdin,
};
use {
    display::*,
    guest::{AbilityType::*, FullHouseAbilityCondition::*, GuestType::*},
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
            macro_rules! refresh {
                (party $boxed_message:expr) => {
                    party_display(&party, player, &victories, $boxed_message.to_string());
                };
                (store $boxed_message:expr) => {
                    store_display(store, player, $boxed_message.to_string())
                };
            }
            init_party(&mut party, player, star_guest_arrivals_for_win);
            refresh!(party "");
            'ongoing_party: loop {
                let (
                    house_is_full,
                    rolodex_is_empty,
                    available_full_house_abilities,
                    replenishes_available,
                ) = get_party_state(&party, player);
                'evaluate_party: {
                    match party.state.clone() {
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
                                    player.add_pop_from_guest(
                                        *party.attendees[newest_guest].popularity,
                                    );
                                    player.add_cash_from_guest(*party.attendees[newest_guest].cash);
                                    player.add_pop_from_guest((party.attendees[newest_guest]
                                        .bonus_pop)(
                                        &party
                                    ));
                                    if party.attendees[newest_guest].guest_type == DANCER {
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
                                    player.add_cash_from_guest((party.attendees[newest_guest]
                                        .bonus_cash)(
                                        &party
                                    ));
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
                                    house_is_full || rolodex_is_empty,
                                ) {
                                    break 'ongoing_party;
                                }
                                continue 'ongoing_party;
                            }
                        }

                        AbilityState(a) => match a {
                            Shutter(target) => {
                                player.add_pop_from_guest(*party.attendees[target].popularity);
                                player.add_cash_from_guest(*party.attendees[target].cash);
                                player.add_pop_from_guest((party.attendees[target].bonus_pop)(
                                    &party,
                                ));
                                if party.attendees[target].guest_type == DANCER {
                                    player.add_pop_from_guest(min(
                                        16,
                                        party
                                            .attendees
                                            .iter()
                                            .filter(|a| a.guest_type == DANCER)
                                            .count()
                                            .pow(2) as i8,
                                    ))
                                };
                                player.add_cash_from_guest((party.attendees[target].bonus_cash)(
                                    &party,
                                ));
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            Style(target) => {
                                party.attendees[target].popularity += 1;
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            StarSwap(target) => {
                                todo!();
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            Boot(target) => {
                                player.booted.push(party.attendees[target].clone());
                                party.attendees.remove(target);
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            LoveArrow(target) => {
                                player.booted.push(party.attendees[target].clone());
                                player.booted.push(party.attendees[target + 1].clone());
                                party.attendees.remove(target);
                                party.attendees.remove(target);
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }

                            Cheer | Evac | Quench | Peek | Greet => {
                                let (mut amount, mut greet): (u8, bool) = (0, false);
                                match a {
                                    Cheer => {
                                        for p in party
                                            .attendees
                                            .iter_mut()
                                            .filter(|g| g.ability_type != Cheer)
                                        {
                                            p.ability_stock = p.ability_base;
                                        }
                                    }
                                    Evac => {
                                        player.rolodex.extend(party.attendees.drain(0..));
                                        if let Some(peek) = party.peek_slot.take() {
                                            player.rolodex.push(peek);
                                        }
                                        let mut rng = thread_rng();
                                        player.rolodex.shuffle(&mut rng);
                                    }
                                    Quench => {
                                        for p in party.attendees.iter_mut() {
                                            p.trouble = false;
                                        }
                                    }
                                    Peek => {
                                        party.peek_slot = Some(player.rolodex.pop().unwrap());
                                    }
                                    Greet => {
                                        amount = 1;
                                        greet = true;
                                    }
                                    _ => unreachable!(),
                                };
                                party.state = IncomingGuest { amount, greet };
                                continue 'ongoing_party;
                            }
                            Summoning => unreachable!(),
                            NoAbility => unreachable!(),
                        },

                        ViewingRolodex => {
                            let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                            let mut attendees_view: Vec<&Guest> = party.attendees.iter().collect();
                            let mut booted_view: Vec<&Guest> = player.booted.iter().collect();
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
                            todo!()
                        }

                        _ => {}
                    }

                    if (house_is_full || rolodex_is_empty)
                        && (available_full_house_abilities || replenishes_available)
                    {
                        party.state = FullHouseUnusedAbility
                    }
                    if check_for_party_end_conditions(&mut party, house_is_full || rolodex_is_empty)
                    {
                        break 'ongoing_party;
                    }
                    todo!()
                }

                'party_input: loop {
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'party_input;
                    }
                    match input.trim() {
                        "h" => {
                            party.state = IncomingGuest {
                                amount: 1,
                                greet: false,
                            };
                            break 'party_input;
                        }
                        "r" => {
                            party.state = ViewingRolodex;
                            break 'party_input;
                        }
                        "e" => {
                            if party.attendees.len() > 0
                                || (rolodex_is_empty && party.peek_slot.is_none())
                            {
                                party.state = EndedSuccessfully;
                                break 'party_input;
                            } else {
                                println!("Don't end the party yet! This place is dead!");
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
                                println!("No one is at the front door!");
                                continue 'party_input;
                            }
                        }
                        i if i.parse::<usize>().map_or(false, |n| {
                            (1..=34).contains(&n) && n <= *party.capacity as usize
                        }) =>
                        {
                            let idx = i.parse::<usize>().unwrap() - 1;
                            if party.attendees[idx].ability_stock < 1 {
                                println!("This guest's ability isn't available. Please select a different guest to use their ability.");
                                continue 'party_input;
                            }
                            party.attendee_ability_source = idx;
                            match party.attendees[idx].ability_type {
                                Evac | Cheer | Quench => {
                                    party.attendees[party.attendee_ability_source].ability_stock -=
                                        1;
                                    party.state = AbilityState(party.attendees[idx].ability_type);
                                    break 'party_input;
                                }
                                Peek => match (party.peek_slot, rolodex_is_empty) {
                                    (Some(_), _) => {
                                        println!("Someone is already at the front door!");
                                        continue 'party_input;
                                    }
                                    (None, true) => {
                                        println!("Rolodex is empty, no one left to invite!");
                                        continue 'party_input;
                                    }
                                    (None, false) => {
                                        party.attendees[party.attendee_ability_source]
                                            .ability_stock -= 1;
                                        party.state = AbilityState(Peek);
                                        break 'party_input;
                                    }
                                },
                                Shutter(mut target) | Style(mut target) | Boot(mut target) => {
                                    'target_one_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n <= *party.capacity as usize
                                            }) =>
                                            {
                                                party.attendees[party.attendee_ability_source]
                                                    .ability_stock -= 1;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                party.state =
                                                    AbilityState(party.attendees[idx].ability_type);
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                break 'party_input;
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'target_one_input;
                                            }
                                        }
                                    }
                                }
                                StarSwap(mut target) => {
                                    match (
                                        party.attendees.iter().filter(|a| *a.stars > 0).count() > 0,
                                        player.rolodex.iter().filter(|a| *a.stars > 0).count() > 0,
                                    ) {
                                        (true, _) | (_, true) => 'target_two_input: loop {
                                            let mut input = String::new();
                                            if let Err(e) = stdin().read_line(&mut input) {
                                                eprintln!("Error reading input: {}", e);
                                                continue 'party_input;
                                            }
                                            match input.trim() {
                                                i if i.parse::<usize>().map_or(false, |n| {
                                                    (1..=34).contains(&n)
                                                        && n <= *party.capacity as usize
                                                }) =>
                                                {
                                                    party.attendees
                                                        [party.attendee_ability_source]
                                                        .ability_stock -= 1;
                                                    target = i.parse::<usize>().unwrap() - 1;
                                                    party.state = AbilityState(
                                                        party.attendees[idx].ability_type,
                                                    );
                                                    break 'party_input;
                                                }
                                                "n" => {
                                                    party.state = IncomingGuest {
                                                        amount: 0,
                                                        greet: false,
                                                    };
                                                    break 'party_input;
                                                }
                                                _ => {
                                                    println!("Invalid input.");
                                                    continue 'target_two_input;
                                                }
                                            }
                                        },
                                        (false, false) => {
                                            println!("There are neither any star guests in the rolodex nor the party.");
                                            continue 'party_input;
                                        }
                                    }
                                }
                                LoveArrow(mut target) => {
                                    if party.attendees.len() < 2 {
                                        println!("At least 2 partygoers need to be paired up by a Cupid's Arrow.");
                                        continue 'party_input;
                                    }
                                    'target_three_input: loop {
                                        let mut input = String::new();
                                        if let Err(e) = stdin().read_line(&mut input) {
                                            eprintln!("Error reading input: {}", e);
                                            continue 'party_input;
                                        }
                                        println!("Select a pair of people who are directly next to each other and will leave together by selecting the person with the lower of the 2 positions.");
                                        match input.trim() {
                                            i if i.parse::<usize>().map_or(false, |n| {
                                                (1..=34).contains(&n)
                                                    && n < *party.capacity as usize
                                            }) =>
                                            {
                                                party.attendees[party.attendee_ability_source]
                                                    .ability_stock -= 1;
                                                target = i.parse::<usize>().unwrap() - 1;
                                                party.state =
                                                    AbilityState(party.attendees[idx].ability_type);
                                                break 'party_input;
                                            }
                                            "n" => {
                                                party.state = IncomingGuest {
                                                    amount: 0,
                                                    greet: false,
                                                };
                                                break 'party_input;
                                            }
                                            _ => {
                                                println!("Invalid input.");
                                                continue 'target_three_input;
                                            }
                                        }
                                    }
                                }
                                Greet | Summoning => match (house_is_full, rolodex_is_empty) {
                                    (true, _) => {
                                        println!("The house is full, cannot invite a new guest!");
                                        continue 'party_input;
                                    }
                                    (_, true) => {
                                        println!("Rolodex is empty, no one left to invite!");
                                        continue 'party_input;
                                    }
                                    (false, false) => match party.attendees[idx].ability_type {
                                        Greet => {
                                            party.attendees[party.attendee_ability_source]
                                                .ability_stock -= 1;
                                            party.state =
                                                AbilityState(party.attendees[idx].ability_type);
                                            break 'party_input;
                                        }
                                        Summoning => 'summoning_input: loop {
                                            println!("Select a contact to invite to the party:");
                                            player.rolodex.sort_by_key(|guest| {
                                                (
                                                    guest.sort_value,
                                                    Reverse(*guest.popularity),
                                                    Reverse(*guest.cash),
                                                )
                                            });
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
                                                    party.attendees
                                                        [party.attendee_ability_source]
                                                        .ability_stock -= 1;
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
                                                    break 'party_input;
                                                }
                                                "n" => {
                                                    let mut rng = thread_rng();
                                                    player.rolodex.shuffle(&mut rng);
                                                    party.state = IncomingGuest {
                                                        amount: 0,
                                                        greet: false,
                                                    };
                                                    break 'party_input;
                                                }
                                                _ => {
                                                    println!("Invalid input.");
                                                    continue 'summoning_input;
                                                }
                                            }
                                        },
                                        _ => unreachable!(),
                                    },
                                },
                                NoAbility => {
                                    println!("This guest does not have an ability. Please select a different guest to use their ability.");
                                    continue 'party_input;
                                }
                            }
                        }
                        _ => {
                            println!("Invalid Input {}", input.trim());
                            continue 'party_input;
                        }
                    }
                }
            }

            'handle_party_end: {
                match party.state {
                    TooMuchTrouble => {
                        todo!(); // Cops Came
                    }
                    Overcrowded => {
                        todo!(); // Fire Marshal Came
                    }
                    EndedSuccessfully => {
                        player.end_of_party_score_guests(&party);
                        if party.attendees.iter().filter(|a| *a.stars == 1).count()
                            - party.attendees.iter().filter(|a| *a.stars == -1).count()
                            >= star_guest_arrivals_for_win
                        {
                            victories[player.id] = true;
                            todo!() // Show that the player won
                        }
                    }
                    _ => unreachable!(),
                }
                player.rolodex.extend(party.attendees.drain(0..));
                if let Some(peek) = party.peek_slot.take() {
                    player.rolodex.push(peek);
                }
            }

            'store: {
                if !&victories[0..=player.id + 1].iter().any(|v| *v) {
                    break 'store;
                }
                let mut boxed_message: String = "".to_string();
                'store_input: loop {
                    store_display(&store, player, &boxed_message);
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'store_input;
                    }
                    match input.trim() {
                        "r" => {
                            let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                            let mut booted_view: Vec<&Guest> = player.booted.iter().collect();
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
                            booted_view.sort_by_key(|guest| {
                                (
                                    guest.sort_value,
                                    Reverse(*guest.popularity),
                                    Reverse(*guest.cash),
                                )
                            });
                            todo!()
                        },
                        "c" => {
                            let cost_of_expansion = match *player.capacity {
                                5..=15 => *player.capacity - 3,
                                16..=33 => 12,
                                34.. => 0,
                                ..=4 => unreachable!()
                            };
                            if cost_of_expansion == 0 {
                                boxed_message = "Player's capacity is maxed out!".to_string();
                            } else if *player.cash >= cost_of_expansion {
                                player.capacity += 1;
                            } else {
                                boxed_message = "Not enough cash to upgrade capacity!".to_string();
                            }
                            continue 'store_input;
                        },
                        "e" => break 'store,
                        i if i.parse::<usize>().map_or(false, |n| (1..=store.len()).contains(&n)) => todo!(),
                        _ => println!("Invalid Input.")
                    }
                }
            }
        }

        'ending_check: {
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
    }

    clear().unwrap();
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
    println!();
}
