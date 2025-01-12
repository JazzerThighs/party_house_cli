use crate::{clampedi8::*, guest::*, player::*};
use clearscreen::clear;
use nestify::nest;
use better_default::Default;
use rand::{seq::SliceRandom, thread_rng};
use std::io::stdin;

nest!(
    #[derive(Default, Debug)]*
    pub struct Party {
        pub attendees: Vec<Guest>,
        #[default(ClampedI8::capacity())]
        pub capacity: ClampedI8,
        pub trouble_count: u8,
        pub chill_count: u8,
        pub star_guest_arrivals_for_win: usize,
        pub party_state:
            #[derive(PartialEq, Eq)]
            pub enum PartyState {
                #[default]
                GoingFine,
                EndedSuccessfully,
                FullHouseUnusedAbilities,
                TagalongsIncoming(u8),
                TooMuchTrouble,
                Overcrowded,
            }
    }
);

pub fn do_partying(party: &mut Party, player: &mut Player, victories: &mut Vec<bool>) {
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
        break 'partyloop;
    }
}
