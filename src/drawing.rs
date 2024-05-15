use crate::materials::{find_closest, Direction};
use crate::nbt::Structure;
use crate::transform::{Rotation, Transform};
use image::{GenericImageView, RgbaImage};

#[derive(Clone)]
pub enum Primitive {
    Cuboid(Cuboid),
    Plane(Plane),
}

#[derive(Clone)]
pub struct Cuboid {
    pub dimensions: Dimensions3D,
    pub offsets: TextureOffsets,
    pub position: Transform,
}

impl Cuboid {
    pub fn faces(&self) -> Vec<(Face, Brush)> {
        vec![
            // Top
            (
                Face {
                    x: self.offsets.top.0,
                    y: self.offsets.top.1,
                    width: self.dimensions.x,
                    height: self.dimensions.z,
                    transform: Transform::new().translate(1, 0, 1 - (self.dimensions.z as i32)),
                },
                Brush::YPos,
            ),
            // Bottom
            (
                Face {
                    x: self.offsets.bottom.0,
                    y: self.offsets.bottom.1,
                    width: self.dimensions.x,
                    height: self.dimensions.z,
                    transform: Transform::new().translate(
                        1,
                        1 - (self.dimensions.y as i32),
                        1 - (self.dimensions.z as i32),
                    ),
                },
                Brush::YNeg,
            ),
            // Back
            (
                Face {
                    x: self.offsets.back.0,
                    y: self.offsets.back.1,
                    width: self.dimensions.x,
                    height: self.dimensions.y,
                    transform: Transform::new()
                        .rotate(Rotation::XPos)
                        .rotate(Rotation::YPos)
                        .rotate(Rotation::YPos)
                        .translate(self.dimensions.x as i32, 0, 1 - (self.dimensions.z as i32)),
                },
                Brush::YPos,
            ),
            // Right
            (
                Face {
                    x: self.offsets.right.0,
                    y: self.offsets.right.1,
                    width: self.dimensions.z,
                    height: self.dimensions.y,
                    transform: Transform::new()
                        .rotate(Rotation::XPos)
                        .rotate(Rotation::YNeg)
                        .translate(1, 0, 1 - (self.dimensions.z as i32)),
                },
                Brush::YPos,
            ),
            // Left
            (
                Face {
                    x: self.offsets.left.0,
                    y: self.offsets.left.1,
                    width: self.dimensions.z,
                    height: self.dimensions.y,
                    transform: Transform::new()
                        .rotate(Rotation::XPos)
                        .rotate(Rotation::YPos)
                        .translate(self.dimensions.x as i32, 0, 0),
                },
                Brush::YPos,
            ),
            // Front
            (
                Face {
                    x: self.offsets.front.0,
                    y: self.offsets.front.1,
                    width: self.dimensions.x,
                    height: self.dimensions.y,
                    transform: Transform::new().rotate(Rotation::XPos).translate(1, 0, 0),
                },
                Brush::YPos,
            ),
        ]
    }
}

#[derive(Clone)]
pub struct Plane {
    pub dimensions: Dimensions2D,
    pub offset: (u32, u32),
    pub position: Transform,
}

impl Plane {
    pub fn face(&self) -> (Face, Brush) {
        (
            Face {
                x: self.offset.0,
                y: self.offset.1,
                width: self.dimensions.x,
                height: self.dimensions.y,
                transform: Transform::new(),
            },
            Brush::Full,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Dimensions3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl From<[u32; 3]> for Dimensions3D {
    fn from(value: [u32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dimensions2D {
    pub x: u32,
    pub y: u32,
}

impl From<[u32; 2]> for Dimensions2D {
    fn from(value: [u32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextureOffsets {
    pub top: (u32, u32),
    pub bottom: (u32, u32),
    pub right: (u32, u32),
    pub left: (u32, u32),
    pub front: (u32, u32),
    pub back: (u32, u32),
}

pub struct Face {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub transform: Transform,
}

impl Face {
    pub fn draw(
        &self,
        structure: &mut Structure,
        image: &RgbaImage,
        pre_transform: &Transform,
        post_transform: &Transform,
        brush: Brush,
    ) {
        let transform = pre_transform
            .clone()
            .then(&self.transform)
            .then(post_transform);
        let brush = transform.brush(brush);

        let net_rotation = transform.rotate_only(0, 1, 0);
        let normal = Direction::from_unit(net_rotation).expect("invalid rotation");

        let view = image.view(self.x, self.y, self.width, self.height);
        for (x, z, pixel) in view.pixels() {
            if pixel.0[3] < 128 {
                continue;
            }

            let palette = find_closest(&pixel.0, normal);

            let coords = transform.apply(x as i32, 0, z as i32);
            for (dx, dy, dz) in brush.make_offsets(transform.scaling).into_iter() {
                let offset_coords = (coords.0 + dx, coords.1 + dy, coords.2 + dz);
                structure.set(offset_coords, palette.clone());
            }
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Brush {
    Full,
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

impl Brush {
    pub fn make_offsets(&self, scaling: i32) -> Vec<(i32, i32, i32)> {
        let xs: Vec<i32> = match self {
            Brush::XPos => vec![scaling - 1],
            Brush::XNeg => vec![0],
            _ => (0..scaling).collect(),
        };

        let ys: Vec<i32> = match self {
            Brush::YPos => vec![scaling - 1],
            Brush::YNeg => vec![0],
            _ => (0..scaling).collect(),
        };

        let zs: Vec<i32> = match self {
            Brush::ZPos => vec![scaling - 1],
            Brush::ZNeg => vec![0],
            _ => (0..scaling).collect(),
        };

        let mut offsets = vec![];
        for &x in xs.iter() {
            for &y in ys.iter() {
                for &z in zs.iter() {
                    offsets.push((x, y, z));
                }
            }
        }

        offsets
    }
}
