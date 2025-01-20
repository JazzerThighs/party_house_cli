use clearscreen::clear;
use crate::{
    guest::{AbilityType::*, FullHouseAbilityCondition::*, GuestType::*, *},
    init::*,
    party::{PartyState::*, *},
    store::*, Player,
};

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
        NoAbility => "",
        Evac => "🔥",
        Shutter => "📸",
        Style(x) => "⬆️{x}",
        Quench => "🧯",
        StarSwap => "🔄",
        Boot => "🥾",
        LoveArrow => "💘",
        Cheer => "🎊",
        Summoning => "⬇️",
        Peek => "👀",
        Greet => "🚪",
    }
}

pub fn display_attendee(guest: &Guest) -> String {
    format!(
        "{:>10} {:>2} {:>2} {:>2} {:>2} {:>2}",
        guest_type_display(&guest.guest_type),
        guest.emoji,
        match (guest.trouble, guest.chill) {
            (true, _) => "❌",
            (false, true) => "🕊️",
            (_, _) => ""
        },
        match *guest.popularity {
            x if x < 0 || x > 0 => {
                x.to_string()
            }
            _ => {
                "".to_string()
            }
        },
        match *guest.cash {
            x if x < 0 || x > 0 => {
                x.to_string()
            }
            _ => {
                "".to_string()
            }
        },
        match guest.ability_stock {
            0 => "",
            1.. => ability_type_display(&guest.ability_type)
        }
    )
}

pub fn party_display(party: &Party, player: &Player, victories: &Vec<bool>, boxed_message: String) {
    clear().unwrap();
    println!("Player {}, throw a party!", player.id);
    if victories.iter().any(|v| *v) {
        for v in victories.iter() {
            if *v {
                println!("Player {} won today!", player.id + 1)
            };
        }
        println!("Last Chance!\n");
    }
    for i in 0..*party.capacity { todo!() }
}

pub fn store_display(store: Store, player: &Player, boxed_message: String) {
    clear().unwrap();
    println!("Player {}, spend Pop to add guests to your rolodex; Spend Cash to expand the capacity of your house:\n", player.id + 1);
    todo!()
}