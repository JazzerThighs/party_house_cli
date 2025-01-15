#![allow(
    dead_code,
    unused_mut,
    unused_imports,
    unused_variables,
    unused_labels,
    unused_assignments
)]
#![allow(unreachable_code)]
mod clampedi8;
mod guest;
mod init;
mod party;
mod player;
mod store;

use clearscreen::clear;
use guest::FullHouseAbilityCondition;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{cmp::min, io::stdin};
use {
    guest::{AbilityType::*, FullHouseAbilityCondition::*, GuestType::*},
    init::*,
    party::{PartyAction::*, PartyState::*, *},
    store::*,
};

fn main() {
    let num_players = get_num_players();
    let (mut players, star_guest_arrivals_for_win) = init_players(num_players);
    let mut store = init_scenerio(num_players);
    let mut day_count = 1;
    let mut victories = vec![false; num_players];
    let mut party: Party;

    'game: loop {
        clear().unwrap();
        for player in players.iter_mut() {
            init_party(&mut party, player, star_guest_arrivals_for_win);

            'ongoing_party: loop {
                'party_display: {
                    clear().unwrap();
                    println!("Player {}, throw a party!", player.id);
                    if victories.iter().any(|v| *v) {
                        for v in victories.iter() {
                            if *v {
                                println!("Player {} won today!", player.id + 1)
                            };
                        }
                        println!("Last Chance!\n");
                    }
                    todo!()
                }

                'evaluate_party: {
                    let (no_more_guests_can_come_in, available_full_house_abilities, replenishes_available) = get_party_state(&party, player);

                    match (
                        party.state,
                        no_more_guests_can_come_in,
                        available_full_house_abilities,
                        replenishes_available,
                    ) {
                        (IncomingGuest { amount, greet }, _, _, _) if amount >= 1 => 'try_to_add_guests: {
                            // Let a new guest into the party.
                            if player.rolodex.is_empty() {
                                party.state = IncomingGuest { amount: 0, greet: false };
                                continue 'ongoing_party;
                            } else {
                                party.attendees.push(player.rolodex.pop().unwrap());
                                let mut scored_guest = &party.attendees[party.attendees.len() - 1];
                                if greet {
                                    player.add_pop_from_guest(*scored_guest.popularity);
                                    player.add_cash_from_guest(*scored_guest.cash);
                                    player.add_pop_from_guest((scored_guest.bonus_pop)(&party));
                                    if scored_guest.guest_type == DANCER {
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
                                    player.add_cash_from_guest((scored_guest.bonus_cash)(&party));
                                }
                                (scored_guest.entrance_effect)(&mut scored_guest);
                                amount += scored_guest.tagalongs;
                                party.state = match amount {
                                    1 => IncomingGuest{ amount: 0, greet: false },
                                    2.. => IncomingGuest{ amount: amount - 1, greet }
                                };
                                continue 'ongoing_party;
                            }
                        },
                        (AbilityState(a), _, _, _) => todo!(),
                        (_, true, true, _) | (_, true, _, true) => { party.state = FullHouseUnusedAbility },
                        (ViewingRolodex, _, _, _) => todo!(),
                        (_, _, _, _) => 'check_for_ending_conditions: {
                            if party.attendees.iter().filter(|g| g.trouble).count()
                                - party.attendees.iter().filter(|g| g.chill).count()
                                >= 3 
                            {
                                party.state = TooMuchTrouble;
                                break 'ongoing_party;
                            }
                            if party.attendees.len() > *party.capacity as usize {
                                party.state = Overcrowded;
                                break 'ongoing_party;
                            }
                            if no_more_guests_can_come_in {
                                party.state = EndedSuccessfully;
                                break 'ongoing_party;
                            }
                        }
                    }
                }

                'party_input: {
                    let next_state: PartyState;
                    'take_turn: loop {
                        let mut input = String::new();
                        if let Err(e) = stdin().read_line(&mut input) {
                            eprintln!("Error reading input: {}", e);
                            continue 'take_turn;
                        }
                        match input.trim() {
                            "h" => party.attendees.push(player.rolodex.pop().unwrap()),
                            "r" => next_state = ViewingRolodex,
                            i if i.parse::<u8>().map_or(false, |n| (1..=34).contains(&n)) => party.try_use_attendee_ability(i),
                            _ => println!("Invalid Input. Please input \"h\" to open the door, \"r\" to see your rolodex, or an integer from 1 to 34 to use an attendee's ability.")
                        }
                    }
                }
            }

            'handle_party_end: {
                match party.state {
                    TooMuchTrouble => {
                        todo!(); // Cops Came
                        player.blame_someone(&mut party);
                    }
                    Overcrowded => {
                        todo!(); // Fire Marshal Came
                        player.blame_someone(&mut party);
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

            'store: loop {
                clear().unwrap();
                if !&victories[0..=player.id + 1].iter().any(|v| *v) || store.done_shopping {
                    break 'store;
                }
                'store_display: {
                    println!("Player {}, spend Pop to add guests to your rolodex; Spend Cash to expand the capacity of your house:\n", player.id + 1);
                    todo!()
                }

                'store_action: {
                    todo!()
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
