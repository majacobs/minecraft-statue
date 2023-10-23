use crate::drawing::{Brush, Face};
use crate::nbt::Structure;
use crate::transform::{Plane, Rotation, Transform};
use image::io::Reader as ImageReader;

const TEXTURE_DIR: &str = "minecraft/1.20.1/assets/minecraft/textures/";

pub struct Item {
    pub texture: String,
    pub face: Face,
    pub position: Transform,
}

impl Item {
    pub fn draw(&self, structure: &mut Structure, scaling: i32) -> std::io::Result<()> {
        let image = ImageReader::open(&self.texture)?.decode().unwrap();
        let image = image.as_rgba8().unwrap();

        let pre_transform = Transform::with_scale(scaling);
        self.face.draw(
            structure,
            image,
            &pre_transform,
            &self.position,
            Brush::Full,
        );

        Ok(())
    }
}

pub fn get_item(name: &str) -> Option<Item> {
    match name {
        "diamond_axe" | "diamond_hoe" | "diamond_pickaxe" | "diamond_shovel" | "diamond_sword"
        | "golden_axe" | "golden_hoe" | "golden_pickaxe" | "golden_shovel" | "golden_sword"
        | "iron_axe" | "iron_hoe" | "iron_pickaxe" | "iron_shovel" | "iron_sword"
        | "netherite_axe" | "netherite_hoe" | "netherite_pickaxe" | "netherite_shovel"
        | "netherite_sword" | "stone_axe" | "stone_hoe" | "stone_pickaxe" | "stone_shovel"
        | "stone_sword" | "wooden_axe" | "wooden_hoe" | "wooden_pickaxe" | "wooden_shovel"
        | "wooden_sword" => Some(tool(name)),
        "trident" => Some(trident()),
        _ => None,
    }
}

fn tool(name: &str) -> Item {
    Item {
        texture: format!("{TEXTURE_DIR}item/{name}.png"),
        face: Face {
            x: 0,
            y: 0,
            width: 16,
            height: 16,
            transform: Transform::new(),
        },
        position: Transform::new().rotate(Rotation::ZPos).mirror(Plane::XY),
    }
}

fn trident() -> Item {
    Item {
        texture: TEXTURE_DIR.to_string() + "entity/trident.png",
        face: Face {
            x: 19,
            y: 1,
            width: 5,
            height: 31,
            transform: Transform::new(),
        },
        position: Transform::new().rotate(Rotation::XPos),
    }
}
