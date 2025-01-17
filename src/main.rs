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
mod store;

use clearscreen::clear;
use guest::Guest;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{cmp::min, io::stdin};
use {
    display::*,
    guest::{AbilityType::*, FullHouseAbilityCondition::*, GuestType::*},
    init::*,
    party::{PartyState::*, *},
    player::*,
    store::*,
};

fn main() {
    let num_players: usize = get_num_players();
    let (mut players, star_guest_arrivals_for_win): (Vec<Player>, usize) =
        init_players(num_players);
    let mut store: Store = init_scenerio(num_players);
    let mut day_count: usize = 1;
    let mut victories: Vec<bool> = vec![false; num_players];
    let mut party: Party = Party::default();

    'game: loop {
        clear().unwrap();
        for player in players.iter_mut() {
            init_party(&mut party, player, star_guest_arrivals_for_win);
            'ongoing_party: loop {
                macro_rules! refresh {
                    (party $boxed_message:expr) => {
                        party_display(party, player, victories, $boxed_message.to_string());
                    };
                    (store $boxed_message:expr) => {
                        store_display(store, player, $boxed_message.to_string())
                    };
                }
                'evaluate_party: {
                    let (
                        house_is_full,
                        rolodex_is_empty,
                        available_full_house_abilities,
                        replenishes_available,
                    ) = get_party_state(&party, player);

                    match (
                        party.state.clone(),
                        house_is_full,
                        rolodex_is_empty,
                        available_full_house_abilities,
                        replenishes_available,
                    ) {
                        (IncomingGuest { mut amount, greet }, _, _, _, _) if amount >= 1 => {
                            // Let a new guest into the party.
                            if player.rolodex.is_empty() {
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            } else {
                                party.attendees.push(player.rolodex.pop().unwrap());
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

                        (AbilityState(a), _, _, _, _) => match a {
                            Shutter | Style(_) | StarSwap | Boot | LoveArrow | Cheer => {
                                // get input to select the attendee that will be affected by the ability
                                todo!()
                            }
                            Evac => {
                                party.attendees[party.attendee_ability_source].ability_stock -= 1;
                                player.rolodex.extend(party.attendees.drain(0..));
                                let mut rng = thread_rng();
                                player.rolodex.shuffle(&mut rng);
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            Quench => {
                                party.attendees[party.attendee_ability_source].ability_stock -= 1;
                                for p in party.attendees.iter_mut() {
                                    p.trouble = false;
                                }
                                party.state = IncomingGuest {
                                    amount: 0,
                                    greet: false,
                                };
                                continue 'ongoing_party;
                            }
                            Peek | Greet | Summoning => {
                                let (mut amount, mut greet): (u8, bool) = (0, false);
                                if !(house_is_full || rolodex_is_empty) {
                                    match a {
                                        Peek => {
                                            if party.peek_slot.is_none() {
                                                party.attendees[party.attendee_ability_source]
                                                    .ability_stock -= 1;
                                                party.peek_slot =
                                                    Some(player.rolodex.pop().unwrap())
                                            } else {
                                                todo!()
                                            }
                                        }
                                        Greet => {
                                            party.attendees[party.attendee_ability_source]
                                                .ability_stock -= 1;
                                            amount = 1;
                                            greet = true;
                                        }
                                        Summoning => {
                                            // show a list of people from the rolodex who aren't banned
                                            todo!()
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                party.state = IncomingGuest { amount, greet };
                                continue 'ongoing_party;
                            }
                            NoAbility => unreachable!(),
                        },

                        (_, true, _, true, _)
                        | (_, true, _, _, true)
                        | (_, _, true, true, _)
                        | (_, _, true, _, true) => party.state = FullHouseUnusedAbility,

                        (ViewingRolodex, _, _, _, _) => {
                            let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                            let mut attendees_view: Vec<&Guest> = party.attendees.iter().collect();
                            let mut booted_view: Vec<&Guest> = player.booted.iter().collect();
                            // if let Some(banned) = &player.banned.guest { booted_view.push(&banned) };
                            rolodex_view.sort_by_key(|guest| guest.sort_value);
                            attendees_view.sort_by_key(|guest| guest.sort_value);
                            booted_view.sort_by_key(|guest| guest.sort_value);
                            todo!()
                        }

                        (_, _, _, _, _) => {
                            if check_for_party_end_conditions(
                                &mut party,
                                house_is_full || rolodex_is_empty,
                            ) {
                                break 'ongoing_party;
                            }
                        }
                    }
                }

                'party_input: loop {
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'party_input;
                    }
                    match input.trim() {
                        "h" => {
                            party.state = IncomingGuest { amount: 1, greet: false };
                            break 'party_input;
                        },
                        "r" => {
                            party.state = ViewingRolodex;
                            break 'party_input;
                        },
                        "e" => {
                            party.state = EndedSuccessfully;
                            break 'party_input;
                        },
                        i if i.parse::<u8>().map_or(false, |n| (1..=34).contains(&n)) => todo!(),
                        _ => println!("Invalid Input. Please input \"h\" to open the door, \"r\" to see your rolodex, \"e\" to end the party, or an integer from 1 to 34 to use an attendee's ability.")
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
                player.rolodex.extend(party.attendees.drain(0..))
            }

            'store: {
                if !&victories[0..=player.id + 1].iter().any(|v| *v) || store.done_shopping {
                    break 'store;
                }

                'store_input: loop {
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'store_input;
                    }
                    match input.trim() {
                        "c" => {
                            todo!();
                            continue 'store_input;
                        },
                        "r" => {
                            todo!();
                            continue 'store_input;
                        },
                        "e" => break 'store,
                        i if i.parse::<u8>().map_or(false, |n| (1..=13).contains(&n)) => todo!(),
                        _ => println!("Invalid Input. Please input \"c\" to increase the capacity of your house, \"r\" to see your rolodex, \"e\" to finish shopping, or an integer from 1 to 13 to add an available contact to your rolodex.")
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

    for i in 0..victories.len() {
        match victories[i] {
            true => println!("Player {} threw the Ultimate Party!", i + 1),
            false => println!("Player {} loses!", i + 1),
        }
    }
    println!();
}
