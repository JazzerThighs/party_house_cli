use crate::Party;
use better_default::Default;
use std::{cmp::max, collections::HashMap};

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
        emoji: '🙂',
        cost: 2,
        popularity: 1
    );
    insert_guest!(
        friends,
        RICH_PAL,
        emoji: '🤑',
        cost: 3,
        cash: 1,
    );
    insert_guest!(
        friends,
        WILD_BUDDY,
        emoji: '🤮',
        popularity: 2,
        trouble: true,
    );
    insert_guest!(
        randos,
        DRIVER,
        emoji: '🚗',
        cost: 3,
        ability_type: Summoning,
        ability_base: 1
    );
    insert_guest!(
        randos,
        MONKEY,
        emoji: '🐒',
        cost: 3,
        popularity: 4,
        trouble: true,
    );
    insert_guest!(
        randos,
        SECURITY,
        emoji: '👮',
        cost: 4,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        TICKET_TKR,
        emoji: '🎫',
        cost: 4,
        popularity: -1,
        cash: 2,
    );
    insert_guest!(
        randos,
        WATCH_DOG,
        emoji: '🦮',
        cost: 4,
        popularity: 2,
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        HIPPY,
        emoji: '✌',
        cost: 4,
        popularity: 1,
        chill: true,
    );
    insert_guest!(
        randos,
        ROCK_STAR,
        emoji: '🎸',
        cost: 5,
        popularity: 3,
        cash: 2,
        trouble: true,
    );
    insert_guest!(
        randos,
        COMEDIAN,
        emoji: '🤣',
        cost: 5,
        cash: -1,
        bonus_pop: |party| if party.attendees.len() as i8 == *party.capacity { 5 } else { 0 },
    );
    insert_guest!(
        randos,
        PRIVATE_I,
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
        emoji: '😶',
        cost: 4,
        popularity: 1,
        bonus_pop: |party| max(0, *party.capacity - party.attendees.len() as i8),
    );
    insert_guest!(
        randos,
        GRILLMASTR,
        emoji: '🍔',
        cost: 5,
        popularity: 2,
        ability_type: Evac,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        MR_POPULAR,
        emoji: '😎',
        cost: 5,
        popularity: 3,
        tagalongs: 1,
    );
    insert_guest!(
        randos,
        DANCER,
        emoji: '💃',
        cost: 7,
        bonus_pop: |party| max(16, party.attendees.iter().filter(|guest| guest.guest == GuestType::DANCER).count().pow(2) as i8),
    );
    insert_guest!(
        randos,
        AUCTIONEER,
        emoji: '🤠',
        cost: 9,
        cash: 3,
    );
    insert_guest!(
        randos,
        MASCOT,
        emoji: '😸',
        cost: 5,
        popularity: 1,
        bonus_pop: |party| party.attendees.iter().filter(|guest| guest.guest == GuestType::OLD_FRIEND).count() as i8,
    );
    insert_guest!(
        randos,
        WRESTLER,
        emoji: '👊',
        cost: 9,
        popularity: 2,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        GANGSTER,
        emoji: '🔫',
        cost: 6,
        cash: 4,
        trouble: true,
    );
    insert_guest!(
        randos,
        CUTE_DOG,
        emoji: '🐶',
        cost: 7,
        popularity: 2,
        chill: true,
    );
    insert_guest!(
        randos,
        GAMBLER,
        emoji: '🎰',
        cost: 7,
        popularity: 2,
        cash: 3,
        trouble: true,
    );
    insert_guest!(
        randos,
        SPY,
        emoji: '🍸',
        cost: 8,
        cash: 2,
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WRITER,
        emoji: '🖋',
        cost: 8,
        popularity: 1,
        bonus_pop: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        PHOTOGRPHR,
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
        emoji: '🎉',
        cost: 5,
        popularity: 1,
        ability_type: Cheer,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        COUNSELOR,
        emoji: '📋',
        cost: 7,
        ability_type: Quench,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        ATHLETE,
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
        emoji: '🍣',
        cost: 5,
        popularity: 4,
        cash: -1,
    );
    insert_guest!(
        randos,
        BARTENDER,
        emoji: '🍺',
        cost: 11,
        popularity: 1,
        bonus_cash: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        CELEBRITY,
        emoji: '👸',
        cost: 11,
        popularity: 3,
        tagalongs: 2,
    );
    insert_guest!(
        randos,
        CUPID,
        emoji: '💘',
        cost: 8,
        popularity: 1,
        ability_type: LoveArrow,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        MAGICIAN,
        emoji: '🧙',
        cost: 5,
        popularity: 1,
        ability_type: StarSwap,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        GREETER,
        emoji: '🤵',
        cost: 5,
        popularity: 1,
        ability_type: Greet,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CLIMBER,
        emoji: '🤳',
        cost: 12,
    );
    insert_guest!(
        randos,
        STYLIST,
        emoji: '✂',
        cost: 7,
        cash: -1,
        ability_type: Style,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WAREWOLF,
        emoji: '🐺',
        cost: 5,
        popularity: 4,
        trouble: true,
    );
    insert_guest!(
        star_guests,
        ALIEN,
        emoji: '👽',
        cost: 40,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        MERMAID,
        emoji: '🧜',
        cost: 35,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        SUPERHERO,
        emoji: '🦸',
        cost: 50,
        popularity: 3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        DINOSAUR,
        emoji: '🦖',
        cost: 25,
        trouble: true,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        GENIE,
        emoji: '🧞',
        cost: 55,
        stars: 1,
        ability_type: Summoning,
        ability_base: 1,
    );
    insert_guest!(
        star_guests,
        DRAGON,
        emoji: '🐲',
        cost: 30,
        cash: -3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        LEPRECHAUN,
        emoji: '🍀',
        cost: 50,
        cash: 3,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        UNICORN,
        emoji: '🦄',
        cost: 45,
        chill: true,
        stars: 1,
    );
    insert_guest!(
        star_guests,
        GHOST,
        emoji: '👻',
        cost: 45,
        stars: 1,
        ability_type: Boot,
        ability_base: 1,
    );

    (friends, randos, star_guests)
}
