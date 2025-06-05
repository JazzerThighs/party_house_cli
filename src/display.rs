use crate::{
    guest::{AbilityType::*, GuestType::*, *},
    party::{PartyState::*, *},
    player::*,
};
use clearscreen::clear;
use colored::*;
use std::{
    cmp::{max, min},
    f32::INFINITY,
    io::*,
};

pub fn pause_for_enter(prompt: &str) {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
}

pub const fn guest_type_display(guesttype: &GuestType) -> &str {
    match guesttype {
        OLD_FRIEND => "OLD_FRIEND",
        RICH_PAL => "RICH_PAL",
        WILD_BUDDY => "WILD_BUDDY",
        DRIVER => "DRIVER",
        MONKEY => "MONKEY",
        SECURITY => "SECURITY",
        TICKET_TKR => "TICKET_TKR",
        WATCH_DOG => "WATCH_DOG",
        HIPPY => "HIPPY",
        ROCK_STAR => "ROCK_STAR",
        COMEDIAN => "COMEDIAN",
        PRIVATE_I => "PRIVATE_I",
        INTROVERT => "INTROVERT",
        GRILLMASTR => "GRILLMASTR",
        MR_POPULAR => "MR_POPULAR",
        DANCER => "DANCER",
        AUCTIONEER => "AUCTIONEER",
        MASCOT => "MASCOT",
        WRESTLER => "WRESTLER",
        GANGSTER => "GANGSTER",
        CUTE_DOG => "CUTE_DOG",
        GAMBLER => "GAMBLER",
        SPY => "SPY",
        WRITER => "WRITER",
        PHOTOGRPHR => "PHOTOGRPHR",
        CHEERLEADR => "CHEERLEADR",
        COUNSELOR => "COUNSELOR",
        ATHLETE => "ATHLETE",
        CATERER => "CATERER",
        BARTENDER => "BARTENDER",
        CELEBRITY => "CELEBRITY",
        CUPID => "CUPID",
        MAGICIAN => "MAGICIAN",
        GREETER => "GREETER",
        CLIMBER => "CLIMBER",
        STYLIST => "STYLIST",
        WAREWOLF => "WAREWOLF",
        ALIEN => "ALIEN",
        MERMAID => "MERMAID",
        SUPERHERO => "SUPERHERO",
        DINOSAUR => "DINOSAUR",
        GENIE => "GENIE",
        DRAGON => "DRAGON",
        LEPRECHAUN => "LEPRECHAUN",
        UNICORN => "UNICORN",
        GHOST => "GHOST",
    }
}

fn ability_type_display(ability_type: &AbilityType) -> String {
    match ability_type {
        NoAbility => "   ".to_string(),
        Evac => "🔥 ".to_string(),
        Shutter => "📸 ".to_string(),
        Style(p) => format!("⬆️ {p}"),
        Quench => "🧯 ".to_string(),
        StarSwap => "🔄 ".to_string(),
        Boot => "🥾 ".to_string(),
        LoveArrow => "💘 ".to_string(),
        Cheer => "🎊 ".to_string(),
        Summoning => "⬇️  ".to_string(),
        Peek => "👀 ".to_string(),
        Greet => "🚪 ".to_string(),
    }
}

pub fn stylized_term_strings() -> (
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
    ColoredString,
) {
    (
        "NON_STAR_GUEST".to_string().white().on_black(),
        "*STAR_GUEST*".to_string().yellow().on_black(),
        "*NEG_STAR_GUEST*".to_string().yellow().on_red(),
        "POP".to_string().yellow().on_black(),
        "-POP".to_string().yellow().on_red(),
        "$_CASH".to_string().green().on_black(),
        "-$_CASH".to_string().green().on_red(),
        "X_TROUBLE".to_string().red().on_black(),
        "X_CHILL".to_string().black().on_white(),
        "+TAGALONGS".to_string().black().on_white(),
    )
}

