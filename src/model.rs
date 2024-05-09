use crate::drawing::{Cuboid, TextureOffsets};
use crate::nbt::Structure;
use crate::transform::{Transform, TransformStep};
use image::io::Reader as ImageReader;
use serde::Deserialize;
use std::collections::HashMap;

pub trait Model {
    fn texture(&self) -> &str;

    fn parts(&self) -> Vec<Cuboid>;

    fn draw(&self, structure: &mut Structure, scaling: u32) -> std::io::Result<()> {
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

#[derive(Deserialize)]
pub struct JsonModel {
    pub name: String,
    pub texture: String,
    pub parts: Vec<Part>,
    #[serde(default)]
    alternates: HashMap<String, String>,
}

impl JsonModel {
    pub fn use_alternate(&mut self, part_name: impl ToString, alt_name: impl ToString) {
        let part_name = part_name.to_string();
        let Some(part) = self.parts.iter().find(|p| p.name == part_name) else {
            return;
        };

        let alt_name = alt_name.to_string();
        if part.alternates.iter().any(|a| a.name == alt_name) {
            let entry = self.alternates.entry(part_name).or_default();
            *entry = alt_name;
        }
    }
}

impl Model for JsonModel {
    fn texture(&self) -> &str {
        &self.texture
    }

    fn parts(&self) -> Vec<Cuboid> {
        let mut cuboids = vec![];
        for part in self.parts.iter() {
            let position_steps = match self.alternates.get(&part.name) {
                Some(alt_name) => {
                    &part
                        .alternates
                        .iter()
                        .find(|a| &a.name == alt_name)
                        .expect("")
                        .position
                }
                None => &part.position,
            };

            let position = position_steps
                .iter()
                .fold(Transform::new(), |pos, step| pos.step(*step));

            cuboids.push(Cuboid {
                dimensions: part.dimensions.into(),
                offsets: TextureOffsets {
                    top: part.offsets.top.into(),
                    bottom: part.offsets.bottom.into(),
                    right: part.offsets.right.into(),
                    left: part.offsets.left.into(),
                    front: part.offsets.front.into(),
                    back: part.offsets.back.into(),
                },
                position,
            });
        }

        cuboids
    }
}

#[derive(Deserialize)]
pub struct Part {
    pub name: String,
    pub dimensions: [u32; 3],
    pub offsets: Offsets,
    pub position: Vec<TransformStep>,
    #[serde(default)]
    pub alternates: Vec<Alternate>,
}

#[derive(Deserialize)]
pub struct Offsets {
    top: [u32; 2],
    bottom: [u32; 2],
    right: [u32; 2],
    left: [u32; 2],
    front: [u32; 2],
    back: [u32; 2],
}

#[derive(Deserialize)]
pub struct Alternate {
    name: String,
    position: Vec<TransformStep>,
}
