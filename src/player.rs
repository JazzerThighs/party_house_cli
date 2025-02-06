use crate::{clampedi8::*, guest::*};
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
        let mut c: i8 = *self.cash + amount;
        while c < 0 {
            self.popularity -= 7;
            c += 1;
        }
        self.cash += amount;
    }
}
