use crate::drawing::{Cuboid, Dimensions, TextureOffsets};
use crate::model::Model;
use crate::transform::{Plane, Rotation, Transform};

pub struct Skin {
    pub texture: String,
    head: Cuboid,
    hat: Cuboid,
    body: Cuboid,
    right_arm: Cuboid,
    left_arm: Cuboid,
    right_leg: Cuboid,
    left_leg: Cuboid,
}

impl Skin {
    pub fn player(name: &str) -> Self {
        Skin {
            texture: format!("skins/{name}.png"),
            head: Cuboid {
                dimensions: Dimensions { x: 8, y: 8, z: 8 },
                offsets: TextureOffsets {
                    top: (8, 0),
                    bottom: (16, 0),
                    right: (0, 8),
                    left: (16, 8),
                    front: (8, 8),
                    back: (24, 8),
                },
                position: Transform::new(),
            },
            hat: Cuboid {
                dimensions: Dimensions { x: 8, y: 8, z: 8 },
                offsets: TextureOffsets {
                    top: (40, 0),
                    bottom: (48, 0),
                    right: (32, 8),
                    left: (48, 8),
                    front: (40, 8),
                    back: (56, 8),
                },
                position: Transform::new().translate(0, 10, 0),
            },
            body: Cuboid {
                dimensions: Dimensions { x: 8, y: 12, z: 4 },
                offsets: TextureOffsets {
                    top: (20, 16),
                    bottom: (28, 16),
                    right: (16, 20),
                    left: (28, 20),
                    front: (20, 20),
                    back: (32, 20),
                },
                position: Transform::new().translate(0, -8, -2),
            },
            right_arm: Cuboid {
                dimensions: Dimensions { x: 4, y: 12, z: 4 },
                offsets: TextureOffsets {
                    top: (44, 16),
                    bottom: (48, 16),
                    right: (40, 20),
                    left: (48, 20),
                    front: (44, 20),
                    back: (52, 20),
                },
                position: Transform::new().translate(-4, -8, -2),
            },
            left_arm: Cuboid {
                dimensions: Dimensions { x: 4, y: 12, z: 4 },
                offsets: TextureOffsets {
                    top: (44, 16),
                    bottom: (48, 16),
                    right: (40, 20),
                    left: (48, 20),
                    front: (44, 20),
                    back: (52, 20),
                },
                position: Transform::new().mirror(Plane::YZ).translate(13, -8, -2),
            },
            right_leg: Cuboid {
                dimensions: Dimensions { x: 4, y: 12, z: 4 },
                offsets: TextureOffsets {
                    top: (4, 16),
                    bottom: (8, 16),
                    right: (0, 20),
                    left: (8, 20),
                    front: (4, 20),
                    back: (12, 20),
                },
                position: Transform::new().translate(0, -20, -2),
            },
            left_leg: Cuboid {
                dimensions: Dimensions { x: 4, y: 12, z: 4 },
                offsets: TextureOffsets {
                    top: (4, 16),
                    bottom: (8, 16),
                    right: (0, 20),
                    left: (8, 20),
                    front: (4, 20),
                    back: (12, 20),
                },
                position: Transform::new().mirror(Plane::YZ).translate(9, -20, -2),
            },
        }
    }

    pub fn right_arm_forward(mut self) -> Self {
        self.right_arm.position = Transform::new()
            .rotate(Rotation::XNeg)
            .translate(-4, -8, -5);
        self
    }
}

impl Model for Skin {
    fn texture(&self) -> &str {
        self.texture.as_str()
    }

    fn parts(&self) -> Vec<&Cuboid> {
        vec![
            &self.head,
            &self.hat,
            &self.body,
            &self.right_arm,
            &self.left_arm,
            &self.right_leg,
            &self.left_leg,
        ]
    }
}
