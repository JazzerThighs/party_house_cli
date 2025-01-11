use crate::Party;
use better_default::Default;
use std::{cmp::max, collections::HashMap};

#[derive(Default, Debug, Clone)]
pub struct Guest {
    pub id: usize,
    pub sort_value: u8,
    pub guest: GuestType,
    pub emoji: char,
    pub cost: u8,
    pub popularity: i8,
    pub cash: i8,
    pub trouble: bool,
    pub chill: bool,
    pub stars: i8,
    pub tagalongs: u8,
    #[default(|_| 0)] 
    pub bonus_pop: fn(&Party) -> i8,
    #[default(|_| 0)] 
    pub bonus_cash: fn(&Party) -> i8,
    pub arrived_already_today: bool,
    pub ability_type: AbilityType,
    pub ability_base: u8,
    pub ability_stock: u8,
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

#[derive(Default, Debug, Clone, PartialEq)]
pub enum AbilityType {
    #[default]
    NoAbility,

    // Able to be used when house is full:
    Boot,
    LoveArrow,
    Evac,
    Shutter,
    Style,
    Quench,
    StarSwap,

    // Able to be used when the house is full and there is at least 1 guest who has a full house ability in the party:
    Cheer,

    // Not able to be used when house is full:
    Summoning,
    Peek,
    Greet,
}

pub fn guest_lists() -> (
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
) {
    let (mut friends, mut randos, mut star_guests) =
        (HashMap::new(), HashMap::new(), HashMap::new());
    use AbilityType::*;
    use GuestType::*;
    macro_rules! insert_guest {
        ($map:expr, $guest:ident $(, $field:ident : $value:expr )* $(,)?) => {
            $map.insert(
                $guest,
                Guest {
                    guest: $guest,
                    $( $field: $value, )*
                    ..Default::default()
                }
            );
        };
    }
    insert_guest!(
        friends,
        OLD_FRIEND,
        sort_value: 0,
        emoji: '🙂',
        cost: 2,
        popularity: 1
    );
    insert_guest!(
        friends,
        RICH_PAL,
        sort_value: 1,
        emoji: '🤑',
        cost: 3,
        cash: 1,
    );
    insert_guest!(
        friends,
        WILD_BUDDY,
        sort_value: 2,
        emoji: '🤮',
        popularity: 2,
        trouble: true,
    );
    insert_guest!(
        randos,
        DRIVER,
        sort_value: 30,
        emoji: '🚗',
        cost: 3,
        ability_type: Summoning,
        ability_base: 1
    );
    insert_guest!(
        randos,
        MONKEY,
        sort_value: 31,
        emoji: '🐒',
        cost: 3,
        popularity: 4,
        trouble: true,
    );
    insert_guest!(
        randos,
        SECURITY,
        sort_value: 40,
        emoji: '👮',
        cost: 4,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        TICKET_TKR,
        sort_value: 41,
        emoji: '🎫',
        cost: 4,
        popularity: -1,
        cash: 2,
    );
    insert_guest!(
        randos,
        WATCH_DOG,
        sort_value: 42,
        emoji: '🦮',
        cost: 4,
        popularity: 2,
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        HIPPY,
        sort_value: 43,
        emoji: '✌',
        cost: 4,
        popularity: 1,
        chill: true,
    );
    insert_guest!(
        randos,
        ROCK_STAR,
        sort_value: 50,
        emoji: '🎸',
        cost: 5,
        popularity: 3,
        cash: 2,
        trouble: true,
    );
    insert_guest!(
        randos,
        COMEDIAN,
        sort_value: 51,
        emoji: '🤣',
        cost: 5,
        cash: -1,
        bonus_pop: |party| if party.attendees.len() as i8 == *party.capacity { 5 } else { 0 },
    );
    insert_guest!(
        randos,
        PRIVATE_I,
        sort_value: 44,
        emoji: '🕵',
        cost: 4,
        popularity: 2,
        cash: -1,
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        INTROVERT,
        sort_value: 45,
        emoji: '😶',
        cost: 4,
        popularity: 1,
        bonus_pop: |party| max(0, *party.capacity - party.attendees.len() as i8),
    );
    insert_guest!(
        randos,
        GRILLMASTR,
        sort_value: 52,
        emoji: '🍔',
        cost: 5,
        popularity: 2,
        ability_type: Evac,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        MR_POPULAR,
        sort_value: 53,
        emoji: '😎',
        cost: 5,
        popularity: 3,
        tagalongs: 1,
    );
    insert_guest!(
        randos,
        DANCER,
        sort_value: 70,
        emoji: '💃',
        cost: 7,
    );
    insert_guest!(
        randos,
        AUCTIONEER,
        sort_value: 90,
        emoji: '🤠',
        cost: 9,
        cash: 3,
    );
    insert_guest!(
        randos,
        MASCOT,
        sort_value: 54,
        emoji: '😸',
        cost: 5,
        popularity: 1,
        bonus_pop: |party| party.attendees.iter().filter(|guest| guest.guest == GuestType::OLD_FRIEND).count() as i8,
    );
    insert_guest!(
        randos,
        WRESTLER,
        sort_value: 91,
        emoji: '👊',
        cost: 9,
        popularity: 2,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        GANGSTER,
        sort_value: 61,
        emoji: '🔫',
        cost: 6,
        cash: 4,
        trouble: true,
    );
    insert_guest!(
        randos,
        CUTE_DOG,
        sort_value: 71,
        emoji: '🐶',
        cost: 7,
        popularity: 2,
        chill: true,
    );
    insert_guest!(
        randos,
        GAMBLER,
        sort_value: 72,
        emoji: '🎰',
        cost: 7,
        popularity: 2,
        cash: 3,
        trouble: true,
    );
    insert_guest!(
        randos,
        SPY,
        sort_value: 80,
        emoji: '🍸',
        cost: 8,
        cash: 2,
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WRITER,
        sort_value: 81,
        emoji: '🖋',
        cost: 8,
        popularity: 1,
        bonus_pop: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        PHOTOGRPHR,
        sort_value: 55,
        emoji: '📷',
        cost: 5,
        popularity: 1,
        cash: -1,
        ability_type: Shutter,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CHEERLEADR,
        sort_value: 56,
        emoji: '🎉',
        cost: 5,
        popularity: 1,
        ability_type: Cheer,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        COUNSELOR,
        sort_value: 73,
        emoji: '📋',
        cost: 7,
        ability_type: Quench,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        ATHLETE,
        sort_value: 62,
        emoji: '⚽',
        cost: 6,
        popularity: 1,
        cash: 1,
        ability_type: Evac,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CATERER,
        sort_value: 57,
        emoji: '🍣',
        cost: 5,
        popularity: 4,
        cash: -1,
    );
    insert_guest!(
        randos,
        BARTENDER,
        sort_value: 110,
        emoji: '🍺',
        cost: 11,
        popularity: 1,
        bonus_cash: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        CELEBRITY,
        sort_value: 111,
        emoji: '👸',
        cost: 11,
        popularity: 3,
        tagalongs: 2,
    );
    insert_guest!(
        randos,
        CUPID,
        sort_value: 82,
        emoji: '💘',
        cost: 8,
        popularity: 1,
        ability_type: LoveArrow,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        MAGICIAN,
        sort_value: 58,
        emoji: '🧙',
        cost: 5,
        popularity: 1,
        ability_type: StarSwap,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        GREETER,
        sort_value: 59,
        emoji: '🤵',
        cost: 5,
        popularity: 1,
        ability_type: Greet,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CLIMBER,
        sort_value: 120,
        emoji: '🤳',
        cost: 12,
    );
    insert_guest!(
        randos,
        STYLIST,
        sort_value: 74,
        emoji: '✂',
        cost: 7,
        cash: -1,
        ability_type: Style,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WAREWOLF,
        sort_value: 60,
        emoji: '🐺',
        cost: 5,
        popularity: 4,
        trouble: true,
    );
    insert_guest!(
        star_guests,
        ALIEN,
        sort_value: 200,
        emoji: '👽',
        cost: 40,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        MERMAID,
        sort_value: 201,
        emoji: '🧜',
        cost: 35,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        SUPERHERO,
        sort_value: 202,
        emoji: '🦸',
        cost: 50,
        popularity: 3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        DINOSAUR,
        sort_value: 203,
        emoji: '🦖',
        cost: 25,
        trouble: true,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        GENIE,
        sort_value: 204,
        emoji: '🧞',
        cost: 55,
        stars: 1,
        ability_type: Summoning,
        ability_base: 1,
    );
    insert_guest!(
        star_guests,
        DRAGON,
        sort_value: 205,
        emoji: '🐲',
        cost: 30,
        cash: -3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        LEPRECHAUN,
        sort_value: 206,
        emoji: '🍀',
        cost: 50,
        cash: 3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        UNICORN,
        sort_value: 207,
        emoji: '🦄',
        cost: 45,
        chill: true,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        GHOST,
        sort_value: 208,
        emoji: '👻',
        cost: 45,
        stars: 1,
        ability_type: Boot,
        ability_base: 1,
    );

    (friends, randos, star_guests)
}
