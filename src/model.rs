use crate::drawing::Cuboid;
use crate::nbt::Structure;
use crate::transform::Transform;
use image::io::Reader as ImageReader;

pub trait Model {
    fn texture(&self) -> &str;

    fn parts(&self) -> Vec<&Cuboid>;

    fn draw(&self, structure: &mut Structure, scaling: i32) -> std::io::Result<()> {
        let image = ImageReader::open(self.texture())?.decode().unwrap();
        let image = image.as_rgba8().unwrap();

        let transform = Transform::with_scale(scaling);
        for part in self.parts().into_iter() {
            for (face, brush) in part.faces().into_iter() {
                face.draw(structure, image, &transform, &part.position, brush);
            }
        }

        Ok(())
    }
}
