use crate::{
    guest::{AbilityType::*, GuestType::*, *},
    party::*,
    player::*,
};
use clearscreen::clear;
use colored::*;
use std::{cmp::max, f32::INFINITY, io::*};

pub fn pause_for_enter(prompt: &str) {
    print!("{}", prompt);
    // Flush the output so it prints immediately without waiting for a newline
    stdout().flush().unwrap();

    let mut buffer = String::new();
    // This will block until user presses Enter
    stdin().read_line(&mut buffer).unwrap();
}

const fn guest_type_display(guesttype: &GuestType) -> &str {
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

const fn ability_type_display(ability_type: &AbilityType) -> &str {
    match ability_type {
        NoAbility => "  ",
        Evac => "🔥",
        Shutter(_) => "📸",
        Style(_) => "⬆️ ",
        Quench => "🧯",
        StarSwap(_) => "🔄",
        Boot(_) => "🥾",
        LoveArrow(_) => "💘",
        Cheer => "🎊",
        Summoning => "⬇️ ",
        Peek => "👀",
        Greet => "🚪",
    }
}

pub fn display_guest(guest: &Guest) -> String {
    format!(
        "{:>12} {:>2} {:>2} {} {:>2} {}",
        match *guest.stars {
            -1 => format!("*{}*", guest_type_display(&guest.guest_type)).yellow().on_red(),
            1 => format!("*{}*", guest_type_display(&guest.guest_type)).yellow().on_black(),
            _ => format!("{}", guest_type_display(&guest.guest_type)).white().on_black()
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
            1.. => format!("+{}", guest.tagalongs).black().on_white()
        },
        match guest.ability_stock {
            0 => "  ",
            1.. => ability_type_display(&guest.ability_type),
        }
    )
}

pub fn party_display(
    party: &Party,
    player: &Player,
    victories: &Vec<bool>,
    day_count: usize,
    boxed_message: &String,
) {
    clear().unwrap();
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
            0 => unreachable!()
        },
        *party.capacity
    );
    // if victories.iter().any(|v| *v) {
    //     for v in victories.iter() {
    //         if *v {
    //             println!("Player {} won today!", player.id + 1)
    //         };
    //     }
    //     println!("Last Chance!\n");
    // }
    println!("[ {} ]", boxed_message.black().on_white());
    print!(
        "Controls:\n {}\n {}\n {}\n {}\n {}\n {}{} {}\n {}\n",
        "\"h\" => Open the door",
        "\"r\" => View your rolodex",
        "\"e\" => End the party",
        match party.peek_slot {
            Some(_) => "\"b\" => Boot the guest at the front door",
            None => "",
        },
        match party.ability_state {
            true => "\"n\" => Decide not to use the currently selected ability",
            false => "",
        },
        "Integers 1..=",
        *party.capacity,
        "=> Use that attendee's ability",
        match party.attendees.iter().filter(|g| g.trouble).count() as i8
            - party.attendees.iter().filter(|g| g.chill).count() as i8
            == 2 {
                true => "X This party is getting out of hand! X".red().on_black(),
                false => "".white().on_black()
            }
    );
    match &party.peek_slot {
        Some(p) => println!("🚪) {}", display_guest(p)),
        None => println!()
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
    // show that the party overflowed if it did
    if party.attendees.len() > *party.capacity as usize {
        println!(
            "{:>2}) {} {}",
            party.attendees.len(),
            display_guest(&party.attendees[party.attendees.len() - 1]),
            "=> Overflow!".black().on_red()
        );
    }
}

pub fn store_display(store: &Vec<(Guest, f32)>, player: &Player, victories: &Vec<bool>, day_count: usize, stars_to_win: &usize, boxed_message: &String) {
    clear().unwrap();
    println!("Player {}, spend Pop to add guests to your rolodex. Spend Cash to expand the capacity of your house:", player.id + 1);
    println!("{}", 
        match victories.len() {
            1 => format!("Going into Day {}/25", day_count + 1),
            2.. => format!("Going into Day {}", day_count + 1),
            0 => unreachable!()
        }
    );
    println!("Win Condition: {} Stars in One Party", stars_to_win.to_string().yellow().on_black());
    println!(
        "| POP: {:>2}/65 | $: {:>2}/30 | Capacity: {:>2}/34 | Rolodex: {:>2} |\n",
        (*player.popularity).to_string().yellow(),
        (*player.cash).to_string().green(),
        (*player.capacity).to_string().white().on_black(),
        player.rolodex.len()
    );
    print!(
        "Controls:\n {}\n {}\n {}\n {}{} {}\n\n",
        "\"r\" => View your rolodex",
        match *player.capacity {
            5..=33 => "\"c\" => Increase the capacity of your house",
            34.. => "",
            ..=4 => unreachable!(),
        },
        "\"e\" => Finish Shopping and move on to the next day of partying",
        "Integers 1..=",
        store.len(),
        "=> Add one copy of that contact to your rolodex"
    );
    print!("[ {} ]\n\n", boxed_message.black().on_white());
    for i in 0..store.len() {
        println!(
            "{:>2}) {}  {}",
            i + 1,
            display_guest(&store[i].0),
            match store[i].1 {
                0.0 => "Sold Out!".to_string(),
                INFINITY => format!("× ∞ => Cost: {:>2}", store[i].0.cost.to_string().yellow().on_black()),
                _ => format!("× {} => Cost: {:>2}", store[i].1, store[i].0.cost.to_string().yellow().on_black()),
            },
        );
    }
    match *player.capacity {
        5..=15 => println!("Upgrade Capacity => Cost: {}", (*player.capacity - 3).to_string().green().on_black()),
        16..=33 => println!("Upgrade Capacity => Cost: {}", "12".green().on_black()),
        34.. => println!("House Capacity Maxed Out! (34 Spots Max)"),
        ..=4 => unreachable!(),
    }
}
