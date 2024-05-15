use crate::nbt::Structure;
use image::RgbaImage;

pub trait Draw {
    fn draw(&self, structure: &mut Structure, scaling: u32, image: &RgbaImage);
}