#[allow(non_snake_case)]
fn ability_type_info(ability_type: &AbilityType) -> String {
    let (GUEST, STAR, NEG_STAR, POP, NEG_POP, CASH, NEG_CASH, TROUBLE, _CHILL, TAGALONGS) =
        stylized_term_strings();
    match ability_type {
        NoAbility => "".to_string(),
        Evac => format!("Evac 🔥: Remove all attendees from the party and reshuffle them back into the rolodex."),
        Shutter => format!("Shutter 📸: Score a single attendee for their {POP}/{NEG_POP}, {CASH}/{NEG_CASH}, and Bonuses."),
        Style(_) => format!("Style ⬆️ : Increase the {POP} of one attendee by a designated amount."),
        Quench => format!("Quench 🧯: Chill out all {TROUBLE} currently in the party."),
        StarSwap => format!("StarSwap 🔄: Swap out a {GUEST} attendee for a {STAR}/{NEG_STAR} from the rolodex, or swap out a {STAR}/{NEG_STAR} attendee for a {GUEST} from the rolodex."),
        Boot => format!("Boot 🥾: Kick out 1 attendee, also prevents them from coming back today."),
        LoveArrow => format!("LoveArrow 💘: Kick out 2 adjacent attendees, also provents them from coming back today."),
        Cheer => format!("Cheer 🎊:  Replenish all non-Cheer abilities for all of the attendees."),
        Summoning => format!("Summoning ⬇️ : Summon 1 guest from the rolodex to join the party."),
        Peek => format!("Peek 👀: Find out who is next in line at the front door; If they are then booted, that guest is prevented from coming back today."),
        Greet => format!("Greet 🚪: Open the door for the next guest in line for the party, score them for their {POP}/{NEG_POP}, {CASH}/{NEG_CASH}, and Bonuses, as well as doing the same for any {TAGALONGS} that they bring with them."),
    }
}

#[allow(non_snake_case)]
pub fn guest_type_info(guest_type: &GuestType) -> String {
    let (_GUEST, _STAR, _NEG_STAR, POP, _NEG_POP, CASH, _NEG_CASH, TROUBLE, _CHILL, _TAGALONGS) =
        stylized_term_strings();
    match guest_type {
        COMEDIAN => format!("+5 {POP} Bonus if the party is full to capacity."),
        INTROVERT => format!("+1 {POP} Bonus for every empty spot in the party."),
        DANCER => format!("\n +1 {POP} Bonus => 1 DANCER present.\n +4 {POP} Bonus => 2 DANCERs present.\n +9 {POP} Bonus => 3 DANCERs present.\n +16 {POP} Bonus => 4 or more DANCERs present."),
        MASCOT => format!("+1 {POP} Bonus for every OLD_FRIEND present."),
        WRITER => format!("+2 {POP} Bonus for each {TROUBLE} present."),
        BARTENDER => format!("+2 {CASH} Bonus for each {TROUBLE} present."),
        CLIMBER => format!("+1 {POP} added to Base Stat each time they enter a party."),
        WAREWOLF => format!("Toggles between {TROUBLE} and Zero-{TROUBLE} each time they enter a party."),
        OLD_FRIEND |
        RICH_PAL |
        WILD_BUDDY |
        DRIVER |
        MONKEY |
        SECURITY |
        TICKET_TKR |
        WATCH_DOG |
        HIPPY |
        ROCK_STAR |
        PRIVATE_I |
        GRILLMASTR |
        MR_POPULAR |
        AUCTIONEER |
        WRESTLER |
        GANGSTER |
        CUTE_DOG |
        GAMBLER |
        SPY |
        PHOTOGRPHR |
        CHEERLEADR |
        COUNSELOR |
        ATHLETE |
        CATERER |
        CELEBRITY |
        CUPID |
        MAGICIAN |
        GREETER |
        STYLIST |
        ALIEN |
        MERMAID |
        SUPERHERO |
        DINOSAUR |
        GENIE |
        DRAGON |
        LEPRECHAUN |
        UNICORN |
        GHOST => String::default()
    }
}

pub fn display_guest(guest: &Guest) -> String {
    format!(
        "{:>12} {:>2} {:>2} {} {:>2} {}",
        match *guest.stars {
            -1 => format!("*{}*", guest_type_display(&guest.guest_type))
                .yellow()
                .on_red(),
            1 => format!("*{}*", guest_type_display(&guest.guest_type))
                .yellow()
                .on_black(),
            _ => format!("{}", guest_type_display(&guest.guest_type))
                .white()
                .on_black(),
        },
        match *guest.popularity {
            x if x > 0 => x.to_string().yellow().on_black(),
            x if x < 0 => x.to_string().yellow().on_red(),
            _ => "".to_string().white().on_black(),
        },
        match *guest.cash {
            x if x > 0 => x.to_string().green().on_black(),
            x if x < 0 => x.to_string().green().on_red(),
            _ => "".to_string().white().on_black(),
        },
        match (guest.trouble, guest.chill) {
            (true, _) => "X".red().on_black(),
            (false, true) => "X".black().on_white(),
            (_, _) => " ".white().on_black(),
        },
        match guest.tagalongs {
            0 => "".white().on_black(),
            1.. => format!("+{}", guest.tagalongs).black().on_white(),
        },
        match guest.ability_stock {
            0 => "   ".to_string(),
            1.. => ability_type_display(&guest.ability_type),
        }
    )
}

