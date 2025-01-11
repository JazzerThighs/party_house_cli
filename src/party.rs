use std::io::stdin;
use crate::init::*;
use clearscreen::clear;
use rand::{seq::SliceRandom, thread_rng};

impl Player {
    fn add_pop_from_guest(&mut self, amount: i8) {
        self.popularity += amount;
    }
    fn add_cash_from_guest(&mut self, amount: i8) {
        let mut c: i8 = *(self.cash.clone() + amount).clone();
        while c < 0 {
            self.popularity += -7;
            c += 1;
        }
        self.cash += amount;
    }
    fn greet_guest(&mut self, party: &Party) {
        let scored_guest = &party.attendees[party.attendees.len() - 1];
        self.add_pop_from_guest(scored_guest.popularity);
        self.add_cash_from_guest(scored_guest.cash);
        self.add_pop_from_guest((scored_guest.bonus_pop)(&party));
        self.add_cash_from_guest((scored_guest.bonus_cash)(&party));
    }
    fn end_of_party_score_guests(&mut self, party: &Party) {
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| a.popularity >= 0)
                .map(|a| a.popularity)
                .sum(),
        );
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| a.popularity < 0)
                .map(|a| a.popularity)
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| a.cash >= 0)
                .map(|a| a.cash)
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| a.cash < 0)
                .map(|a| a.cash)
                .sum(),
        );
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| (a.bonus_pop)(&party) >= 0)
                .map(|a| (a.bonus_pop)(&party))
                .sum(),
        );
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| (a.bonus_pop)(&party) < 0)
                .map(|a| (a.bonus_pop)(&party))
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| (a.bonus_cash)(&party) >= 0)
                .map(|a| (a.bonus_cash)(&party))
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| (a.bonus_cash)(&party) < 0)
                .map(|a| (a.bonus_cash)(&party))
                .sum(),
        );
    }
}

pub fn do_partying(party: &mut Party, player: &mut Player, victories: &mut Vec<bool>) {
    let mut tagalong_carry: u8 = 0;
    let mut greet_carry: bool = false;
    let mut end_party: bool = false;
    let mut rng = thread_rng();
    player.rolodex.shuffle(&mut rng);

    'partyloop: loop {
        'partyheader: {
            clear().unwrap();
            println!("Player {}, throw a party!", player.id);
            if victories.iter().any(|v| *v) {
                for i in 0..victories.len() {
                    if victories[i] {
                        println!("Player {} won today!", i + 1)
                    };
                }
                println!("Last Chance!\n");
            } else {
                break 'partyheader;
            }
        }

        match player.rolodex.is_empty() {
            false => { // Rolodex is not empty
                'nonemptyloop: loop {
                    let mut input = String::new();
                    if let Err(e) = stdin().read_line(&mut input) {
                        eprintln!("Error reading input: {}", e);
                        continue 'nonemptyloop;
                    }
                    match input.trim() {
                        "h" => { // Open Door for New Guest ("h" for "Hit Me")
                            party.attendees.push(player.rolodex.pop().unwrap());
                            if greet_carry { player.greet_guest(party); }
                            tagalong_carry = party.attendees[party.attendees.len() - 1].tagalongs;
                            break 'nonemptyloop;
                        },
                        _ => eprintln!("Invalid input. Please enter a valid input."),
                    }
                }
            },
            true => { // Rolodex is empty

            }
        }
        match (
            party.attendees.len(), 
            tagalong_carry, 
            greet_carry,
            end_party
        ) {
            (0, tc, gc, ep) if tc != 0 || gc == true || ep == true => unreachable!(),
            (_, 0, false, true) => {
                player.end_of_party_score_guests(&party);
                break 'partyloop;
            },
            (_, tc, _, _) if tc > 0 => {},
            (_, _, _, _) => {}
        }
    }
}
