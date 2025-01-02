use crate::guests::*;
use card_deck::Deck;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn create_scenerio(num_players: usize) -> Vec<Guest> {
    let (friends, randos, star_guests) = guest_lists();

    let num_stocks = usize::max(4, 4 + (2 * (num_players - 1)));

    let mut store = vec![friends[&GuestType::OLD_FRIEND].clone(); num_stocks];
    store.extend(vec![friends[&GuestType::OLD_FRIEND].clone(); num_stocks]);
    store.extend(vec![friends[&GuestType::RICH_PAL].clone(); num_stocks]);

    let flip: f64 = thread_rng().gen();
    let (randos_num, star_guests_num) = if flip < 0.5 { (11, 2) } else { (12, 1) };

    let mut randos_keys: Vec<GuestType> = randos.keys().cloned().collect();
    let mut rng = thread_rng();
    randos_keys.shuffle(&mut rng);
    let chosen_randos = &randos_keys[..randos_num];
    for guest in chosen_randos {
        store.extend(vec![randos[&guest].clone(); 4]);
    }
    let mut star_keys: Vec<GuestType> = star_guests.keys().cloned().collect();
    let mut rng = thread_rng();
    star_keys.shuffle(&mut rng);
    let chosen_stars = &star_keys[..star_guests_num];
    for guest in chosen_stars {
        store.extend(vec![star_guests[&guest].clone()]);
    }
    for i in 0..store.len() {
        store[i].id = i + 8;
    }

    store
}

pub fn init_rolodex() -> Deck<Guest> {
    let (friends, _, _) = guest_lists();
    let mut rolodex = vec![friends[&GuestType::OLD_FRIEND].clone(); 4];
    rolodex.extend(vec![friends[&GuestType::RICH_PAL].clone(); 2]);
    rolodex.extend(vec![friends[&GuestType::WILD_BUDDY].clone(); 4]);
    for i in 0..rolodex.len() {
        rolodex[i].id = i;
    }
    Deck::new(rolodex)
}