#[allow(non_snake_case)]
pub fn display_information() {
    let (GUEST, STAR, NEG_STAR, POP, NEG_POP, CASH, NEG_CASH, TROUBLE, CHILL, TAGALONGS) =
        stylized_term_strings();

    clear().unwrap();

    println!("Party House is a Deck Builder! During the Party phase, invite guests by opening the front door. Guests will be pulled at random from your rolodex, and the goal is to invite lots of guests so that you end up with lots of {POP} and {CASH} to spend in the store. At the end of the party, the guests' {POP} and {CASH} attributes will be tallied up, along with any guest-specific Bonuses that they possess. {POP} is self-explanatory, but {CASH} is not. If you end up with less {POP} than you started with, you just end up with 0 {POP}. However, if you go into the negatives for {CASH}, each {CASH} point will be instead deducted from your {POP} balance, -7 {POP} points per {CASH} point.\nOne must worry about {TROUBLE}. If too many {TROUBLE}makers show up to the party, the police will be called and the party will end without granting you any {POP}, {CASH}, or Bonuses! Same goes for if the party overflows; If a guest has {TAGALONGS}, that means that they will forcably bring along their own plus-1s (from your rolodex), even if those plus-1s cannot fit into the house, and the fire marshal will shut it down! Be careful if you have them in the deck, because they can also bring each other for massive chains of unwanted guests!");
    println!("\nWin Condition: End a party successfully when {STAR}s minus {NEG_STAR}s is greater than or equal to the designated amount.");

    println!("\nUniversal Guest Attributes:\n{GUEST}/{STAR}/{NEG_STAR}\n{POP}/{NEG_POP}\n{CASH}\n{NEG_CASH}\n{TROUBLE}/{CHILL}\n{TAGALONGS}\nAbility_Available_Symbol");

    let (_, _, _, all_guests) = guest_lists();
    let mut v: Vec<AbilityType> = vec![];
    for (i, _) in all_guests.iter() {
        if !v.contains(&all_guests[i].ability_type) {
            v.push(all_guests[i].ability_type.clone())
        }
    }
    println!("\nAbility Types:");
    for i in v.iter() {
        if *i != NoAbility {
            println!("{}", ability_type_info(i));
        }
    }

    println!("\nSpecial guest Effects:");
    for (i, _) in all_guests.iter() {
        if !guest_type_info(&all_guests[i].guest_type).is_empty() {
            println!(
                "{}: {}",
                guest_type_display(i),
                guest_type_info(&all_guests[i].guest_type)
            );
        }
    }
}

