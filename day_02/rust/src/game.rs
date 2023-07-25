pub mod game_variant_1;
pub mod result;
pub mod shape;

pub trait Game {
    fn get_points(&self) -> u32;
}
