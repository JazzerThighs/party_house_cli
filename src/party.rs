use clearscreen::clear;
use crate::{guests::*, init::*};

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
    fn add_bonus_from_guest(&mut self, party: &Party, guest: &Guest) {
        self.add_pop_from_guest((guest.bonus_pop)(&party));
        self.add_cash_from_guest((guest.bonus_cash)(&party));
    }
}

fn photograph_guest(party: &Party, player: &mut Player) {
    let scored_guest = &party.attendees[party.attendees.len() - 1];
    player.add_pop_from_guest(scored_guest.popularity);
    player.add_cash_from_guest(scored_guest.cash);
    player.add_bonus_from_guest(party, scored_guest);
}

pub fn do_partying(party: &mut Party, player: &mut Player, victories: &mut Vec<bool>) -> bool {
    let still_partying: bool = true;
    println!("Player {}, throw a party!", player.id);
    if victories.iter().any(|v| *v) {
        for i in 0..victories.len() {
            if victories[i] {
                println!("Player {} won today!", i + 1)
            };
        }
        println!("Last Chance!\n");
    }
    clear().unwrap();

    still_partying
}