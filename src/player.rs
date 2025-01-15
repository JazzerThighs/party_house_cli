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
                pub guest: Option<Guest>,
                pub already_served_time: bool
            },
        pub booted: Vec<Guest>, 
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
    pub fn add_pop_from_guest(&mut self, amount: i8) {
        self.popularity += amount;
    }
    pub fn add_cash_from_guest(&mut self, amount: i8) {
        let mut c: i8 = *(self.cash.clone() + amount);
        while c < 0 {
            self.popularity += -7;
            c += 1;
        }
        self.cash += amount;
    }
    #[rustfmt::skip]
    pub fn end_of_party_score_guests(&mut self, party: &Party) {
        // Pop Score
        self.add_pop_from_guest(party.attendees.iter().filter(|a| *a.popularity >= 0).map(|a| *a.popularity).sum());
        self.add_pop_from_guest(party.attendees.iter().filter(|a| *a.popularity < 0).map(|a| *a.popularity).sum());
        
        // Cash Score
        self.add_cash_from_guest(party.attendees.iter().filter(|a| *a.cash >= 0).map(|a| *a.cash).sum());
        self.add_cash_from_guest(party.attendees.iter().filter(|a| *a.cash < 0).map(|a| *a.cash).sum());
        
        // Pop Bonuses; Note: Dancer Bonus seperated from other bonuses to eliminate duplicate Dancer bonuses.
        self.add_pop_from_guest(party.attendees.iter().filter(|a| (a.bonus_pop)(&party) >= 0).filter(|a| a.guest_type != GuestType::DANCER).map(|a| (a.bonus_pop)(&party)).sum());
        self.add_pop_from_guest(min(
            16,
            party.attendees.iter().filter(|a| a.guest_type == GuestType::DANCER).count().pow(2) as i8
        ));
        self.add_pop_from_guest(party.attendees.iter().filter(|a| (a.bonus_pop)(&party) < 0).map(|a| (a.bonus_pop)(&party)).sum());
        
        // Cash Bonuses
        self.add_cash_from_guest(party.attendees.iter().filter(|a| (a.bonus_cash)(&party) >= 0).map(|a| (a.bonus_cash)(&party)).sum(),);
        self.add_cash_from_guest(party.attendees.iter().filter(|a| (a.bonus_cash)(&party) < 0).map(|a| (a.bonus_cash)(&party)).sum(),);
    }
    pub fn ban_guest(&mut self, guest: Guest) {
        if let Some(g) = &self.banned.guest {
            self.rolodex.push(g.clone());
        }
        self.banned.guest = Some(guest);
        self.banned.already_served_time = false;
    }
}
