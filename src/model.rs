use crate::draw::Draw;
use crate::drawing::{Cuboid, Plane, Primitive, TextureOffsets};
use crate::nbt::Structure;
use crate::transform::{Transform, TransformStep};
use image::RgbaImage;
use serde::Deserialize;
use std::collections::HashMap;

pub trait Model {
    fn parts(&self) -> Vec<Primitive>;
}

impl<T: Model> Draw for T {
    fn draw(&self, structure: &mut Structure, scaling: u32, image: &RgbaImage) {
        let transform = Transform::with_scale(scaling);
        for part in self.parts().into_iter() {
            match part {
                Primitive::Cuboid(cuboid) => {
                    for (face, brush) in cuboid.faces().into_iter() {
                        face.draw(structure, image, &transform, &cuboid.position, brush);
                    }
                }
                Primitive::Plane(plane) => {
                    let (face, brush) = plane.face();
                    face.draw(structure, image, &transform, &plane.position, brush);
                }
            }
        }
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
        let Some(alternates) = self.parts.iter().find_map(|p| match p {
            Part::Cuboid(p) if p.name == part_name => Some(&p.alternates),
            Part::Flat(p) if p.name == part_name => Some(&p.alternates),
            _ => None,
        }) else {
            return;
        };

        let alt_name = alt_name.to_string();
        if alternates.iter().any(|a| a.name == alt_name) {
            let entry = self.alternates.entry(part_name).or_default();
            *entry = alt_name;
        }
    }
}

impl Model for JsonModel {
    fn parts(&self) -> Vec<Primitive> {
        let mut drawables = vec![];
        for part in self.parts.iter() {
            let primitive = match part {
                Part::Cuboid(cuboid) => cuboid.parts(&self.alternates),
                Part::Flat(flat) => flat.parts(&self.alternates),
            };

            if let Some(primitive) = primitive {
                drawables.push(primitive);
            } else {
                println!("Skipping invalid alternate for {}.", part.name());
            }
        }

        drawables
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Part {
    Cuboid(CuboidPart),
    Flat(FlatPart),
}

impl Part {
    fn name(&self) -> &str {
        match self {
            Part::Cuboid(cuboid) => &cuboid.name,
            Part::Flat(flat) => &flat.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CuboidPart {
    pub name: String,
    pub dimensions: [u32; 3],
    pub offsets: Offsets,
    pub position: Vec<TransformStep>,
    #[serde(default)]
    pub alternates: Vec<Alternate>,
}

impl CuboidPart {
    fn parts(&self, applied_alternates: &HashMap<String, String>) -> Option<Primitive> {
        let position = self.get_position(applied_alternates)?;

        Some(Primitive::Cuboid(Cuboid {
            dimensions: self.dimensions.into(),
            offsets: TextureOffsets {
                top: self.offsets.top.into(),
                bottom: self.offsets.bottom.into(),
                right: self.offsets.right.into(),
                left: self.offsets.left.into(),
                front: self.offsets.front.into(),
                back: self.offsets.back.into(),
            },
            position,
        }))
    }

    fn get_position(&self, applied_alternates: &HashMap<String, String>) -> Option<Transform> {
        let position_steps = match applied_alternates.get(&self.name) {
            Some(alt_name) => {
                &self
                    .alternates
                    .iter()
                    .find(|a| &a.name == alt_name)?
                    .position
            }
            None => &self.position,
        };

        let position = position_steps
            .iter()
            .fold(Transform::new(), |pos, step| pos.step(*step));

        Some(position)
    }
}

#[derive(Deserialize)]
pub struct FlatPart {
    pub name: String,
    pub dimensions: [u32; 2],
    pub offset: [u32; 2],
    pub position: Vec<TransformStep>,
    #[serde(default)]
    pub alternates: Vec<Alternate>,
}

impl FlatPart {
    fn parts(&self, applied_alternates: &HashMap<String, String>) -> Option<Primitive> {
        let position = self.get_position(applied_alternates)?;

        Some(Primitive::Plane(Plane {
            dimensions: self.dimensions.into(),
            offset: self.offset.into(),
            position,
        }))
    }

    fn get_position(&self, applied_alternates: &HashMap<String, String>) -> Option<Transform> {
        let position_steps = match applied_alternates.get(&self.name) {
            Some(alt_name) => {
                &self
                    .alternates
                    .iter()
                    .find(|a| &a.name == alt_name)?
                    .position
            }
            None => &self.position,
        };

        let position = position_steps
            .iter()
            .fold(Transform::new(), |pos, step| pos.step(*step));

        Some(position)
    }
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
