use std::cmp::min;

use crate::{clampedi8::*, guest::*, player::*};
use better_default::Default;
use nestify::nest;

nest!(
    #[derive(Default, Clone, Debug)]*
    pub struct Party {
        pub attendees: Vec<Guest>,
        #[default(ClampedI8::capacity())]
        pub capacity: ClampedI8,
        pub trouble_count: u8,
        pub chill_count: u8,
        pub star_guest_arrivals_for_win: usize,
        pub state:
            #[derive(PartialEq, Eq)]
            pub enum PartyState {
                TooMuchTrouble,
                Overcrowded,
                FullHouseUnusedAbilities,
                EndedSuccessfully,
                IncomingGuest{amount: u8, greet: bool},
                AbilityState(AbilityType),
                #[default]
                GoingFine,
            },
        pub action:
            #[derive(PartialEq, Eq)]
            pub enum PartyAction {
                #[default]
                TakeTurn,
                OpenDoor{greet: bool},
                EndParty
            }
    }
);
