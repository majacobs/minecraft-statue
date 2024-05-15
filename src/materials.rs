use crate::nbt::Palette;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs::File;

static ALL_BLOCKS: Lazy<Vec<Block>> = Lazy::new(read_blocks);

fn read_blocks() -> Vec<Block> {
    let Ok(file) = File::open("materials.json") else {
        return vec![];
    };
    serde_json::from_reader(file).unwrap_or_default()
}

#[derive(Deserialize)]
struct Block {
    block_id: String,
    texturing: Texturing,
    part: Option<String>,
    avg_color: [u8; 3],
}

impl Block {
    fn try_orient(&self, texturing: Texturing, normal: Direction) -> Option<Orient> {
        match texturing {
            Texturing::Uniform => Some(Orient::AsIs),
            Texturing::Axis => match (self.part.as_deref()?, normal) {
                ("top", Direction::PosX) => Some(Orient::with("axis", "x")),
                ("top", Direction::PosY) => Some(Orient::with("axis", "y")),
                ("top", Direction::PosZ) => Some(Orient::with("axis", "z")),
                ("top", Direction::NegX) => Some(Orient::with("axis", "x")),
                ("top", Direction::NegY) => Some(Orient::with("axis", "y")),
                ("top", Direction::NegZ) => Some(Orient::with("axis", "z")),
                ("" | "side", Direction::PosX) => Some(Orient::with("axis", "z")),
                ("" | "side", Direction::PosY) => Some(Orient::with("axis", "x")),
                ("" | "side", Direction::PosZ) => Some(Orient::with("axis", "x")),
                ("" | "side", Direction::NegX) => Some(Orient::with("axis", "z")),
                ("" | "side", Direction::NegY) => Some(Orient::with("axis", "x")),
                ("" | "side", Direction::NegZ) => Some(Orient::with("axis", "x")),
                _ => None,
            },
            Texturing::Facing4 => match (self.part.as_deref()?, normal) {
                ("top", Direction::PosY) => Some(Orient::AsIs),
                ("bottom", Direction::NegY) => Some(Orient::AsIs),
                ("front", Direction::NegZ) => Some(Orient::with("facing", "north")),
                ("front", Direction::PosZ) => Some(Orient::with("facing", "south")),
                ("front", Direction::PosX) => Some(Orient::with("facing", "east")),
                ("front", Direction::NegX) => Some(Orient::with("facing", "west")),
                ("side", Direction::PosZ) => Some(Orient::with("facing", "north")),
                ("side", Direction::NegZ) => Some(Orient::with("facing", "south")),
                ("side", Direction::NegX) => Some(Orient::with("facing", "east")),
                ("side", Direction::PosX) => Some(Orient::with("facing", "west")),
                _ => None,
            },
            Texturing::Facing6 => match (self.part.as_deref()?, normal) {
                ("front", Direction::PosY) => Some(Orient::with("facing", "up")),
                ("front", Direction::NegY) => Some(Orient::with("facing", "down")),
                ("front", Direction::NegZ) => Some(Orient::with("facing", "north")),
                ("front", Direction::PosZ) => Some(Orient::with("facing", "south")),
                ("front", Direction::PosX) => Some(Orient::with("facing", "east")),
                ("front", Direction::NegX) => Some(Orient::with("facing", "west")),
                ("back", Direction::NegY) => Some(Orient::with("facing", "up")),
                ("back", Direction::PosY) => Some(Orient::with("facing", "down")),
                ("back", Direction::PosZ) => Some(Orient::with("facing", "north")),
                ("back", Direction::NegZ) => Some(Orient::with("facing", "south")),
                ("back", Direction::NegX) => Some(Orient::with("facing", "east")),
                ("back", Direction::PosX) => Some(Orient::with("facing", "west")),
                _ => None,
            },
            Texturing::Upright => match (self.part.as_deref()?, normal) {
                ("top", Direction::PosY | Direction::NegY) => Some(Orient::AsIs),
                ("side" | "", Direction::PosX | Direction::NegX) => Some(Orient::AsIs),
                ("side" | "", Direction::PosZ | Direction::NegZ) => Some(Orient::AsIs),
                _ => None,
            },
        }
    }

    fn distance(&self, color: &[u8]) -> i32 {
        fn diff_squared(a: u8, b: u8) -> i32 {
            (a as i32 - b as i32).pow(2)
        }

        diff_squared(self.avg_color[0], color[0])
            + diff_squared(self.avg_color[1], color[1])
            + diff_squared(self.avg_color[2], color[2])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize)]
enum Texturing {
    #[serde(rename = "uniform")]
    Uniform,
    #[serde(rename = "axis")]
    Axis,
    #[serde(rename = "facing4")]
    Facing4,
    #[serde(rename = "facing6")]
    Facing6,
    #[serde(rename = "upright")]
    Upright,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Direction {
    pub fn from_unit(unit: (i32, i32, i32)) -> Result<Self, ()> {
        match unit {
            (x, 0, 0) if x > 0 => Ok(Self::PosX),
            (0, y, 0) if y > 0 => Ok(Self::PosY),
            (0, 0, z) if z > 0 => Ok(Self::PosZ),
            (x, 0, 0) if x < 0 => Ok(Self::NegX),
            (0, y, 0) if y < 0 => Ok(Self::NegY),
            (0, 0, z) if z < 0 => Ok(Self::NegZ),
            _ => Err(()),
        }
    }
}

enum Orient {
    AsIs,
    With {
        name: &'static str,
        value: &'static str,
    },
}

impl Orient {
    fn with(name: &'static str, value: &'static str) -> Self {
        Self::With { name, value }
    }
}

pub fn find_closest(color: &[u8], normal: Direction) -> Palette {
    let mut closest_dist = i32::MAX;
    let mut palette = Palette::new("air");

    for block in ALL_BLOCKS.iter() {
        let Some(orient) = block.try_orient(block.texturing, normal) else {
            continue;
        };

        let dist = block.distance(color);
        if dist >= closest_dist {
            continue;
        }

        closest_dist = dist;
        let p = Palette::new(&block.block_id);
        if let Orient::With { name, value } = orient {
            palette = p.with_property(name, value);
        } else {
            palette = p;
        }
    }

    palette
}
