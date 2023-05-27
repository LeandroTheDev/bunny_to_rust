use crate::world;

#[derive(Clone)]
//Object Declaration
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub character: char,
    pub colour: world::Colour,
}
//Object Functions
impl Object {
    //Object Creation
    pub fn new(x: i32, y: i32, character: char, colour: world::Colour) -> Self {
        Object {
            x,
            y,
            character,
            colour,
        }
    }
}
