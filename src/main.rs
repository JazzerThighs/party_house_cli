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

use std::{cmp::min, io::stdin};
use clearscreen::clear;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
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
    let mut victories = vec![false; num_players + 1];
    let (mut party, mut rng): (Party, ThreadRng);

    'game: loop {
        clear().unwrap();
        for player in players.iter_mut() {
            'party_boilerplate: {
                player.start_of_day_guest_refresh();
                party = init_party(&player.capacity, star_guest_arrivals_for_win);
                rng = thread_rng();
                player.rolodex.shuffle(&mut rng);
            }

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

                'party_action: {
                    match party.state {
                        TooMuchTrouble | Overcrowded | EndedSuccessfully => break 'ongoing_party,
                        _ => {}
                    }
                    (party.state, party.action) = match party.state {
                        TooMuchTrouble | Overcrowded | EndedSuccessfully => unreachable!(),
                        IncomingGuest { amount, greet } => {
                            let gr = if greet { true } else { false };
                            match amount {
                                1 => (GoingFine, OpenDoor { greet: gr }),
                                2.. => (
                                    IncomingGuest {
                                        amount: amount - 1,
                                        greet,
                                    },
                                    OpenDoor { greet: gr },
                                ),
                                0 => unreachable!(),
                            }
                        }
                        AbilityState(a) => match a {
                            NoAbility => unreachable!(),
                            Evac => todo!(),
                            Shutter => todo!(),
                            Style(_) => todo!(),
                            Quench => todo!(),
                            StarSwap => todo!(),
                            Boot => todo!(),
                            LoveArrow => todo!(),
                            LoveArrowSecond => todo!(),
                            Cheer => todo!(),
                            Summoning => todo!(),
                            Peek => todo!(),
                            Greet => (GoingFine, OpenDoor{ greet: true }),
                        },
                        GoingFine | FullHouseUnusedAbilities => (party.state, TakeTurn),
                    };

                    party.state = match party.action {
                        TakeTurn => 'turn_selection: loop {
                            let next_state: PartyState;
                            let mut input = String::new();
                            if let Err(e) = stdin().read_line(&mut input) {
                                eprintln!("Error reading input: {}", e);
                                continue 'turn_selection;
                            }
                            match input.trim() {
                                "h" => party.attendees.push(player.rolodex.pop().unwrap()),
                                "r" => player.see_rolodex(&party.attendees),
                                i if i.parse::<u8>().map_or(false, |n| (1..=34).contains(&n)) => party.try_use_attendee_ability(i),
                                _ => println!("Invalid Input. Please input \"h\" to open the door, \"r\" to see your rolodex, or an integer from 1 to 34 to use an attendee's ability.")
                            }
                            next_state;
                        },
                        OpenDoor { greet } => {
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
                            if scored_guest.tagalongs > 0 {
                                IncomingGuest { amount: scored_guest.tagalongs, greet }
                            } else {
                                party.state
                            }
                        },
                        EndParty => EndedSuccessfully
                    };
                }

                'evaluate_party: {
                    #[rustfmt::skip]
                    let evaluate_party = || {
                        party.state = {
                            if party.attendees.iter().filter(|g| g.trouble).count() - party.attendees.iter().filter(|g| g.chill).count() >= 3 {
                                TooMuchTrouble
                            } else if party.attendees.len() > *party.capacity as usize {
                                Overcrowded
                            }
                            // Check if the party is full/rolodex is empty, but the player can still use certain abilities:
                            else if 
                                // If you can no longer add any more people to the party...
                                (party.attendees.len() == *party.capacity as usize || player.rolodex.is_empty())
                                && 
                                // ...and there are either...
                                (
                                    // ...attendees with unused Full-House Abilities...
                                    party.attendees.iter().filter(|g| g.full_house_ability == Yes && g.ability_stock > 0).count() >= 1
                                    || 
                                    // ...or...
                                    (
                                        // ... attendees with unused Full-House-Ability replenishes...
                                        party.attendees.iter().filter(|g| g.full_house_ability == IfYesIsPresent && g.ability_stock > 0).count() >= 1
                                        && 
                                        // ...and attendees who can have their Full-House Abilities replenished, ...
                                        party.attendees.iter().filter(|g| g.full_house_ability == Yes).count() >= 1
                                    )
                                )
                            {
                                // then give the player the option to use those abilities/replenishes
                                FullHouseUnusedAbilities
                            } else if party.attendees.len() == *party.capacity as usize || player.rolodex.is_empty() {
                                EndedSuccessfully
                            } else {
                                party.state
                            }
                        };
                    };
                    evaluate_party();
                }
            }

            'handle_party_end: {
                if party.state == EndedSuccessfully {
                    player.end_of_party_score_guests(&party);
                    if party.attendees.iter().filter(|a| *a.stars == 1).count()
                        - party.attendees.iter().filter(|a| *a.stars == 1).count()
                        >= star_guest_arrivals_for_win
                    {
                        victories[player.id] = true;
                        todo!() // Show that the player won
                    }
                } else {
                    if party.state == TooMuchTrouble {
                        todo!() // Show that the cops came
                    } else if party.state == Overcrowded {
                        todo!() // Show that the fire marshall came
                    }
                    player.blame_someone(&mut party);
                }
            }

            'return_guests_to_player: {
                todo!()
            };

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
