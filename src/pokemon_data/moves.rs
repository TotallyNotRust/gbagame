use core::iter::FromIterator;

use agb::hash_map::HashMap;

pub fn get_moves() -> HashMap<u16, Move> {
    return HashMap::from_iter([
        (
            1,
            Move {
                name: "Scratch",
                move_type: MoveType::Offensive,
                damage: Some(20),
                accuracy: 100,
                status_effect: None,
            },
        ),
        (
            2,
            Move {
                name: "Howl",
                move_type: MoveType::Status,
                damage: None,
                accuracy: 100,
                status_effect: Some(StatusEffect {
                    atk: Some(-1),
                    spd: None,
                    def: None,
                }),
            },
        ),
    ]);
}

#[derive(Clone, Copy)]
pub enum MoveType {
    Offensive,
    Status,
}

#[derive(Clone, Copy)]
pub struct Move {
    name: &'static str,
    move_type: MoveType,
    damage: Option<u8>,
    accuracy: u8,
    status_effect: Option<StatusEffect>,
}

#[derive(Clone, Copy)]
pub struct StatusEffect {
    atk: Option<i8>,
    spd: Option<i8>,
    def: Option<i8>,
}
