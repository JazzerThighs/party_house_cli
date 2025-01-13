use crate::{clampedi8::*, guest::*, player::*};
use better_default::Default;
use nestify::nest;

nest!(
    #[derive(Default, Debug)]*
    pub struct Party {
        pub attendees: Vec<Guest>,
        #[default(ClampedI8::capacity())]
        pub capacity: ClampedI8,
        pub trouble_count: u8,
        pub chill_count: u8,
        pub star_guest_arrivals_for_win: usize,
        pub state:
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
    pub fn evaluate_state(&mut self, player: &Player) {
        self.state = {
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
                self.state.clone()
            }
        };
    }
}
