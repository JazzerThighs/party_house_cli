mod clampedi8;
mod display;
mod guest;
mod init;
mod party;
mod party_input;
mod player;

use clearscreen::clear;
use rand::{rng, seq::SliceRandom};
use std::{
    cmp::{min, Reverse},
    io::stdin,
};
use {
    display::*,
    guest::{Guest, GuestType, GuestType::*},
    init::*,
    party::{PartyState::*, *},
    party_input::*,
    player::*,
};

fn main() {
    let num_players: usize = get_num_players();
    let (mut players, star_guest_arrivals_for_win): (Vec<Player>, usize) =
        init_players(num_players);
    let mut store: Vec<(Guest, f32)> = init_scenerio(num_players);
    let mut day_count: usize = 1;
    let mut victories: Vec<bool> = vec![false; num_players];

    'game: loop {
        clear().unwrap();
        'take_turns: for player in players.iter_mut() {
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
            let mut party = Party {
                capacity: player.capacity.clone(),
                stars_to_win: star_guest_arrivals_for_win,
                ..Default::default()
            };
            let mut boxed_message: String = "".to_string();
            let mut tagalong_bringer: GuestType = GuestType::default();
            macro_rules! refresh {
                (party $message:expr) => {
                    boxed_message = $message;
                    party_display(&party, player, &victories, day_count, &boxed_message);
                };
                (store $message:expr) => {
                    boxed_message = $message;
                    store_display(
                        &store,
                        player,
                        &victories,
                        day_count,
                        &star_guest_arrivals_for_win,
                        &boxed_message,
                    );
                };
            }
            'ongoing_party: loop {
                let mut random_number = rng();
                player.rolodex.shuffle(&mut random_number);
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
                                player
                                    .add_pop_from_guest(*party.attendees[newest_guest].popularity);
                                if *party.attendees[newest_guest].cash >= 0 {
                                    player.add_cash_from_guest(*party.attendees[newest_guest].cash);
                                }
                                player
                                    .add_pop_from_guest((party.attendees[newest_guest].bonus_pop)(
                                        &party,
                                    ));
                                if party.attendees[newest_guest].guest_type == DANCER {
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
                                player.add_cash_from_guest((party.attendees[newest_guest]
                                    .bonus_cash)(
                                    &party
                                ));
                                if *party.attendees[newest_guest].cash < 0 {
                                    player.add_cash_from_guest(*party.attendees[newest_guest].cash);
                                }
                            }
                            (party.attendees[newest_guest].entrance_effect)(
                                &mut party.attendees[newest_guest],
                            );
                            amount += party.attendees[newest_guest].tagalongs;
                            if party.attendees[newest_guest].tagalongs >= 1 {
                                tagalong_bringer = party.attendees[newest_guest].guest_type.clone();
                            }

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
                            match party.state {
                                IncomingGuest { amount, greet: _ } if amount >= 1 => {
                                    refresh!(party format!("{} brought another guest", guest_type_display(&tagalong_bringer)));
                                    boxed_message = "".to_string();
                                    pause_for_enter("Press enter to continue...");
                                }
                                _ => {}
                            }
                            continue 'ongoing_party;
                        }
                    }
                    _ => {}
                }

                if (house_is_full || rolodex_is_empty)
                    && (available_full_house_abilities || replenishes_available)
                {
                    boxed_message =
                        "Party is full, but you still can use some abilities!".to_string();
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

                boxed_message = party_input(
                    player,
                    &mut party,
                    &house_is_full,
                    &rolodex_is_empty,
                    &victories,
                    day_count,
                    boxed_message,
                );
            }

            match party.state {
                TooMuchTrouble => {
                    refresh!(party "Oh no! The cops have shown up! Select a guest to take the blame!".to_string());
                    let banned = ban_guest(player, &mut party);
                    refresh!(party format!("{} has been banned for 1 day.", guest_type_display(&banned)));
                    pause_for_enter("");
                }
                Overcrowded => {
                    refresh!(party "Oh no! The fire marshal has shown up! Select a guest to take the blame!".to_string());
                    let banned = ban_guest(player, &mut party);
                    refresh!(party format!("{} has been banned for 1 day.", guest_type_display(&banned)));
                    pause_for_enter("");
                }
                #[allow(non_snake_case)]
                EndedSuccessfully => {
                    player.add_pop_from_guest(party.attendees.iter().map(|a| *a.popularity).sum());
                    player.add_cash_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| *a.cash >= 0)
                            .map(|a| *a.cash)
                            .sum(),
                    );
                    player.add_cash_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| *a.cash < 0)
                            .map(|a| *a.cash)
                            .sum(),
                    );
                    player.add_pop_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| (a.bonus_pop)(&party) >= 0)
                            .filter(|a| a.guest_type != GuestType::DANCER)
                            .map(|a| (a.bonus_pop)(&party))
                            .sum(),
                    );
                    player.add_pop_from_guest(min(
                        16,
                        party
                            .attendees
                            .iter()
                            .filter(|a| a.guest_type == GuestType::DANCER)
                            .count()
                            .pow(2) as i8,
                    ));
                    player.add_pop_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| (a.bonus_pop)(&party) < 0)
                            .map(|a| (a.bonus_pop)(&party))
                            .sum(),
                    );
                    player.add_cash_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| (a.bonus_cash)(&party) >= 0)
                            .map(|a| (a.bonus_cash)(&party))
                            .sum(),
                    );
                    
                    player.add_cash_from_guest(
                        party
                            .attendees
                            .iter()
                            .filter(|a| (a.bonus_cash)(&party) < 0)
                            .map(|a| (a.bonus_cash)(&party))
                            .sum(),
                    );

                    if party.attendees.iter().filter(|a| *a.stars == 1).count()
                        - party.attendees.iter().filter(|a| *a.stars == -1).count()
                        >= star_guest_arrivals_for_win
                    {
                        victories[player.id] = true;
                        refresh!(party "You threw the Ultimate Party! Win!".to_string());
                        pause_for_enter("");
                        continue 'take_turns;
                    } else {
                        refresh!(party "Party Ended Successfully!".to_string());
                    }

                    display_end_of_party_info(&party);
                }
                _ => unreachable!(),
            }

            handle_end_of_party(player, &mut party);

            boxed_message = "".to_string();

            'store: {
                if victories[0..=player.id].iter().any(|v| *v)
                    || (victories.len() == 1 && day_count == 25)
                {
                    break 'store;
                }
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
                                boxed_message = "Player's capacity is maxed out!".to_string();
                            } else if *player.cash >= cost_of_expansion {
                                player.cash -= cost_of_expansion;
                                player.capacity += 1;
                                boxed_message = "".to_string();
                            } else {
                                boxed_message = "Not enough cash to upgrade capacity!".to_string();
                            }
                            continue 'store_input;
                        }
                        "e" => break 'store,
                        "i" => {
                            display_information();
                            pause_for_enter("\nPress \"Enter\" to go back to the store...");
                            continue 'store_input;
                        }
                        i if i
                            .parse::<usize>()
                            .map_or(false, |n| (1..=store.len()).contains(&n)) =>
                        {
                            let idx = i.parse::<usize>().unwrap() - 1;
                            if store[idx].1 == 0.0 {
                                boxed_message = "This guest is no longer available.".to_string();
                                continue 'store_input;
                            } else if *player.popularity < store[idx].0.cost as i8 {
                                boxed_message = "Cannot afford this guest.".to_string();
                                continue 'store_input;
                            } else {
                                player.popularity -= store[idx].0.cost as i8;
                                store[idx].1 -= 1.0;
                                player.rolodex.push(store[idx].0.clone());
                                boxed_message = "".to_string();
                                continue 'store_input;
                            }
                        }
                        _ => {
                            refresh!(store "Invalid Input.".to_string());
                        }
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
            false => println!("You lose! Your vibes were off!"),
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
                        false => println!("Player {}'s vibes were off! Lose!", i + 1)
                    }
                }
            }
        }
        0 => unreachable!(),
    }
    println!();
}
