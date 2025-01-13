use crate::{clampedi8::*, guest::*, party::*};
use std::cmp::min;
use better_default::Default;
use nestify::nest;

nest!(
    #[derive(Default, Debug, Clone)]*
    pub struct Player {
        pub rolodex: Vec<Guest>,
        pub banned: 
            pub struct BannedGuest {
                guest: Option<Guest>,
                already_served_time: bool
            },
        #[default(ClampedI8::from((0, 0, 65)))]
        pub popularity: ClampedI8,
        #[default(ClampedI8::from((0, 0, 30)))]
        pub cash: ClampedI8,
        #[default(ClampedI8::capacity())]
        pub capacity: ClampedI8,
        pub id: usize,
    }
);

impl Player {
    pub fn start_of_day_guest_refresh(&mut self) {
        if let Some(g) = &self.banned.guest {
            if self.banned.already_served_time {
                self.rolodex.push(g.clone());
                self.banned.guest = None;
                self.banned.already_served_time = true;
            }
        }
        for guest in self.rolodex.iter_mut() {
            guest.trouble = guest.trouble_base;
            guest.chill = guest.chill_base;
            guest.ability_stock = guest.ability_base;
            guest.arrived_already_today = false;
        }
    }
    pub fn add_pop_from_guest(&mut self, amount: i8) {
        self.popularity += amount;
    }
    pub fn add_cash_from_guest(&mut self, amount: i8) {
        let mut c: i8 = *(self.cash.clone() + amount).clone();
        while c < 0 {
            self.popularity += -7;
            c += 1;
        }
        self.cash += amount;
    }
    pub fn greet_guest(&mut self, party: &Party) {
        let scored_guest = &party.attendees[party.attendees.len() - 1];
        self.add_pop_from_guest(*scored_guest.popularity);
        self.add_cash_from_guest(*scored_guest.cash);
        self.add_pop_from_guest((scored_guest.bonus_pop)(&party));
        if scored_guest.guest_type == GuestType::DANCER {
            self.add_pop_from_guest(min(
                16,
                party
                    .attendees
                    .iter()
                    .filter(|a| a.guest_type == GuestType::DANCER)
                    .count()
                    .pow(2) as i8,
            ))
        };
        self.add_cash_from_guest((scored_guest.bonus_cash)(&party));
    }
    pub fn end_of_party_score_guests(&mut self, party: &Party) {
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| *a.popularity >= 0)
                .map(|a| *a.popularity)
                .sum(),
        );
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| *a.popularity < 0)
                .map(|a| *a.popularity)
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| *a.cash >= 0)
                .map(|a| *a.cash)
                .sum(),
        );
        self.add_cash_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| *a.cash < 0)
                .map(|a| *a.cash)
                .sum(),
        );
        self.add_pop_from_guest(
            party
                .attendees
                .iter()
                .filter(|a| (a.bonus_pop)(&party) >= 0)
                .filter(|a| a.guest_type != GuestType::DANCER)
                .map(|a| (a.bonus_pop)(&party))
                .sum(),
        );
        // Dancer Bonus seperated from other bonuses to eliminate duplicate bonuses
        self.add_pop_from_guest(min(
            16,
            party
                .attendees
                .iter()
                .filter(|a| a.guest_type == GuestType::DANCER)
                .count()
                .pow(2) as i8,
        ));
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
    pub fn ban_guest(&mut self, guest: Guest) {
        if let Some(g) = &self.banned.guest {
            self.rolodex.push(g.clone());
        }
        self.banned.guest = Some(guest);
        self.banned.already_served_time = false;
    }
    pub fn blame_someone(&mut self, party: &mut Party) {
        todo!()
    }
}
