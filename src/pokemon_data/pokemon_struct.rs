use core::panic;

use super::moves::Move;
use agb::{display::object::Graphics, hash_map::HashMap};

pub struct BasePokemon {
    pub graphics: &'static Graphics,
    pub learned_moves: HashMap<u8, Move>,
}

pub struct Pokemon {
    pub pokemon: BasePokemon,
    pub moves: PokemonMoves,
}

pub struct PokemonMoves(Option<Move>, Option<Move>, Option<Move>, Option<Move>);

impl PokemonMoves {
    fn set(mut self, i: u8, _move: Move) -> Self {
        match i {
            0 => self.0 = Some(_move),
            1 => self.1 = Some(_move),
            2 => self.2 = Some(_move),
            3 => self.3 = Some(_move),
            _ => (),
        };
        return self;
    }
}

pub trait PokemonImpl {
    fn new(level: u8) -> Self;

    fn get_moves_from_level(level: u8, learned_moves: &HashMap<u8, Move>) -> PokemonMoves {
        let mut possible = learned_moves.keys().filter(|key| key <= &&level);

        let mut moves = PokemonMoves(None, None, None, None);

        let mut moves_found = 0;
        loop {
            let x = possible.next();

            match x {
                None => break,
                Some(n) => match learned_moves.get(n).clone() {
                    None => continue,
                    Some(_move) => {
                        moves = moves.set(moves_found, _move.clone());
                        moves_found += 1;
                    }
                },
            }
        }

        return moves;
    }
}
