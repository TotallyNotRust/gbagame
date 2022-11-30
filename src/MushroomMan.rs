// pub enum MushroomManAnimation {
//     Battle = 0,
// }

// const ANIMATIONS = [
//     "Battle",
// ];

// pub struct MushroomMan<'a> {
//     pub sprite: &'a Graphics,
//     pub object: &'a mut Object<'a>,
//     pub x: u16,
//     pub y: u16,
// }

// pub impl MushroomMan<'_> {
//     pub fn init<'a>(
//         sprite: &'a Graphics,
//         animation: MushroomManAnimation,
//         start_x: Option<u16>,
//         start_y: Option<u16>,
//     ) -> &'a mut Pokemon<'a> {
//         // Create pokemon obect
//         let mut pkmn: Pokemon = Pokemon {
//             sprite: sprite,
//             obe
//             x: 0,
//             y: 0,
//         };

//         // Move to starting position, this may need changing in the future
//         match (start_x, start_y) {
//             (Some(x), None) => {
//                 pkmn.x = x;
//                 pkmn.object.set_x(x).show();
//             }
//             (None, Some(y)) => {
//                 pkmn.y = y;
//                 pkmn.object.set_x(y).show();
//             }
//             (Some(x), Some(y)) => {
//                 pkmn.x = x;
//                 pkmn.y = y;
//                 pkmn.object.set_x(x).set_y(y).show();
//             }
//             _ => {
//                 pkmn.object.set_x(0).set_y(0).show();
//             }
//         };

//         // Return new pokemon object
//         let x: &'a mut Pokemon = &mut pkmn;
//         x
//     }
//     pub fn move_by_vel<'a>(&mut self, x: u16, y: u16) {
//         let temp_x = x + self.x;
//         let temp_y = y + self.y;

//         self.x = temp_x.clamp(0, agb::display::WIDTH as u16);
//         self.y = temp_y.clamp(0, agb::display::HEIGHT as u16);

//         self.object.set_x(self.x).set_y(self.y);
//     }

//     pub fn move_to_point<'a>(&mut self, x: u16, y: u16) {
//         self.x = x;
//         self.y = y;

//         self.object.set_x(self.x).set_y(self.y);
//     }
// }
