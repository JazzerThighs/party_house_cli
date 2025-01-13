#![allow(dead_code, unused_mut, unused_imports, unused_variables, unused_labels, unused_assignments)]
#![allow(unreachable_code)]
mod clampedi8;
mod guest;
mod init;
mod party;
mod player;
mod store;
use clearscreen::clear;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use {
    guest::{AbilityType::*, FullHouseAbilityCondition::*, GuestType::*},
    init::*,
    party::{PartyState::*, *},
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

            'party: loop {
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
                        TooMuchTrouble | Overcrowded | EndedSuccessfully => break 'party,
                        TagalongsIncoming(t) => {
                            party.state = TagalongsIncoming(t - 1);
                            todo!()
                        },
                        GreetsIncoming(g) => {
                            party.state = TagalongsIncoming(g - 1);
                            todo!()
                        },
                        FullHouseUnusedAbilities => todo!(),
                        GoingFine => todo!(),
                    }
                }

                party.evaluate_state(player);
            }

            'handle_party_end: {
                if party.state == EndedSuccessfully {
                    if party.attendees.iter().filter(|a| *a.stars == 1).count()
                        - party.attendees.iter().filter(|a| *a.stars == 1).count()
                        >= star_guest_arrivals_for_win
                    {
                        victories[player.id] = true
                    }
                    player.end_of_party_score_guests(&party);
                } else if party.state == TooMuchTrouble || party.state == Overcrowded {
                    player.blame_someone(&mut party);
                }
            }

            'return_guests_to_player: {
                todo!()
            };

            'store: loop {
                if !&victories[0..=player.id + 1].iter().any(|v| *v) || store.done_shopping {
                    break 'store;
                }
                'store_display: {
                    println!("Player {}, spend Pop to add guests to your rolodex; Spend Cash to expand the capacity of your house:\n", player.id + 1);
                    todo!()
                }

                // take input to determine what action to take, then loop around
            }
        }

        clear().unwrap();

        // Check for Ultimate Parties or Run Out Of Time after everyone has had a turn
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

    for i in 0..victories.len() {
        match victories[i] {
            true => println!("Player {} threw the Ultimate Party!", i + 1),
            false => println!("Player {} loses!", i + 1),
        }
    }
    println!();
}
