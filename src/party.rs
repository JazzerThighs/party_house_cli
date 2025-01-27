use std::io::stdin;

use crate::{clampedi8::*, guest::*, player::*};
use better_default::Default;
use nestify::nest;

nest!(
    #[derive(Default, Clone, Debug)]*
    pub struct Party {
        pub attendees: Vec<Guest>,
        #[default(ClampedI8::capacity())]
        pub capacity: ClampedI8,
        pub peek_slot: Option<Guest>,
        pub ability_state: bool,
        pub stars_to_win: usize,
        pub state:
            #[derive(PartialEq, Eq)]
            pub enum PartyState {
                #[default(amount: 0, greet: false)]
                IncomingGuest{amount: u8, greet: bool},
                AbilityState(AbilityType),
                ViewingRolodex,
                FullHouseUnusedAbility,
                TooMuchTrouble,
                Overcrowded,
                EndedSuccessfully,
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

pub fn check_for_party_end_conditions(party: &mut Party, house_is_full: bool, rolodex_is_empty: bool, available_full_house_abilities: bool, replenishes_available: bool) -> bool {
    use PartyState::*;
    if party.state == EndedSuccessfully {
        return true;
    }
    if party.attendees.iter().filter(|g| g.trouble).count() as i8
        - party.attendees.iter().filter(|g| g.chill).count() as i8
        >= 3 
    {
        party.state = TooMuchTrouble;
        return true;
    }
    if party.attendees.len() > *party.capacity as usize {
        party.state = Overcrowded;
        return true;
    }
    if (house_is_full || rolodex_is_empty) && !available_full_house_abilities && !replenishes_available{
        party.state = EndedSuccessfully;
        return true;
    }
    false
}

pub fn ban_guest(player: &mut Player, party: &mut Party) {
    if let Some(g) = &player.banned.guest.take() {
        player.rolodex.push(g.clone());
    }
    loop {
    let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }
        match input.trim() {
            i if i.parse::<usize>().map_or(false, |n| (1..=34).contains(&n) && n <= party.attendees.len()) => {
                let idx = i.parse::<usize>().unwrap() - 1;
                let banned = party.attendees[idx].clone();
                party.attendees.remove(idx);
                player.banned.guest = Some(banned);
                player.banned.already_served_time = false;
                break;
            },
            _ => println!("Invalid input.")
        }
    }
}