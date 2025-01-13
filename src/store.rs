use crate::guest::*;

#[derive(Debug, Clone)]
pub struct Store {
    pub stock: Vec<(Guest, f32)>,
    pub done_shopping: bool,
}
impl Store {
        const fn cost_of_expansion(capacity: i8) -> i8 {
        match capacity {
            5..=15 => capacity - 3,
            16..=34 => 12,
            35.. => 0,
            ..=4 => unreachable!()
        }
    }
}