pub fn party_display(
    party: &Party,
    player: &Player,
    victories: &Vec<bool>,
    day_count: usize,
    boxed_message: &String,
) {
    clear().unwrap();
    match victories.len() {
        1 => {}
        2.. => {
            print!("Ultimate Parties: [");
            for v in victories.iter() {
                match v {
                    true => print!(" {} ", "☑".to_string().yellow().on_black()),
                    false => print!(" {} ", "☐".to_string().red().on_black()),
                }
            }
            println!("]")
        }
        0 => unreachable!(),
    }
    println!("Player {}, throw a party!", player.id + 1);
    println!(
        "| POP: {:>2}/65 | $: {:>2}/30 |",
        (*player.popularity).to_string().yellow(),
        (*player.cash).to_string().green()
    );
    println!(
        "{}Stars: {}/{}{} | {} | Capacity: {}/35",
        "*".yellow(),
        max(
            0,
            party.attendees.iter().filter(|a| *a.stars == 1).count() as i8
                - party.attendees.iter().filter(|a| *a.stars == -1).count() as i8
        ),
        party.stars_to_win,
        "*".yellow(),
        match victories.len() {
            1 => format!("Day {}/25", day_count),
            2.. => format!("Day {}", day_count),
            0 => unreachable!(),
        },
        *party.capacity
    );
    println!("[ {} ]", boxed_message.black().on_white());
    print!(
        "Controls:\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n",
        match party.ability_state {
            true => "",
            false => "\"h\" => Open the door",
        },
        match party.ability_state {
            true => "",
            false => "\"r\" => View your rolodex",
        },
        match party.ability_state {
            true => "",
            false => "\"e\" => End the party",
        },
        match party.ability_state {
            true => "",
            false => "\"i\" => View Information",
        },
        match party.ability_state {
            true => "",
            false => match party.peek_slot {
                Some(_) => "\"b\" => Boot the guest at the front door",
                None => "",
            },
        },
        match party.ability_state {
            true => "\"n\" => Decide not to use the currently selected ability",
            false => "",
        },
        match party.ability_state {
            true => "".to_string(),
            false => format!(
                "Integers 1..={} => Use that attendee's ability",
                *party.capacity
            ),
        },
        match (
            &party.state,
            party.attendees.iter().filter(|g| g.trouble).count() as i8
                - party.attendees.iter().filter(|g| g.chill).count() as i8
                == 2
        ) {
            (EndedSuccessfully | TooMuchTrouble | Overcrowded, _) => "".white().on_black(),
            (_, false) => "".white().on_black(),
            (_, true) => "X This party is getting out of hand! X".red().on_black(),
        }
    );
    match &party.peek_slot {
        Some(p) => println!("🚪) {}", display_guest(p)),
        None => println!(),
    }
    for i in 0..*party.capacity as usize {
        println!(
            "{:>2}) {}",
            i + 1,
            match i < party.attendees.len() {
                true => display_guest(&party.attendees[i]),
                false => "".to_string(),
            }
        );
    }
    if party.attendees.len() > *party.capacity as usize {
        println!(
            "{:>2}{} {} {}",
            party.attendees.len().to_string().white().on_red(),
            ")".white().on_red(),
            display_guest(&party.attendees[party.attendees.len() - 1]),
            "=> Overflow!".black().on_red()
        );
    }
}

#[allow(non_snake_case)]
pub fn display_end_of_party_info(party: &Party) {
    let (_GUEST, _STAR, _NEG_STAR, POP, NEG_POP, CASH, NEG_CASH, _TROUBLE, _CHILL, _TAGALONGS) =
        stylized_term_strings();

    if party
        .attendees
        .iter()
        .filter(|a| *a.popularity >= 0)
        .map(|a| *a.popularity)
        .sum::<i8>()
        != 0
    {
        println!(
            "\nSum of {POP} attributes: {}",
            party
                .attendees
                .iter()
                .filter(|a| *a.popularity >= 0)
                .map(|a| *a.popularity)
                .sum::<i8>()
                .to_string()
                .yellow()
                .on_black()
        );
    }
    if party
        .attendees
        .iter()
        .filter(|a| *a.popularity < 0)
        .map(|a| *a.popularity)
        .sum::<i8>()
        != 0
    {
        println!(
            "Sum of {NEG_POP} attributes: {}",
            party
                .attendees
                .iter()
                .filter(|a| *a.popularity < 0)
                .map(|a| *a.popularity)
                .sum::<i8>()
                .to_string()
                .yellow()
                .on_red()
        );
    }
    if party
        .attendees
        .iter()
        .filter(|a| *a.cash >= 0)
        .map(|a| *a.cash)
        .sum::<i8>()
        != 0
    {
        println!(
            "Sum of {CASH} attributes: {}",
            party
                .attendees
                .iter()
                .filter(|a| *a.cash >= 0)
                .map(|a| *a.cash)
                .sum::<i8>()
                .to_string()
                .green()
                .on_black()
        );
    }
    if party
        .attendees
        .iter()
        .filter(|a| *a.cash < 0)
        .map(|a| *a.cash)
        .sum::<i8>()
        != 0
    {
        println!(
            "Sum of {NEG_CASH} attributes: {}",
            party
                .attendees
                .iter()
                .filter(|a| *a.cash < 0)
                .map(|a| *a.cash)
                .sum::<i8>()
                .to_string()
                .green()
                .on_red()
        );
    }
    for a in party.attendees.iter().filter(|a| (a.bonus_pop)(&party) > 0) {
        println!(
            "{} {POP} Bonus: {}",
            guest_type_display(&a.guest_type),
            (a.bonus_pop)(&party).to_string().yellow().on_black()
        )
    }
    if party
        .attendees
        .iter()
        .filter(|a| a.guest_type == GuestType::DANCER)
        .count()
        > 0
    {
        println!(
            "DANCER {POP} Bonus: {}",
            min(
                16,
                party
                    .attendees
                    .iter()
                    .filter(|a| a.guest_type == GuestType::DANCER)
                    .count()
                    .pow(2) as i8
            )
            .to_string()
            .yellow()
            .on_black()
        );
    }
    for a in party.attendees.iter().filter(|a| (a.bonus_pop)(&party) < 0) {
        println!(
            "{} {NEG_POP} Bonus: {}",
            guest_type_display(&a.guest_type),
            (a.bonus_pop)(&party).to_string().yellow().on_red()
        )
    }
    for a in party
        .attendees
        .iter()
        .filter(|a| (a.bonus_cash)(&party) > 0)
    {
        println!(
            "{} {CASH} Bonus: {}",
            guest_type_display(&a.guest_type),
            (a.bonus_cash)(&party).to_string().green().on_black()
        )
    }
    for a in party
        .attendees
        .iter()
        .filter(|a| (a.bonus_cash)(&party) < 0)
    {
        println!(
            "{} {NEG_CASH} Bonus: {}",
            guest_type_display(&a.guest_type),
            (a.bonus_cash)(&party).to_string().green().on_red()
        )
    }
    pause_for_enter("Press enter to continue...");
}

