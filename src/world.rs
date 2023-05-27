//Color Declaration
type Colour = [f32; 4];
const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

//Window Declaration
pub const WINDOW_SIZE: i32 = 512;
pub const PIXEL_SIZE: f64 = 32.0;
pub const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

//Tile API
#[derive(Clone)]
pub struct Tile {
    pub colour: Colour,
}

//Tile Parse
impl Tile {
    pub fn empty() -> Self {
        Tile { colour: BLACK }
    }

    pub fn wall() -> Self {
        Tile { colour: WHITE }
    }
}

//Create the map
pub fn consctruct_map() -> Vec<Vec<Tile>> {
    let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
}
