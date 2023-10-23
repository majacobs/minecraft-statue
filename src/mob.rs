use crate::drawing::{Cuboid, Dimensions, TextureOffsets};
use crate::model::Model;
use crate::transform::{Rotation, Transform};

const TEXTURE_DIR: &str = "minecraft/1.20.1/assets/minecraft/textures/";

pub fn get_mob(name: &str) -> Option<Box<dyn Model>> {
    match name {
        "goat" => Some(Box::new(Goat::new())),
        _ => None,
    }
}

pub struct Goat {
    texture: String,
    head: Cuboid,
    torso: Cuboid,
    coat: Cuboid,
    front_left_leg: Cuboid,
    front_right_leg: Cuboid,
    rear_left_leg: Cuboid,
    rear_right_leg: Cuboid,
}

impl Goat {
    fn new() -> Self {
        Goat {
            texture: format!("{TEXTURE_DIR}entity/goat/goat.png"),
            head: Cuboid {
                dimensions: Dimensions { x: 5, y: 7, z: 10 },
                offsets: TextureOffsets {
                    top: (44, 46),
                    bottom: (49, 46),
                    right: (34, 56),
                    left: (49, 56),
                    front: (44, 56),
                    back: (59, 56),
                },
                position: Transform::new().rotate(Rotation::XPos).translate(2, -10, 7),
            },
            torso: Cuboid {
                dimensions: Dimensions { x: 9, y: 11, z: 16 },
                offsets: TextureOffsets {
                    top: (17, 1),
                    bottom: (26, 1),
                    right: (1, 17),
                    left: (26, 17),
                    front: (17, 17),
                    back: (42, 17),
                },
                position: Transform::new(),
            },
            coat: Cuboid {
                dimensions: Dimensions { x: 11, y: 14, z: 11 },
                offsets: TextureOffsets {
                    top: (11, 28),
                    bottom: (22, 28),
                    right: (0, 39),
                    left: (22, 39),
                    front: (11, 39),
                    back: (33, 39),
                },
                position: Transform::new().translate(-1, 1, 1),
            },
            front_left_leg: Cuboid {
                dimensions: Dimensions { x: 3, y: 10, z: 3 },
                offsets: TextureOffsets {
                    top: (38, 2),
                    bottom: (41, 2),
                    right: (35, 5),
                    left: (41, 5),
                    front: (38, 5),
                    back: (44, 5),
                },
                position: Transform::new().translate(5, -7, -1),
            },
            front_right_leg: Cuboid {
                dimensions: Dimensions { x: 3, y: 10, z: 3 },
                offsets: TextureOffsets {
                    top: (52, 2),
                    bottom: (55, 2),
                    right: (49, 5),
                    left: (55, 5),
                    front: (52, 5),
                    back: (59, 5),
                },
                position: Transform::new().translate(1, -7, -1),
            },
            rear_left_leg: Cuboid {
                dimensions: Dimensions { x: 3, y: 6, z: 3 },
                offsets: TextureOffsets {
                    top: (39, 29),
                    bottom: (42, 29),
                    right: (36, 32),
                    left: (42, 32),
                    front: (39, 32),
                    back: (45, 32),
                },
                position: Transform::new().translate(5, -11, -11),
            },
            rear_right_leg: Cuboid {
                dimensions: Dimensions { x: 3, y: 6, z: 3 },
                offsets: TextureOffsets {
                    top: (52, 29),
                    bottom: (55, 29),
                    right: (49, 32),
                    left: (55, 32),
                    front: (52, 32),
                    back: (59, 32),
                },
                position: Transform::new().translate(1, -11, -11),
            },
        }
    }
}

impl Model for Goat {
    fn texture(&self) -> &str {
        self.texture.as_str()
    }

    fn parts(&self) -> Vec<&Cuboid> {
        vec![
            &self.head,
            &self.torso,
            &self.coat,
            &self.front_left_leg,
            &self.front_right_leg,
            &self.rear_left_leg,
            &self.rear_right_leg,
        ]
    }
}
