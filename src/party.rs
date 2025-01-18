use std::cmp::min;

use crate::{clampedi8::*, guest::{GuestType::* ,*}, player::*};
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
        pub attendee_ability_source: usize,
        pub peek_slot: Option<Guest>,
        pub state:
            #[derive(PartialEq, Eq)]
            pub enum PartyState {
                TooMuchTrouble,
                Overcrowded,
                EndedSuccessfully,
                #[default(amount: 0, greet: false)]
                IncomingGuest{amount: u8, greet: bool},
                FullHouseUnusedAbility,
                AbilityState(AbilityType),
                ViewingRolodex
            },
    }
);

pub fn get_party_state(party: &Party, player: &Player) -> (bool, bool, bool, bool) {
    let house_is_full: bool =party.attendees.len() == *party.capacity as usize;
    let rolodex_is_empty: bool = player.rolodex.is_empty();
    let available_full_house_abilities: bool = party
        .attendees
        .iter()
        .filter(|g| g.full_house_ability == FullHouseAbilityCondition::Yes && g.ability_stock > 0)
        .count()
        >= 1;
    let replenishes_available: bool = party
        .attendees
        .iter()
        .filter(|g| {
            g.full_house_ability == FullHouseAbilityCondition::IfYesIsPresent && g.ability_stock > 0
        })
        .count()
        >= 1
        && party
            .attendees
            .iter()
            .filter(|g| g.full_house_ability == FullHouseAbilityCondition::Yes)
            .count()
            >= 1;
    (
        house_is_full,
        rolodex_is_empty,
        available_full_house_abilities,
        replenishes_available,
    )
}

pub fn check_for_party_end_conditions(party: &mut Party, no_more_guests_can_come_in: bool) -> bool {
    use PartyState::*;
    if party.attendees.iter().filter(|g| g.trouble).count()
        - party.attendees.iter().filter(|g| g.chill).count()
        >= 3 
    {
        party.state = TooMuchTrouble;
        return true;
    }
    if party.attendees.len() > *party.capacity as usize {
        party.state = Overcrowded;
        return true;
    }
    if no_more_guests_can_come_in {
        party.state = EndedSuccessfully;
        return true;
    }
    false
}