#[allow(non_snake_case)]
pub fn store_display(
    store: &Vec<(Guest, f32)>,
    player: &Player,
    victories: &Vec<bool>,
    day_count: usize,
    stars_to_win: &usize,
    boxed_message: &String,
) {
    let (_GUEST, _STAR, _NEG_STAR, POP, _NEG_POP, CASH, _NEG_CASH, _TROUBLE, _CHILL, _TAGALONGS) =
        stylized_term_strings();
    
    clear().unwrap();
    
    println!("Player {}, spend {POP} to add guests to your rolodex. Spend {CASH} to expand the capacity of your house:", player.id + 1);
    match victories.len() {
        1 => {}
        2.. => {
            print!("Ultimate Parties: [");
            for v in victories.iter() {
                match v {
                    true => print!(" {} ", "☑".to_string().yellow().on_black()),
                    false => print!(" {} ", "☐".to_string().red().on_black()),
                }
            }
            println!("]")
        }
        0 => unreachable!(),
    }
    println!(
        "{}",
        match victories.len() {
            1 => format!("Going into Day {}/25", day_count + 1),
            2.. => format!("Going into Day {}", day_count + 1),
            0 => unreachable!(),
        }
    );
    println!(
        "Win Condition: {} Stars in One Party",
        stars_to_win.to_string().yellow().on_black()
    );
    println!(
        "| {POP}: {:>2}/65 | {CASH}: {:>2}/30 | Capacity: {:>2}/34 | Rolodex: {:>2} |\n",
        (*player.popularity).to_string().yellow(),
        (*player.cash).to_string().green(),
        (*player.capacity).to_string().white().on_black(),
        player.rolodex.len()
    );
    print!(
        "Controls:\n {}\n {}\n {}\n {}\n {}{} {}\n\n",
        "\"r\" => View your rolodex",
        match *player.capacity {
            5..=33 => "\"c\" => Increase the capacity of your house",
            34.. => "",
            ..=4 => unreachable!(),
        },
        "\"e\" => Finish Shopping and move on to the next day of partying",
        "\"i\" => View Information",
        "Integers 1..=",
        store.len(),
        "=> Add one copy of that contact to your rolodex"
    );
    print!("[ {} ]\n\n", boxed_message.black().on_white());
    for i in 0..store.len() {
        let store_stock = format!(
            "{:>2}) {}  {}",
            i + 1,
            display_guest(&store[i].0),
            match store[i].1 {
                0.0 => "Sold Out!".to_string(),
                INFINITY => format!(
                    "× ∞ => Cost: {:>2}",
                    store[i].0.cost.to_string().yellow().on_black()
                ),
                _ => format!(
                    "× {} => Cost: {:>2}",
                    store[i].1,
                    store[i].0.cost.to_string().yellow().on_black()
                ),
            },
        );
        println!(
            "{}{store_stock}",
            if *player.cash >= store[i].0.cost as i8 {
                "🏷️"
            } else {
                "❌"
            }
        );
    }
    match *player.capacity {
        5..=15 => println!(
            "Upgrade Capacity => Cost: {}",
            (*player.capacity - 3).to_string().green().on_black()
        ),
        16..=33 => println!("Upgrade Capacity => Cost: {}", "12".green().on_black()),
        34.. => println!("House Capacity Maxed Out! (34 Spots Max)"),
        ..=4 => unreachable!(),
    }
}
