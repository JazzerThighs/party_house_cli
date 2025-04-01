use crate::{clampedi8::ClampedI8, Party};
use nestify::nest;
use better_default::Default;
use std::{cmp::max, collections::HashMap};

nest!(
    #[derive(Default, Debug, Clone)]*
    pub struct Guest {
        pub sort_value: u8,
        #[default('🙂')]
        pub emoji: char,
        pub cost: u8,
        #[default(ClampedI8::pop_cash(0))]
        pub popularity: ClampedI8,
        #[default(ClampedI8::pop_cash(0))]
        pub cash: ClampedI8,
        #[default(ClampedI8::stars(0))]
        pub stars: ClampedI8,
        pub tagalongs: u8,
        #[default(|_| 0)] 
        pub bonus_pop: fn(&Party) -> i8,
        #[default(|_| 0)] 
        pub bonus_cash: fn(&Party) -> i8,
        #[default(|_| ())]
        pub entrance_effect: fn(&mut Self),
        
        pub trouble_base: bool,
        pub trouble: bool,
        
        pub chill_base: bool,
        pub chill: bool,
        
        pub ability_base: u8,
        pub ability_stock: u8,

        pub ability_type: 
            #[derive(PartialEq, Eq)]
            pub enum AbilityType {
                #[default]
                NoAbility,
                Evac,
                Quench,
                Cheer,
                Shutter,
                Style(usize),
                StarSwap,
                Boot,
                LoveArrow,
                Summoning,
                Peek,
                Greet,
            },
        pub full_house_ability:
            #[derive(PartialEq, Eq)]
            pub enum FullHouseAbilityCondition {
                Yes,
                IfYesIsPresent,
                #[default]
                No,
            },
        
        pub guest_type: 
            #[allow(non_camel_case_types)]
            #[derive(PartialEq, Eq, Hash)]
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
            },
    }
);

