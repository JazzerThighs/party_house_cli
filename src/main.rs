mod guests;
mod init;
mod inventory;
use init::*;
use inventory::*;

fn main() {
    let num_players = 1;
    let mut store = create_scenerio(num_players);
    let players: Vec<Inventory> = vec![Inventory::default().clone(); num_players];
}
