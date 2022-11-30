use core::iter::FromIterator;

use agb::{display::object::Graphics, hash_map::HashMap, include_aseprite};

use super::{
    moves,
    pokemon_struct::{self, BasePokemon, Pokemon, PokemonMoves},
};

const SPRITE: &'static Graphics = include_aseprite!("graphics/pokemon_no_001.aseprite");

pub struct MushroomMan {
    pub pokemon: Pokemon,
}

impl pokemon_struct::PokemonImpl for MushroomMan {
    fn new(level: u8) -> MushroomMan {
        let allMoves = moves::get_moves();
        let pokemon = BasePokemon {
            graphics: SPRITE,
            learned_moves: HashMap::from_iter([(
                0,
                allMoves.get(&0).expect("No such move").clone(),
            )]),
        };
        let moves: PokemonMoves = Self::get_moves_from_level(level, &pokemon.learned_moves);

        return MushroomMan {
            pokemon: Pokemon {
                pokemon: pokemon,
                moves: moves,
            },
        };
    }
}
