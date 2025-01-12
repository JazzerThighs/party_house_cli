use crate::{clampedi8::*, guest::*, player::*};
use better_default::Default;
use clearscreen::clear;
use nestify::nest;
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
            #[derive(Clone, PartialEq, Eq)]
            pub enum PartyState {
                TooMuchTrouble,
                Overcrowded,
                FullHouseUnusedAbilities,
                EndedSuccessfully,
                
                #[default]
                GoingFine,
                
                // For when a guest brings their own friends, but that guest and their friends are not being Greeted
                TagalongsIncoming(u8),
                
                // For when a guest brings their own friends, and that guest as well as their friends being Greeted
                GreetsIncoming(u8),
            }
    }
);

impl Party {
    #[rustfmt::skip]
    fn evaluate_party_state(&mut self, player: &Player) {
        self.party_state = {
            use FullHouseAbilityCondition::*;
            use PartyState::*;
            if self.attendees.iter().filter(|g| g.trouble).count() - self.attendees.iter().filter(|g| g.chill).count() >= 3 {
                TooMuchTrouble
            } else if self.attendees.len() > *self.capacity as usize {
                Overcrowded
            }
            // Check if the party is full/rolodex is empty, but the player can still use certain abilities:
            else if 
                // If you can no longer add any more people to the party...
                (self.attendees.len() == *self.capacity as usize || player.rolodex.is_empty())
                && 
                // ...and there are either...
                (
                    // ...attendees with unused Full-House Abilities...
                    self.attendees.iter().filter(|g| g.full_house_ability == Yes && g.ability_stock > 0).count() >= 1
                    || 
                    // ...or...
                    (
                        // ... attendees with unused Full-House-Ability replenishes...
                        self.attendees.iter().filter(|g| g.full_house_ability == IfYesIsPresent && g.ability_stock > 0).count() >= 1
                        && 
                        // ...and attendees who can have their Full-House Abilities replenished, ...
                        self.attendees.iter().filter(|g| g.full_house_ability == Yes).count() >= 1
                    )
                )
            {
                // then give the player the option to use those abilities/replenishes
                FullHouseUnusedAbilities
            } else if self.attendees.len() == *self.capacity as usize || player.rolodex.is_empty() {
                EndedSuccessfully
            } else {
                self.party_state.clone()
            }
        };
    }
}

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
