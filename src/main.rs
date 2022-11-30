// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use core::panic;

mod pokemon_data;

use agb::{
    display,
    display::object::{self, Graphics, Object, Tag},
    include_aseprite,
    input::{self, Button, ButtonController},
    syscall,
};

const POKEMON_001: &Graphics = include_aseprite!("graphics/pokemon_no_001.aseprite");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let object = gba.display.object.get();
    // let mut pkmn_object = object.object_sprite(POKEMON_001.tags().get("battle").sprite(0));
    // let pkmn = MushroomMan::init(
    //     POKEMON_001,
    //     MushroomMan::MushroomManAnimation::Battle,
    //     Some(0),
    //     Some(0),
    // );
    let pokemon =
        <pokemon_data::MusroomMan::MushroomMan as pokemon_data::pokemon_struct::PokemonImpl>::new(
            5,
        );

    let pkmn_object = object.object_sprite(
        pokemon
            .pokemon
            .pokemon
            .graphics
            .tags()
            .get("battle")
            .sprite(0),
    );

    loop {
        // pkmn.move_by_vel(0, 0);

        object.commit();
    }
}
