use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Guest {
    pub id: usize,
    pub guest: GuestType,
    pub emoji: char,
    pub cost: u8,
    pub popularity: i8,
    pub cash: i8,
    pub trouble: bool,
    pub chill: bool,
    pub stars: i8,
    pub tagalongs: u8,
    pub summonings: u8,
    pub boots: u8,
    pub evacs: u8,
    pub camera_flashes: u8,
    pub peeks: u8,
    pub star_swaps: u8,
    pub greets: u8,
    pub pompoms: u8,
    pub quenches: u8,
    pub pop_ups: u8,
    pub arrived_already_today: bool,
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GuestType {
    #[default]
    OLD_FRIEND,
    RICH_PAL,
    WILD_BUDDY,
    DRIVER,
    MONKEY,
    SECURITY,
    TICKET_TKR,
    WATCH_DOG,
    HIPPY,
    ROCK_STAR,
    COMEDIAN,
    PRIVATE_I,
    INTROVERT,
    GRILLMASTR,
    MR_POPULAR,
    DANCER,
    AUCTIONEER,
    MASCOT,
    WRESTLER,
    GANGSTER,
    CUTE_DOG,
    GAMBLER,
    SPY,
    WRITER,
    PHOTOGRPHR,
    CHEERLEADR,
    COUNSELOR,
    ATHLETE,
    CATERER,
    BARTENDER,
    CELEBRITY,
    CUPID,
    MAGICIAN,
    GREETER,
    CLIMBER,
    STYLIST,
    WAREWOLF,
    ALIEN,
    MERMAID,
    SUPERHERO,
    DINOSAUR,
    GENIE,
    DRAGON,
    LEPRECHAUN,
    UNICORN,
    GHOST,
}

pub fn guest_lists() -> (
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
) {
    let (mut friends, mut randos, mut star_guests) =
        (HashMap::new(), HashMap::new(), HashMap::new());
    use GuestType::*;
    friends.insert(
        OLD_FRIEND,
        Guest {
            guest: OLD_FRIEND,
            emoji: '🙂',
            cost: 2,
            popularity: 1,
            ..Default::default()
        },
    );
    friends.insert(
        RICH_PAL,
        Guest {
            guest: RICH_PAL,
            emoji: '🤑',
            cost: 3,
            cash: 1,
            ..Default::default()
        },
    );
    friends.insert(
        WILD_BUDDY,
        Guest {
            guest: WILD_BUDDY,
            emoji: '🤮',
            popularity: 2,
            trouble: true,
            ..Default::default()
        },
    );
    randos.insert(
        DRIVER,
        Guest {
            guest: DRIVER,
            emoji: '🚗',
            cost: 3,
            summonings: 1,
            ..Default::default()
        },
    );
    randos.insert(
        MONKEY,
        Guest {
            guest: MONKEY,
            emoji: '🐒',
            cost: 3,
            popularity: 4,
            trouble: true,
            ..Default::default()
        },
    );
    randos.insert(
        SECURITY,
        Guest {
            guest: SECURITY,
            emoji: '👮',
            cost: 4,
            boots: 1,
            ..Default::default()
        },
    );
    randos.insert(
        TICKET_TKR,
        Guest {
            guest: TICKET_TKR,
            emoji: '🎫',
            cost: 4,
            popularity: -1,
            cash: 2,
            ..Default::default()
        },
    );
    randos.insert(
        WATCH_DOG,
        Guest {
            guest: WATCH_DOG,
            emoji: '🦮',
            cost: 4,
            popularity: 2,
            peeks: 1,
            ..Default::default()
        },
    );
    randos.insert(
        HIPPY,
        Guest {
            guest: HIPPY,
            emoji: '✌',
            cost: 4,
            popularity: 1,
            chill: true,
            ..Default::default()
        },
    );
    randos.insert(
        ROCK_STAR,
        Guest {
            guest: ROCK_STAR,
            emoji: '🎸',
            cost: 5,
            popularity: 3,
            cash: 2,
            trouble: true,
            ..Default::default()
        },
    );
    randos.insert(
        COMEDIAN,
        Guest {
            guest: COMEDIAN,
            emoji: '🤣',
            cost: 5,
            cash: -1,
            ..Default::default()
        },
    );
    randos.insert(
        PRIVATE_I,
        Guest {
            guest: PRIVATE_I,
            emoji: '🕵',
            cost: 4,
            popularity: 2,
            cash: -1,
            summonings: 1,
            ..Default::default()
        },
    );
    randos.insert(
        INTROVERT,
        Guest {
            guest: INTROVERT,
            emoji: '😶',
            cost: 4,
            popularity: 1,
            ..Default::default()
        },
    );
    randos.insert(
        GRILLMASTR,
        Guest {
            guest: GRILLMASTR,
            emoji: '🍔',
            cost: 5,
            popularity: 2,
            evacs: 1,
            ..Default::default()
        },
    );
    randos.insert(
        MR_POPULAR,
        Guest {
            guest: MR_POPULAR,
            emoji: '😎',
            cost: 5,
            popularity: 3,
            tagalongs: 1,
            ..Default::default()
        },
    );
    randos.insert(
        DANCER,
        Guest {
            guest: DANCER,
            emoji: '💃',
            cost: 7,
            ..Default::default()
        },
    );
    randos.insert(
        AUCTIONEER,
        Guest {
            guest: AUCTIONEER,
            emoji: '🤠',
            cost: 9,
            cash: 3,
            ..Default::default()
        },
    );
    randos.insert(
        MASCOT,
        Guest {
            guest: MASCOT,
            emoji: '😸',
            cost: 5,
            popularity: 1,
            ..Default::default()
        },
    );
    randos.insert(
        WRESTLER,
        Guest {
            guest: WRESTLER,
            emoji: '👊',
            cost: 9,
            popularity: 2,
            boots: 1,
            ..Default::default()
        },
    );
    randos.insert(
        GANGSTER,
        Guest {
            guest: GANGSTER,
            emoji: '🔫',
            cost: 6,
            cash: 4,
            trouble: true,
            ..Default::default()
        },
    );
    randos.insert(
        CUTE_DOG,
        Guest {
            guest: CUTE_DOG,
            emoji: '🐶',
            cost: 7,
            popularity: 2,
            chill: true,
            ..Default::default()
        },
    );
    randos.insert(
        GAMBLER,
        Guest {
            guest: GAMBLER,
            emoji: '🎰',
            cost: 7,
            popularity: 2,
            cash: 3,
            trouble: true,
            ..Default::default()
        },
    );
    randos.insert(
        SPY,
        Guest {
            guest: SPY,
            emoji: '🍸',
            cost: 8,
            cash: 2,
            peeks: 1,
            ..Default::default()
        },
    );
    randos.insert(
        WRITER,
        Guest {
            guest: WRITER,
            emoji: '🖋',
            cost: 8,
            popularity: 1,
            ..Default::default()
        },
    );
    randos.insert(
        PHOTOGRPHR,
        Guest {
            guest: PHOTOGRPHR,
            emoji: '📷',
            cost: 5,
            popularity: 1,
            cash: -1,
            camera_flashes: 1,
            ..Default::default()
        },
    );
    randos.insert(
        CHEERLEADR,
        Guest {
            guest: CHEERLEADR,
            emoji: '🎉',
            cost: 5,
            popularity: 1,
            pompoms: 1,
            ..Default::default()
        },
    );
    randos.insert(
        COUNSELOR,
        Guest {
            guest: COUNSELOR,
            emoji: '📋',
            cost: 7,
            quenches: 1,
            ..Default::default()
        },
    );
    randos.insert(
        ATHLETE,
        Guest {
            guest: ATHLETE,
            emoji: '⚽',
            cost: 6,
            popularity: 1,
            cash: 1,
            evacs: 1,
            ..Default::default()
        },
    );
    randos.insert(
        CATERER,
        Guest {
            guest: CATERER,
            emoji: '🍣',
            cost: 5,
            popularity: 4,
            cash: -1,
            ..Default::default()
        },
    );
    randos.insert(
        BARTENDER,
        Guest {
            guest: BARTENDER,
            emoji: '🍺',
            cost: 11,
            popularity: 1,
            ..Default::default()
        },
    );
    randos.insert(
        CELEBRITY,
        Guest {
            guest: CELEBRITY,
            emoji: '👸',
            cost: 11,
            popularity: 3,
            tagalongs: 2,
            ..Default::default()
        },
    );
    randos.insert(
        CUPID,
        Guest {
            guest: CUPID,
            emoji: '💘',
            cost: 8,
            popularity: 1,
            boots: 2,
            ..Default::default()
        },
    );
    randos.insert(
        MAGICIAN,
        Guest {
            guest: MAGICIAN,
            emoji: '🧙',
            cost: 5,
            popularity: 1,
            star_swaps: 1,
            ..Default::default()
        },
    );
    randos.insert(
        GREETER,
        Guest {
            guest: GREETER,
            emoji: '🤵',
            cost: 5,
            popularity: 1,
            greets: 1,
            ..Default::default()
        },
    );
    randos.insert(
        CLIMBER,
        Guest {
            guest: CLIMBER,
            emoji: '🤳',
            cost: 12,
            ..Default::default()
        },
    );
    randos.insert(
        STYLIST,
        Guest {
            guest: STYLIST,
            emoji: '✂',
            cost: 7,
            cash: -1,
            pop_ups: 1,
            ..Default::default()
        },
    );
    randos.insert(
        WAREWOLF,
        Guest {
            guest: WAREWOLF,
            emoji: '🐺',
            cost: 5,
            popularity: 4,
            trouble: true,
            ..Default::default()
        },
    );
    star_guests.insert(
        ALIEN,
        Guest {
            guest: ALIEN,
            emoji: '👽',
            cost: 40,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        MERMAID,
        Guest {
            guest: MERMAID,
            emoji: '🧜',
            cost: 35,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        SUPERHERO,
        Guest {
            guest: SUPERHERO,
            emoji: '🦸',
            cost: 50,
            popularity: 3,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        DINOSAUR,
        Guest {
            guest: DINOSAUR,
            emoji: '🦖',
            cost: 25,
            trouble: true,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        GENIE,
        Guest {
            guest: GENIE,
            emoji: '🧞',
            cost: 55,
            stars: 1,
            summonings: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        DRAGON,
        Guest {
            guest: DRAGON,
            emoji: '🐲',
            cost: 30,
            cash: -3,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        LEPRECHAUN,
        Guest {
            guest: LEPRECHAUN,
            emoji: '🍀',
            cost: 50,
            cash: 3,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        UNICORN,
        Guest {
            guest: UNICORN,
            emoji: '🦄',
            cost: 45,
            chill: true,
            stars: 1,
            ..Default::default()
        },
    );
    star_guests.insert(
        GHOST,
        Guest {
            guest: GHOST,
            emoji: '👻',
            cost: 45,
            stars: 1,
            boots: 1,
            ..Default::default()
        },
    );

    (friends, randos, star_guests)
}
