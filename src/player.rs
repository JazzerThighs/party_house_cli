use crate::{guests::*, clampedi8::*};
use card_deck::Deck;

#[derive(Debug, Clone)]
pub struct Player {
    pub rolodex: Deck<crate::guests::Guest>,
    pub popularity: ClampedI8,
    pub cash: ClampedI8,
    pub capacity: ClampedI8,
    pub victory: bool,
    pub id: usize,
}

pub fn init_players(num_players: usize) -> Vec<Player> {
    let mut players = vec![];
    let rolodex = {
        let (friends, _, _) = guest_lists();
        let mut rolodex = vec![friends[&GuestType::OLD_FRIEND].clone(); 4];
        rolodex.extend(vec![friends[&GuestType::RICH_PAL].clone(); 2]);
        rolodex.extend(vec![friends[&GuestType::WILD_BUDDY].clone(); 4]);
        for i in 0..rolodex.len() {
            rolodex[i].id = i;
        }
        Deck::new(rolodex)
    };
    for i in 0..num_players {
        players.push(Player {
            rolodex: rolodex.clone(),
            popularity: ClampedI8 {
                value: 0,
                min: 0,
                max: 65,
            },
            cash: ClampedI8 {
                value: 0,
                min: 0,
                max: 30,
            },
            capacity: ClampedI8 {
                value: 5,
                min: 5,
                max: 34,
            },
            victory: false,
            id: i + 1
        })
    };
    players
}