#[allow(non_snake_case)]
pub fn guest_lists() -> (
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
    HashMap<GuestType, Guest>,
) {
    let (mut friends, mut randos, mut star_guests, mut all_guests) =
        (HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new());

    use FullHouseAbilityCondition::*;
    use AbilityType::*;
    use GuestType::*;
    let mut sort_value = 0;
    macro_rules! insert_guest {
        ($map:expr, $guest:ident $(, $field:ident : $value:expr )* $(,)?) => {
            sort_value += 1;
            $map.insert(
                $guest,
                Guest {
                    guest_type: $guest,
                    sort_value,
                    $( $field: $value, )*
                    ..Default::default()
                }
            );
            all_guests.insert(
                $guest, 
                $map[&$guest].clone()
            );
        };
    }
    
    // Friends, in every starting rolodex
    insert_guest!(
        friends,
        OLD_FRIEND,
        emoji: '🙂',
        cost: 2,
        popularity: ClampedI8::pop_cash(1)
    );
    insert_guest!(
        friends,
        RICH_PAL,
        emoji: '🤑',
        cost: 3,
        cash: ClampedI8::pop_cash(1),
    );
    insert_guest!(
        friends,
        WILD_BUDDY,
        emoji: '🤮',
        popularity: ClampedI8::pop_cash(2),
        trouble_base: true,
    );
    
    // Randos
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
        popularity: ClampedI8::pop_cash(4),
        trouble_base: true,
    );
    insert_guest!(
        randos,
        SECURITY,
        emoji: '👮',
        cost: 4,
        full_house_ability: Yes,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        TICKET_TKR,
        emoji: '🎫',
        cost: 4,
        popularity: ClampedI8::pop_cash(-1),
        cash: ClampedI8::pop_cash(2),
    );
    insert_guest!(
        randos,
        WATCH_DOG,
        emoji: '🦮',
        cost: 4,
        popularity: ClampedI8::pop_cash(2),
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        HIPPY,
        emoji: '🌼',
        cost: 4,
        popularity: ClampedI8::pop_cash(2),
        chill_base: true,
    );
    insert_guest!(
        randos,
        PRIVATE_I,
        emoji: '🕵',
        cost: 4,
        popularity: ClampedI8::pop_cash(2),
        cash: ClampedI8::pop_cash(-1),
        ability_type: Summoning,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        INTROVERT,
        emoji: '😶',
        cost: 4,
        popularity: ClampedI8::pop_cash(1),
        bonus_pop: |party| max(0, *party.capacity - party.attendees.len() as i8),
    );
    insert_guest!(
        randos,
        ROCK_STAR,
        emoji: '🎸',
        cost: 5,
        popularity: ClampedI8::pop_cash(3),
        cash: ClampedI8::pop_cash(2),
        trouble_base: true,
    );
    insert_guest!(
        randos,
        COMEDIAN,
        emoji: '🤣',
        cost: 5,
        cash: ClampedI8::pop_cash(-1),
        bonus_pop: |party| if party.attendees.len() as i8 >= *party.capacity { 5 } else { 0 },
    );
    insert_guest!(
        randos,
        GRILLMASTR,
        emoji: '🍔',
        cost: 5,
        popularity: ClampedI8::pop_cash(2),
        full_house_ability: Yes,
        ability_type: Evac,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        MR_POPULAR,
        emoji: '😎',
        cost: 5,
        popularity: ClampedI8::pop_cash(3),
        tagalongs: 1,
    );
    insert_guest!(
        randos,
        MASCOT,
        emoji: '😸',
        cost: 5,
        popularity: ClampedI8::pop_cash(1),
        bonus_pop: |party| party.attendees.iter().filter(|guest| guest.guest_type == GuestType::OLD_FRIEND).count() as i8,
    );
    insert_guest!(
        randos,
        PHOTOGRPHR,
        emoji: '📷',
        cost: 5,
        popularity: ClampedI8::pop_cash(1),
        cash: ClampedI8::pop_cash(-1),
        full_house_ability: Yes,
        ability_type: Shutter,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CHEERLEADR,
        emoji: '🎉',
        cost: 5,
        popularity: ClampedI8::pop_cash(1),
        full_house_ability: IfYesIsPresent,
        ability_type: Cheer,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        CATERER,
        emoji: '🍣',
        cost: 5,
        popularity: ClampedI8::pop_cash(4),
        cash: ClampedI8::pop_cash(-1),
    );
    insert_guest!(
        randos,
        MAGICIAN,
        emoji: '🧙',
        cost: 5,
        popularity: ClampedI8::pop_cash(1),
        full_house_ability: Yes,
        ability_type: StarSwap,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        GREETER,
        emoji: '🤵',
        cost: 5,
        popularity: ClampedI8::pop_cash(1),
        ability_type: Greet,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WAREWOLF,
        emoji: '🐺',
        cost: 5,
        popularity: ClampedI8::pop_cash(4),
        entrance_effect: |g| {
            g.trouble_base = !g.trouble_base;
            g.trouble = g.trouble_base;
        }
    );
    insert_guest!(
        randos,
        GANGSTER,
        emoji: '🔫',
        cost: 6,
        cash: ClampedI8::pop_cash(4),
        trouble_base: true,
    );
    insert_guest!(
        randos,
        ATHLETE,
        emoji: '⚽',
        cost: 6,
        popularity: ClampedI8::pop_cash(1),
        cash: ClampedI8::pop_cash(1),
        full_house_ability: Yes,
        ability_type: Evac,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        DANCER,
        emoji: '💃',
        cost: 7,
    );
    insert_guest!(
        randos,
        CUTE_DOG,
        emoji: '🐶',
        cost: 7,
        popularity: ClampedI8::pop_cash(2),
        chill_base: true,
    );
    insert_guest!(
        randos,
        GAMBLER,
        emoji: '🎰',
        cost: 7,
        popularity: ClampedI8::pop_cash(2),
        cash: ClampedI8::pop_cash(3),
        trouble_base: true,
    );
    insert_guest!(
        randos,
        COUNSELOR,
        emoji: '📋',
        cost: 7,
        full_house_ability: Yes,
        ability_type: Quench,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        STYLIST,
        emoji: '✂',
        cost: 7,
        cash: ClampedI8::pop_cash(-1),
        full_house_ability: Yes,
        ability_type: Style(1),
        ability_base: 1,
    );
    insert_guest!(
        randos,
        SPY,
        emoji: '🍸',
        cost: 8,
        cash: ClampedI8::pop_cash(2),
        ability_type: Peek,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        WRITER,
        emoji: '🖋',
        cost: 8,
        popularity: ClampedI8::pop_cash(1),
        bonus_pop: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        CUPID,
        emoji: '👼',
        cost: 8,
        popularity: ClampedI8::pop_cash(1),
        full_house_ability: Yes,
        ability_type: LoveArrow,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        AUCTIONEER,
        emoji: '🤠',
        cost: 9,
        cash: ClampedI8::pop_cash(3),
    );
    insert_guest!(
        randos,
        WRESTLER,
        emoji: '👊',
        cost: 9,
        popularity: ClampedI8::pop_cash(2),
        full_house_ability: Yes,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        randos,
        BARTENDER,
        emoji: '🍺',
        cost: 11,
        popularity: ClampedI8::pop_cash(1),
        bonus_cash: |party| 2 * party.attendees.iter().filter(|guest| guest.trouble).count() as i8,
    );
    insert_guest!(
        randos,
        CELEBRITY,
        emoji: '👸',
        cost: 11,
        popularity: ClampedI8::pop_cash(2),
        cash: ClampedI8::pop_cash(3),
        tagalongs: 2,
    );
    insert_guest!(
        randos,
        CLIMBER,
        emoji: '🤳',
        cost: 12,
        entrance_effect: |g| g.popularity += 1
    );
    
    // Star Guests, win condition fulfillers
    insert_guest!(
        star_guests,
        DINOSAUR,
        emoji: '🦖',
        cost: 25,
        trouble_base: true,
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        DRAGON,
        emoji: '🐲',
        cost: 30,
        cash: ClampedI8::pop_cash(-3),
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        MERMAID,
        emoji: '🧜',
        cost: 35,
        tagalongs: 1,
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        ALIEN,
        emoji: '👽',
        cost: 40,
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        UNICORN,
        emoji: '🦄',
        cost: 45,
        chill_base: true,
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        GHOST,
        emoji: '👻',
        cost: 45,
        stars: ClampedI8::stars(1),
        full_house_ability: Yes,
        ability_type: Boot,
        ability_base: 1,
    );
    insert_guest!(
        star_guests,
        SUPERHERO,
        emoji: '🦸',
        cost: 50,
        popularity: ClampedI8::pop_cash(3),
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        LEPRECHAUN,
        emoji: '🍀',
        cost: 50,
        cash: ClampedI8::pop_cash(3),
        stars: ClampedI8::stars(1),
    );
    insert_guest!(
        star_guests,
        GENIE,
        emoji: '🧞',
        cost: 55,
        stars: ClampedI8::stars(1),
        ability_type: Summoning,
        ability_base: 1,
    );

    (friends, randos, star_guests, all_guests)
}
