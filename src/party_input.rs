use clearscreen::clear;
use rand::{rng, seq::SliceRandom};
use std::{
    cmp::{min, Reverse},
    io::stdin,
};

use crate::{
    display::*,
    guest::{AbilityType::*, GuestType::*, *},
    party::{PartyState::*, *},
    player::*,
};

#[allow(unused_assignments)]
pub fn party_input(
    player: &mut Player,
    party: &mut Party,
    house_is_full: &bool,
    rolodex_is_empty: &bool,
    victories: &Vec<bool>,
    day_count: usize,
    mut boxed_message: String,
) -> String {
    macro_rules! refresh {
        (party $message:expr) => {
            boxed_message = $message;
            party_display(&party, player, &victories, day_count, &boxed_message);
        };
    }
    'party_input: loop {
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue 'party_input;
        }
        match input.trim() {
            "h" => match (house_is_full, rolodex_is_empty) {
                (true, _) => {
                    refresh!(party "The house is full, cannot invite a new guest!".to_string());
                    continue 'party_input;
                }
                (_, true) => {
                    refresh!(party "Rolodex is empty, no one left to invite!".to_string());
                    continue 'party_input;
                }
                (false, false) => {
                    boxed_message = "".to_string();
                    party.state = IncomingGuest {
                        amount: 1,
                        greet: false,
                    };
                    return boxed_message;
                }
            },
            "r" => {
                let mut rolodex_view: Vec<&Guest> = player.rolodex.iter().collect();
                let mut attendees_view: Vec<&Guest> = party.attendees.iter().collect();
                let mut booted_view: Vec<&Guest> = player.booted.iter().collect();
                if let Some(peek) = &party.peek_slot {
                    rolodex_view.push(&peek)
                };
                if let Some(banned) = &player.banned.guest {
                    booted_view.push(&banned)
                };
                rolodex_view.sort_by_key(|guest| {
                    (
                        guest.sort_value,
                        Reverse(*guest.popularity),
                        Reverse(*guest.cash),
                    )
                });
                attendees_view.sort_by_key(|guest| {
                    (
                        guest.sort_value,
                        Reverse(*guest.popularity),
                        Reverse(*guest.cash),
                    )
                });
                booted_view.sort_by_key(|guest| {
                    (
                        guest.sort_value,
                        Reverse(*guest.popularity),
                        Reverse(*guest.cash),
                    )
                });
                clear().unwrap();
                println!("Player {}", player.id + 1);
                let mut i = 1;
                println!("The following contacts can still show up to the party:");
                for r in rolodex_view {
                    println!("{i:>2}) {}", display_guest(r));
                    i += 1;
                }
                if attendees_view.len() > 0 {
                    println!("\nThe following contacts have already showed up to the party:");
                    for a in attendees_view {
                        println!("{i:>2}) {}", display_guest(a));
                        i += 1;
                    }
                }
                if booted_view.len() > 0 {
                    println!("\nThe following contacts cannot show up to the party today:");
                    for b in booted_view {
                        println!("{i:>2}) {}", display_guest(b));
                        i += 1;
                    }
                }
                pause_for_enter("\nPress \"Enter\" to go back to the party...");
                return boxed_message;
            }
            "e" => {
                if party.attendees.len() > 0 || (*rolodex_is_empty && party.peek_slot.is_none()) {
                    party.state = EndedSuccessfully;
                    return boxed_message;
                } else {
                    refresh!(party "Don't end the party yet! This place is dead!".to_string());
                    continue 'party_input;
                }
            }
            "b" => {
                if party.peek_slot.is_some() {
                    player.booted.push(party.peek_slot.take().unwrap());
                    party.state = IncomingGuest {
                        amount: 0,
                        greet: false,
                    };
                    return boxed_message;
                } else {
                    refresh!(party "No one is at the front door!".to_string());
                    continue 'party_input;
                }
            }
            "i" => {
                display_information();
                pause_for_enter("\nPress \"Enter\" to go back to the party...");
                return boxed_message;
            }
            i if i.parse::<usize>().map_or(false, |n| {
                (1..=34).contains(&n) && n <= *party.capacity as usize && n <= party.attendees.len()
            }) =>
            {
                let idx = i.parse::<usize>().unwrap() - 1;
                if party.attendees[idx].ability_type == NoAbility {
                    refresh!(party "This guest does not have an ability. Please select a different guest to use their ability.".to_string());
                    continue 'party_input;
                }
                if party.attendees[idx].ability_stock < 1 {
                    refresh!(party "This guest's ability isn't available. Please select a different guest to use their ability.".to_string());
                    continue 'party_input;
                }
                party.ability_state = true;
                match party.attendees[idx].ability_type {
                    Evac => {
                        party.attendees[idx].ability_stock -= 1;
                        party.ability_state = false;
                        player.rolodex.extend(party.attendees.drain(0..));
                        if let Some(peek) = party.peek_slot.take() {
                            player.rolodex.push(peek);
                        }
                        let mut random_number = rng();
                        player.rolodex.shuffle(&mut random_number);
                        party.state = IncomingGuest {
                            amount: 0,
                            greet: false,
                        };
                        return boxed_message;
                    }
                    Cheer => {
                        if party
                            .attendees
                            .iter()
                            .filter(|a| a.ability_type != Cheer)
                            .filter(|a| a.ability_stock < a.ability_base)
                            .count()
                            == 0
                        {
                            party.ability_state = false;
                            refresh!(party "The party has no one who can gain an ability stock from a Cheer.".to_string());
                            continue 'party_input;
                        } else {
                            party.attendees[idx].ability_stock -= 1;
                            party.ability_state = false;
                            for p in party
                                .attendees
                                .iter_mut()
                                .filter(|g| g.ability_type != Cheer)
                            {
                                p.ability_stock = p.ability_base;
                            }
                            party.state = IncomingGuest {
                                amount: 0,
                                greet: false,
                            };
                            return boxed_message;
                        }
                    }
                    Quench => {
                        if party.attendees.iter().filter(|a| a.trouble).count() == 0 {
                            party.ability_state = false;
                            refresh!(party "The party has no one that needs counseling.".to_string());
                            continue 'party_input;
                        } else {
                            party.attendees[idx].ability_stock -= 1;
                            party.ability_state = false;
                            for p in party.attendees.iter_mut() {
                                p.trouble = false;
                            }
                            party.state = IncomingGuest {
                                amount: 0,
                                greet: false,
                            };
                            return boxed_message;
                        }
                    }
                    Peek => match (&party.peek_slot, rolodex_is_empty) {
                        (Some(_), _) => {
                            party.ability_state = false;
                            refresh!(party "Someone is already at the front door!".to_string());
                            continue 'party_input;
                        }
                        (None, true) => {
                            party.ability_state = false;
                            refresh!(party "Rolodex is empty, no one left to invite!".to_string());
                            continue 'party_input;
                        }
                        (None, false) => {
                            party.attendees[idx].ability_stock -= 1;
                            party.ability_state = false;
                            party.peek_slot = Some(player.rolodex.pop().unwrap());
                            return boxed_message;
                        }
                    },
                    Shutter => {
                        refresh!(party "Select a guest to Photograph.".to_string());
                        'shutter_input: loop {
                            let mut input = String::new();
                            if let Err(e) = stdin().read_line(&mut input) {
                                eprintln!("Error reading input: {}", e);
                                continue 'party_input;
                            }
                            match input.trim() {
                                i if i.parse::<usize>().map_or(false, |n| {
                                    (1..=34).contains(&n) && n <= party.attendees.len()
                                }) =>
                                {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    let target = i.parse::<usize>().unwrap() - 1;
                                    player.add_pop_from_guest(*party.attendees[target].popularity);
                                    player.add_cash_from_guest(*party.attendees[target].cash);
                                    player.add_pop_from_guest((party.attendees[target].bonus_pop)(
                                        &party,
                                    ));
                                    if party.attendees[target].guest_type == DANCER {
                                        player.add_pop_from_guest(min(
                                            16,
                                            party
                                                .attendees
                                                .iter()
                                                .filter(|a| a.guest_type == DANCER)
                                                .count()
                                                .pow(2)
                                                as i8,
                                        ))
                                    };
                                    player
                                        .add_cash_from_guest((party.attendees[target].bonus_cash)(
                                            &party,
                                        ));
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                "n" => {
                                    party.ability_state = false;
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                _ => {
                                    println!("Invalid input.");
                                    continue 'shutter_input;
                                }
                            }
                        }
                    }
                    Style(pop_up) => {
                        refresh!(party "Select a guest to Style.".to_string());
                        'style_input: loop {
                            let mut input = String::new();
                            if let Err(e) = stdin().read_line(&mut input) {
                                eprintln!("Error reading input: {}", e);
                                continue 'party_input;
                            }
                            match input.trim() {
                                i if i.parse::<usize>().map_or(false, |n| {
                                    (1..=34).contains(&n) && n <= party.attendees.len()
                                }) =>
                                {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    let target = i.parse::<usize>().unwrap() - 1;
                                    party.attendees[target].popularity += pop_up as i8;
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                "n" => {
                                    party.ability_state = false;
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                _ => {
                                    println!("Invalid input.");
                                    continue 'style_input;
                                }
                            }
                        }
                    }
                    Boot => {
                        refresh!(party "Select a guest to Kick from the party.".to_string());
                        'boot_input: loop {
                            let mut input = String::new();
                            if let Err(e) = stdin().read_line(&mut input) {
                                eprintln!("Error reading input: {}", e);
                                continue 'party_input;
                            }
                            match input.trim() {
                                i if i.parse::<usize>().map_or(false, |n| {
                                    (1..=34).contains(&n) && n <= party.attendees.len()
                                }) =>
                                {
                                    party.attendees[idx].ability_stock -= 1;
                                    party.ability_state = false;
                                    let target = i.parse::<usize>().unwrap() - 1;
                                    player.booted.push(party.attendees[target].clone());
                                    party.attendees.remove(target);
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                "n" => {
                                    party.ability_state = false;
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                _ => {
                                    println!("Invalid input.");
                                    continue 'boot_input;
                                }
                            }
                        }
                    }
                    StarSwap => {
                        if *rolodex_is_empty {
                            party.ability_state = false;
                            refresh!(party "Rolodex is empty, no one can be swapped out.".to_string());
                            continue 'party_input;
                        }
                        match (
                            party.attendees.iter().filter(|a| *a.stars > 0).count() > 0,
                            player.rolodex.iter().filter(|a| *a.stars > 0).count() > 0,
                        ) {
                            (true, _) | (_, true) => {
                                refresh!(party "Select a guest to Swap Out.".to_string());
                                'star_swap_input: loop {
                                    let mut input = String::new();
                                    if let Err(e) = stdin().read_line(&mut input) {
                                        eprintln!("Error reading input: {}", e);
                                        continue 'party_input;
                                    }
                                    match input.trim() {
                                        i if i.parse::<usize>().map_or(false, |n| {
                                            (1..=34).contains(&n) && n <= party.attendees.len()
                                        }) =>
                                        {
                                            let target = i.parse::<usize>().unwrap() - 1;
                                            let mut replacement: Guest = Guest::default();
                                            let goes_away: Guest;
                                            for r in 0..player.rolodex.len() {
                                                if *party.attendees[target].stars == 0 {
                                                    if player
                                                        .rolodex
                                                        .iter()
                                                        .filter(|g| *g.stars != 0)
                                                        .count()
                                                        == 0
                                                    {
                                                        refresh!(party "Cannot swap star guest for non-star guest because your rolodex has no available non-star guests.".to_string());
                                                        continue 'star_swap_input;
                                                    }
                                                    if *player.rolodex[r].stars != 0 {
                                                        replacement = player.rolodex[r].clone();
                                                        player.rolodex.remove(r);
                                                        break;
                                                    }
                                                } else {
                                                    if player
                                                        .rolodex
                                                        .iter()
                                                        .filter(|g| *g.stars == 0)
                                                        .count()
                                                        == 0
                                                    {
                                                        refresh!(party "Cannot swap non-star guest for star guest because your rolodex has no available star guests.".to_string());
                                                        continue 'star_swap_input;
                                                    }
                                                    if *player.rolodex[r].stars == 0 {
                                                        replacement = player.rolodex[r].clone();
                                                        player.rolodex.remove(r);
                                                        break;
                                                    }
                                                }
                                            }
                                            party.attendees[idx].ability_stock -= 1;
                                            goes_away = party.attendees[target].clone();
                                            party.attendees.remove(target);
                                            party.attendees.insert(target, replacement);
                                            player.booted.push(goes_away);
                                            party.state = IncomingGuest {
                                                amount: party.attendees[target].tagalongs,
                                                greet: false,
                                            };
                                            party.ability_state = false;
                                            boxed_message = "".to_string();
                                            return boxed_message;
                                        }
                                        "n" => {
                                            party.state = IncomingGuest {
                                                amount: 0,
                                                greet: false,
                                            };
                                            party.ability_state = false;
                                            boxed_message = "".to_string();
                                            return boxed_message;
                                        }
                                        _ => {
                                            println!("Invalid input.");
                                            continue 'star_swap_input;
                                        }
                                    }
                                }
                            }
                            (false, false) => {
                                party.ability_state = false;
                                refresh!(party "There are neither any star guests in the rolodex nor the party.".to_string());
                                continue 'party_input;
                            }
                        }
                    }
                    LoveArrow => {
                        if party.attendees.len() < 2 {
                            party.ability_state = false;
                            refresh!(party "At least 2 partygoers need to be paired up by a Cupid's Arrow.".to_string());
                            continue 'party_input;
                        }
                        refresh!(party "Select a pair of people who are directly next to each other and will leave together by selecting the person with the lower of the 2 positions.".to_string());
                        'love_arrow_input: loop {
                            let mut input = String::new();
                            if let Err(e) = stdin().read_line(&mut input) {
                                eprintln!("Error reading input: {}", e);
                                continue 'party_input;
                            }
                            match input.trim() {
                                i if i.parse::<usize>().map_or(false, |n| {
                                    (1..=34).contains(&n) && n <= party.attendees.len() - 1
                                }) =>
                                {
                                    party.attendees[idx].ability_stock -= 1;
                                    let target = i.parse::<usize>().unwrap() - 1;
                                    player.booted.push(party.attendees[target].clone());
                                    player.booted.push(party.attendees[target + 1].clone());
                                    party.attendees.remove(target);
                                    party.attendees.remove(target);
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    party.ability_state = false;
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                "n" => {
                                    party.state = IncomingGuest {
                                        amount: 0,
                                        greet: false,
                                    };
                                    party.ability_state = false;
                                    boxed_message = "".to_string();
                                    return boxed_message;
                                }
                                i if i
                                    .parse::<usize>()
                                    .map_or(false, |n| n == *party.capacity as usize) =>
                                {
                                    refresh!(party "If you want to pair-kick the two guests at the back, select the guest with the lower of the 2 positional numbers.".to_string());
                                    continue 'love_arrow_input;
                                }
                                _ => {
                                    println!("Invalid input.");
                                    continue 'love_arrow_input;
                                }
                            }
                        }
                    }
                    Greet => match (house_is_full, rolodex_is_empty) {
                        (true, _) => {
                            party.ability_state = false;
                            refresh!(party "The house is full, cannot invite a new guest!".to_string());
                            continue 'party_input;
                        }
                        (_, true) => {
                            party.ability_state = false;
                            refresh!(party "Rolodex is empty, no one left to invite!".to_string());
                            continue 'party_input;
                        }
                        (false, false) => {
                            party.attendees[idx].ability_stock -= 1;
                            party.state = IncomingGuest {
                                amount: 1,
                                greet: true,
                            };
                            party.ability_state = false;
                            return boxed_message;
                        }
                    },
                    Summoning => match (house_is_full, rolodex_is_empty) {
                        (true, _) => {
                            party.ability_state = false;
                            refresh!(party "The house is full, cannot invite a new guest!".to_string());
                            continue 'party_input;
                        }
                        (_, true) => {
                            party.ability_state = false;
                            refresh!(party "Rolodex is empty, no one left to invite!".to_string());
                            continue 'party_input;
                        }
                        (false, false) => {
                            refresh!(party "Select a contact to invite to the party.".to_string());
                            'summoning_input: loop {
                                player.rolodex.sort_by_key(|guest| {
                                    (
                                        guest.sort_value,
                                        Reverse(*guest.popularity),
                                        Reverse(*guest.cash),
                                    )
                                });
                                println!("\nRolodex:\n");
                                for contact_num in 0..player.rolodex.len() {
                                    println!(
                                        "{:>2}) {}",
                                        contact_num + 1,
                                        display_guest(&player.rolodex[contact_num])
                                    );
                                }
                                let mut input = String::new();
                                if let Err(e) = stdin().read_line(&mut input) {
                                    eprintln!("Error reading input: {}", e);
                                    continue 'party_input;
                                }
                                match input.trim() {
                                    i if i.parse::<usize>().map_or(false, |n| {
                                        (1..=player.rolodex.len()).contains(&n)
                                    }) =>
                                    {
                                        party.attendees[idx].ability_stock -= 1;
                                        let target = i.parse::<usize>().unwrap() - 1;
                                        party.attendees.push(player.rolodex[target].clone());
                                        player.rolodex.remove(target);
                                        let mut random_number = rng();
                                        player.rolodex.shuffle(&mut random_number);
                                        party.state = IncomingGuest {
                                            amount: party.attendees[party.attendees.len() - 1]
                                                .tagalongs,
                                            greet: false,
                                        };
                                        party.ability_state = false;
                                        boxed_message = "".to_string();
                                        return boxed_message;
                                    }
                                    "n" => {
                                        let mut random_number = rng();
                                        player.rolodex.shuffle(&mut random_number);
                                        party.state = IncomingGuest {
                                            amount: 0,
                                            greet: false,
                                        };
                                        party.ability_state = false;
                                        boxed_message = "".to_string();
                                        return boxed_message;
                                    }
                                    _ => {
                                        println!("Invalid input.");
                                        continue 'summoning_input;
                                    }
                                }
                            }
                        }
                    },
                    NoAbility => unreachable!(),
                }
            }
            _ => {
                println!("Invalid Input {}", input.trim());
                continue 'party_input;
            }
        }
    }
}
