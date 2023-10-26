use crate::nbt::Palette;
use glob::glob;
use image::io::Reader as ImageReader;
use image::Pixel;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

static ALL_BLOCKS: Lazy<Vec<Block>> = Lazy::new(read_blocks);

const SUPPORTED_BLOCKS: [(&str, Texturing); 307] = [
    ("acacia_log", Texturing::Distinct(Positioning::Axis)),
    ("acacia_stem", Texturing::Uniform),
    ("acacia_wood", Texturing::Uniform),
    ("amethyst_block", Texturing::Uniform),
    ("ancient_debris", Texturing::Distinct(Positioning::Upright)),
    ("andesite", Texturing::Uniform),
    ("bamboo_block", Texturing::Distinct(Positioning::Axis)),
    ("bamboo_mosaic", Texturing::Uniform),
    ("bamboo_stem", Texturing::Uniform),
    ("barrel", Texturing::Distinct(Positioning::Facing6)),
    ("basalt", Texturing::Distinct(Positioning::Axis)),
    ("bee_nest", Texturing::Distinct(Positioning::Facing4)),
    ("beehive", Texturing::Distinct(Positioning::Facing4)),
    ("birch_log", Texturing::Distinct(Positioning::Axis)),
    ("birch_stem", Texturing::Uniform),
    ("birch_wood", Texturing::Uniform),
    ("black_concrete", Texturing::Uniform),
    ("black_concrete_powder", Texturing::Uniform),
    ("black_glazed_terracotta", Texturing::Uniform),
    ("black_terracotta", Texturing::Uniform),
    ("black_wool", Texturing::Uniform),
    ("blackstone", Texturing::Distinct(Positioning::Upright)),
    ("blast_furnace", Texturing::Distinct(Positioning::Facing4)),
    ("blue_concrete", Texturing::Uniform),
    ("blue_concrete_powder", Texturing::Uniform),
    ("blue_glazed_terracotta", Texturing::Uniform),
    ("blue_ice", Texturing::Uniform),
    ("blue_terracotta", Texturing::Uniform),
    ("blue_wool", Texturing::Uniform),
    ("bone_block", Texturing::Distinct(Positioning::Axis)),
    ("bookshelf", Texturing::Distinct(Positioning::Upright)),
    ("bricks", Texturing::Uniform),
    ("brown_concrete", Texturing::Uniform),
    ("brown_concrete_powder", Texturing::Uniform),
    ("brown_glazed_terracotta", Texturing::Uniform),
    ("brown_mushroom_block", Texturing::Uniform),
    ("brown_terracotta", Texturing::Uniform),
    ("brown_wool", Texturing::Uniform),
    ("calcite", Texturing::Uniform),
    ("cartography_table", Texturing::Distinct(Positioning::Upright), ),
    ("carved_pumpkin", Texturing::Distinct(Positioning::Facing4)),
    ("cauldron", Texturing::Distinct(Positioning::Upright)),
    ("cherry_log", Texturing::Distinct(Positioning::Axis)),
    ("cherry_stem", Texturing::Uniform),
    ("cherry_wood", Texturing::Uniform),
    ("chiseled_bookshelf", Texturing::Distinct(Positioning::Facing4), ),
    ("chiseled_deepslate", Texturing::Distinct(Positioning::Upright)),
    ("chiseled_nether_bricks", Texturing::Uniform),
    ("chiseled_polished_blackstone", Texturing::Distinct(Positioning::Upright)),
    ("chiseled_quartz_block", Texturing::Distinct(Positioning::Upright)),
    ("chiseled_red_sandstone", Texturing::Distinct(Positioning::Upright), ),
    ("chiseled_sandstone", Texturing::Distinct(Positioning::Upright), ),
    ("chiseled_stone_bricks", Texturing::Uniform),
    ("clay", Texturing::Uniform),
    ("coal_block", Texturing::Uniform),
    ("coal_ore", Texturing::Uniform),
    ("coarse_dirt", Texturing::Uniform),
    ("cobbled_deepslate", Texturing::Uniform),
    ("cobblestone", Texturing::Uniform),
    ("composter", Texturing::Distinct(Positioning::Upright)),
    ("copper_ore", Texturing::Uniform),
    ("cracked_deepslate_bricks", Texturing::Uniform),
    ("cracked_deepslate_tiles", Texturing::Uniform),
    ("cracked_nether_bricks", Texturing::Uniform),
    ("cracked_polished_blackstone_bricks", Texturing::Uniform),
    ("cracked_stone_bricks", Texturing::Uniform),
    ("crafting_table", Texturing::Distinct(Positioning::Upright)),
    ("crimson_nylium", Texturing::Uniform),
    ("crimson_stem", Texturing::Distinct(Positioning::Axis)),
    ("crying_obsidian", Texturing::Uniform),
    ("cut_red_sandstone", Texturing::Distinct(Positioning::Upright)),
    ("cut_sandstone", Texturing::Distinct(Positioning::Upright)),
    ("cyan_concrete", Texturing::Uniform),
    ("cyan_concrete_powder", Texturing::Uniform),
    ("cyan_glazed_terracotta", Texturing::Uniform),
    ("cyan_terracotta", Texturing::Uniform),
    ("cyan_wool", Texturing::Uniform),
    ("dark_oak_log", Texturing::Distinct(Positioning::Axis)),
    ("dark_oak_stem", Texturing::Uniform),
    ("dark_oak_wood", Texturing::Uniform),
    ("dark_prismarine", Texturing::Uniform),
    ("dead_brain_coral_block", Texturing::Uniform),
    ("dead_bubble_coral_block", Texturing::Uniform),
    ("dead_fire_coral_block", Texturing::Uniform),
    ("dead_horn_coral_block", Texturing::Uniform),
    ("dead_tube_coral_block", Texturing::Uniform),
    ("deepslate", Texturing::Distinct(Positioning::Axis)),
    ("deepslate_bricks", Texturing::Uniform),
    ("deepslate_coal_ore", Texturing::Uniform),
    ("deepslate_copper_ore", Texturing::Uniform),
    ("deepslate_diamond_ore", Texturing::Uniform),
    ("deepslate_emerald_ore", Texturing::Uniform),
    ("deepslate_gold_ore", Texturing::Uniform),
    ("deepslate_iron_ore", Texturing::Uniform),
    ("deepslate_lapis_ore", Texturing::Uniform),
    ("deepslate_redstone_ore", Texturing::Uniform),
    ("deepslate_tiles", Texturing::Uniform),
    ("diamond_block", Texturing::Uniform),
    ("diamond_ore", Texturing::Uniform),
    ("diorite", Texturing::Uniform),
    ("dirt", Texturing::Uniform),
    ("dirt_path", Texturing::Distinct(Positioning::Upright)),
    ("dispenser", Texturing::Distinct(Positioning::Facing6)),
    ("dried_kelp_block", Texturing::Distinct(Positioning::Upright)),
    ("dripstone_block", Texturing::Uniform),
    ("dropper", Texturing::Distinct(Positioning::Facing6)),
    ("emerald_block", Texturing::Uniform),
    ("emerald_ore", Texturing::Uniform),
    ("end_stone", Texturing::Uniform),
    ("end_stone_bricks", Texturing::Uniform),
    ("fletching_table", Texturing::Distinct(Positioning::Upright)),
    ("furnace", Texturing::Distinct(Positioning::Facing4)),
    ("gilded_blackstone", Texturing::Uniform),
    ("glowstone", Texturing::Uniform),
    ("gold_block", Texturing::Uniform),
    ("gold_ore", Texturing::Uniform),
    ("granite", Texturing::Uniform),
    ("gravel", Texturing::Uniform),
    ("gray_concrete", Texturing::Uniform),
    ("gray_concrete_powder", Texturing::Uniform),
    ("gray_glazed_terracotta", Texturing::Uniform),
    ("gray_terracotta", Texturing::Uniform),
    ("gray_wool", Texturing::Uniform),
    ("green_concrete", Texturing::Uniform),
    ("green_concrete_powder", Texturing::Uniform),
    ("green_glazed_terracotta", Texturing::Uniform),
    ("green_terracotta", Texturing::Uniform),
    ("green_wool", Texturing::Uniform),
    ("hay_block", Texturing::Distinct(Positioning::Axis)),
    ("honey_block", Texturing::Uniform),
    ("honeycomb_block", Texturing::Uniform),
    ("iron_block", Texturing::Uniform),
    ("iron_ore", Texturing::Uniform),
    ("jack_o_lantern", Texturing::Distinct(Positioning::Facing4)),
    ("jukebox", Texturing::Distinct(Positioning::Upright)),
    ("jungle_log", Texturing::Distinct(Positioning::Axis)),
    ("jungle_stem", Texturing::Uniform),
    ("jungle_wood", Texturing::Uniform),
    ("lapis_block", Texturing::Uniform),
    ("lapis_ore", Texturing::Uniform),
    ("light_blue_concrete", Texturing::Uniform),
    ("light_blue_concrete_powder", Texturing::Uniform),
    ("light_blue_glazed_terracotta", Texturing::Uniform),
    ("light_blue_terracotta", Texturing::Uniform),
    ("light_blue_wool", Texturing::Uniform),
    ("light_gray_concrete", Texturing::Uniform),
    ("light_gray_concrete_powder", Texturing::Uniform),
    ("light_gray_glazed_terracotta", Texturing::Uniform),
    ("light_gray_terracotta", Texturing::Uniform),
    ("light_gray_wool", Texturing::Uniform),
    ("lime_concrete", Texturing::Uniform),
    ("lime_concrete_powder", Texturing::Uniform),
    ("lime_glazed_terracotta", Texturing::Uniform),
    ("lime_terracotta", Texturing::Uniform),
    ("lime_wool", Texturing::Uniform),
    ("lodestone", Texturing::Distinct(Positioning::Upright)),
    ("loom", Texturing::Distinct(Positioning::Facing4)),
    ("magenta_concrete", Texturing::Uniform),
    ("magenta_concrete_powder", Texturing::Uniform),
    ("magenta_glazed_terracotta", Texturing::Uniform),
    ("magenta_terracotta", Texturing::Uniform),
    ("magenta_wool", Texturing::Uniform),
    ("magma_block", Texturing::Uniform),
    ("mangrove_log", Texturing::Distinct(Positioning::Axis)),
    ("mangrove_stem", Texturing::Uniform),
    ("mangrove_roots", Texturing::Distinct(Positioning::Upright)),
    ("mangrove_wood", Texturing::Uniform),
    ("melon", Texturing::Distinct(Positioning::Upright)),
    ("moss_block", Texturing::Uniform),
    ("mossy_cobblestone", Texturing::Uniform),
    ("mossy_stone_bricks", Texturing::Uniform),
    ("mud", Texturing::Uniform),
    ("mud_bricks", Texturing::Uniform),
    ("muddy_mangrove_roots", Texturing::Distinct(Positioning::Axis), ),
    ("mycelium", Texturing::Distinct(Positioning::Upright)),
    ("nether_bricks", Texturing::Uniform),
    ("nether_gold_ore", Texturing::Uniform),
    ("nether_quartz_ore", Texturing::Uniform),
    ("nether_wart_block", Texturing::Uniform),
    ("netherite_block", Texturing::Uniform),
    ("netherrack", Texturing::Uniform),
    ("note_block", Texturing::Uniform),
    ("oak_log", Texturing::Distinct(Positioning::Axis)),
    ("oak_stem", Texturing::Uniform),
    ("oak_wood", Texturing::Uniform),
    ("observer", Texturing::Distinct(Positioning::Facing6)),
    ("obsidian", Texturing::Uniform),
    ("ochre_froglight", Texturing::Distinct(Positioning::Axis)),
    ("orange_concrete", Texturing::Uniform),
    ("orange_concrete_powder", Texturing::Uniform),
    ("orange_glazed_terracotta", Texturing::Uniform),
    ("orange_terracotta", Texturing::Uniform),
    ("orange_wool", Texturing::Uniform),
    ("packed_ice", Texturing::Uniform),
    ("packed_mud", Texturing::Uniform),
    ("pearlescent_froglight", Texturing::Distinct(Positioning::Axis), ),
    ("pink_concrete", Texturing::Uniform),
    ("pink_concrete_powder", Texturing::Uniform),
    ("pink_glazed_terracotta", Texturing::Uniform),
    ("pink_terracotta", Texturing::Uniform),
    ("pink_wool", Texturing::Uniform),
    ("podzol", Texturing::Distinct(Positioning::Upright)),
    ("piston", Texturing::Distinct(Positioning::Facing6)),
    ("polished_andesite", Texturing::Uniform),
    ("polished_basalt", Texturing::Distinct(Positioning::Axis)),
    ("polished_blackstone", Texturing::Uniform),
    ("polished_blackstone_bricks", Texturing::Uniform),
    ("polished_deepslate", Texturing::Uniform),
    ("polished_diorite", Texturing::Uniform),
    ("polished_granite", Texturing::Uniform),
    ("powder_snow", Texturing::Uniform),
    ("prismarine", Texturing::Uniform),
    ("prismarine_bricks", Texturing::Uniform),
    ("pumpkin", Texturing::Distinct(Positioning::Facing4)),
    ("purple_concrete", Texturing::Uniform),
    ("purple_concrete_powder", Texturing::Uniform),
    ("purple_glazed_terracotta", Texturing::Uniform),
    ("purple_terracotta", Texturing::Uniform),
    ("purple_wool", Texturing::Uniform),
    ("purpur_block", Texturing::Uniform),
    ("purpur_pillar", Texturing::Distinct(Positioning::Axis)),
    ("quartz_block", Texturing::Distinct(Positioning::Upright)),
    ("quartz_bricks", Texturing::Uniform),
    ("quartz_pillar", Texturing::Distinct(Positioning::Axis)),
    ("raw_copper_block", Texturing::Uniform),
    ("raw_gold_block", Texturing::Uniform),
    ("raw_iron_block", Texturing::Uniform),
    ("red_concrete", Texturing::Uniform),
    ("red_concrete_powder", Texturing::Uniform),
    ("red_glazed_terracotta", Texturing::Uniform),
    ("red_mushroom_block", Texturing::Uniform),
    ("red_nether_bricks", Texturing::Uniform),
    ("red_sand", Texturing::Uniform),
    ("red_sandstone", Texturing::Distinct(Positioning::Upright)),
    ("red_terracotta", Texturing::Uniform),
    ("red_wool", Texturing::Uniform),
    ("redstone_block", Texturing::Uniform),
    ("redstone_ore", Texturing::Uniform),
    ("respawn_anchor", Texturing::Distinct(Positioning::Upright)),
    ("rooted_dirt", Texturing::Uniform),
    ("sand", Texturing::Uniform),
    ("sandstone", Texturing::Distinct(Positioning::Upright)),
    ("sculk", Texturing::Uniform),
    ("sculk_catalyst", Texturing::Distinct(Positioning::Upright)),
    ("sea_lantern", Texturing::Uniform),
    ("shroomlight", Texturing::Uniform),
    ("smithing_table", Texturing::Distinct(Positioning::Upright)),
    ("smoker", Texturing::Distinct(Positioning::Facing4)),
    ("smooth_basalt", Texturing::Uniform),
    ("smooth_quartz", Texturing::Uniform),
    ("smooth_red_sandstone", Texturing::Uniform),
    ("smooth_sandstone", Texturing::Uniform),
    ("smooth_stone", Texturing::Uniform),
    ("snow_block", Texturing::Uniform),
    ("soul_sand", Texturing::Uniform),
    ("soul_soil", Texturing::Uniform),
    ("sponge", Texturing::Uniform),
    ("spruce_log", Texturing::Distinct(Positioning::Axis)),
    ("spruce_stem", Texturing::Uniform),
    ("spruce_wood", Texturing::Uniform),
    ("stone", Texturing::Uniform),
    ("stone_bricks", Texturing::Uniform),
    ("stripped_acacia_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_acacia_wood", Texturing::Uniform),
    ("stripped_bamboo_block", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_birch_log", Texturing::Distinct(Positioning::Axis)),
    ("stripped_birch_wood", Texturing::Uniform),
    ("stripped_cherry_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_cherry_wood", Texturing::Uniform),
    ("stripped_crimson_stem", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_dark_oak_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_dark_oak_wood", Texturing::Uniform),
    ("stripped_jungle_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_jungle_wood", Texturing::Uniform),
    ("stripped_mangrove_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_mangrove_wood", Texturing::Uniform),
    ("stripped_oak_log", Texturing::Distinct(Positioning::Axis)),
    ("stripped_oak_wood", Texturing::Uniform),
    ("stripped_spruce_log", Texturing::Distinct(Positioning::Axis), ),
    ("stripped_spruce_wood", Texturing::Uniform),
    ("stripped_warped_stem", Texturing::Distinct(Positioning::Axis), ),
    ("target", Texturing::Distinct(Positioning::Upright)),
    ("terracotta", Texturing::Uniform),
    ("tnt", Texturing::Distinct(Positioning::Upright)),
    ("tuff", Texturing::Uniform),
    ("verdant_froglight", Texturing::Distinct(Positioning::Axis)),
    ("warped_stem", Texturing::Distinct(Positioning::Axis)),
    ("warped_wart_block", Texturing::Uniform),
    ("waxed_copper_block", Texturing::Uniform),
    ("waxed_cut_copper", Texturing::Uniform),
    ("waxed_exposed_copper", Texturing::Uniform),
    ("waxed_exposed_cut_copper", Texturing::Uniform),
    ("waxed_oxidized_copper", Texturing::Uniform),
    ("waxed_oxidized_cut_copper", Texturing::Uniform),
    ("waxed_weathered_copper", Texturing::Uniform),
    ("waxed_weathered_cut_copper", Texturing::Uniform),
    ("wet_sponge", Texturing::Uniform),
    ("white_concrete", Texturing::Uniform),
    ("white_concrete_powder", Texturing::Uniform),
    ("white_glazed_terracotta", Texturing::Uniform),
    ("white_terracotta", Texturing::Uniform),
    ("white_wool", Texturing::Uniform),
    ("yellow_concrete", Texturing::Uniform),
    ("yellow_concrete_powder", Texturing::Uniform),
    ("yellow_glazed_terracotta", Texturing::Uniform),
    ("yellow_terracotta", Texturing::Uniform),
    ("yellow_wool", Texturing::Uniform),
];

fn read_blocks() -> Vec<Block> {
    let mut blocks = vec![];
    for (block_id, texturing) in SUPPORTED_BLOCKS {
        let mut block = Block {
            block_id: block_id.to_string(),
            sides: vec![],
            texturing,
        };

        let textures = read_textures(block_id);
        for (aspect, path) in textures.into_iter() {
            block.sides.push(Texture {
                aspect,
                avg_color: measure_avg(&path),
            });
        }

        blocks.push(block);
    }

    blocks
}

fn read_textures(block_id: &str) -> Vec<(String, PathBuf)> {
    const BASE: &str = "minecraft/1.20.1/assets/minecraft/textures/block";
    let mut textures = vec![];
    for result in glob(&format!("{BASE}/{block_id}*.png")).unwrap() {
        let Ok(path) = result else { continue };

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let mut suffix = file_name
            .strip_suffix(".png")
            .unwrap()
            .strip_prefix(block_id)
            .unwrap();

        if !suffix.is_empty() {
            let Some(cleaned) = suffix.strip_prefix('_') else {
                continue;
            };

            if cleaned.contains('_')
                || block_id.ends_with("concrete") && cleaned == "powder"
                || block_id == "stone" && cleaned == "bricks"
                || block_id == "sculk" && cleaned == "vein"
                || block_id == "melon" && cleaned == "stem"
                || cleaned == "block"
                || cleaned == "bricks"
                || cleaned == "tiles"
            {
                continue;
            }

            suffix = cleaned;
        }

        textures.push((suffix.to_string(), path));
    }

    textures
}

fn measure_avg(path: &Path) -> [u8; 3] {
    let image = ImageReader::open(path).unwrap().decode().unwrap();
    if let Some(rgba) = image.as_rgba8() {
        return measure_avg_impl(rgba.pixels());
    }

    if let Some(rgb) = image.as_rgb8() {
        return measure_avg_impl(rgb.pixels());
    }

    unimplemented!()
}

fn measure_avg_impl<'a, I, P>(pixels: I) -> [u8; 3]
where
    I: Iterator<Item = &'a P>,
    P: Pixel<Subpixel = u8> + 'a,
{
    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
    let mut count = 0;
    for pixel in pixels {
        let channels = pixel.channels();
        if P::CHANNEL_COUNT == 4 && channels[3] < 128 {
            continue;
        }

        r += channels[0] as u32;
        g += channels[1] as u32;
        b += channels[2] as u32;
        count += 1;
    }

    [(r / count) as u8, (g / count) as u8, (b / count) as u8]
}

struct Block {
    block_id: String,
    sides: Vec<Texture>,
    texturing: Texturing,
}

struct Texture {
    aspect: String,
    avg_color: [u8; 3],
}

impl Texture {
    fn try_orient(&self, texturing: Texturing, normal: Direction) -> Option<Orient> {
        match texturing {
            Texturing::Uniform => Some(Orient::AsIs),
            Texturing::Distinct(Positioning::Axis) => match (self.aspect.as_str(), normal) {
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
            Texturing::Distinct(Positioning::Facing4) => match (self.aspect.as_str(), normal) {
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
            Texturing::Distinct(Positioning::Facing6) => match (self.aspect.as_str(), normal) {
                ("front", Direction::PosY) => Some(Orient::With { name: "facing", value: "up" }),
                ("front", Direction::NegY) => Some(Orient::With { name: "facing", value: "down" }),
                ("front", Direction::NegZ) => Some(Orient::With { name: "facing", value: "north" }),
                ("front", Direction::PosZ) => Some(Orient::With { name: "facing", value: "south" }),
                ("front", Direction::PosX) => Some(Orient::With { name: "facing", value: "east" }),
                ("front", Direction::NegX) => Some(Orient::With { name: "facing", value: "west" }),
                ("back", Direction::NegY) => Some(Orient::With { name: "facing", value: "up" }),
                ("back", Direction::PosY) => Some(Orient::With { name: "facing", value: "down" }),
                ("back", Direction::PosZ) => Some(Orient::With { name: "facing", value: "north" }),
                ("back", Direction::NegZ) => Some(Orient::With { name: "facing", value: "south" }),
                ("back", Direction::NegX) => Some(Orient::With { name: "facing", value: "east" }),
                ("back", Direction::PosX) => Some(Orient::With { name: "facing", value: "west" }),
                _ => None,
            },
            Texturing::Distinct(Positioning::Upright) => match (self.aspect.as_str(), normal) {
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
    pub fn from_unit(unit: (i32, i32, i32)) -> Self {
        match unit {
            (x, 0, 0) if x > 0 => Self::PosX,
            (0, y, 0) if y > 0 => Self::PosY,
            (0, 0, z) if z > 0 => Self::PosZ,
            (x, 0, 0) if x < 0 => Self::NegX,
            (0, y, 0) if y < 0 => Self::NegY,
            (0, 0, z) if z < 0 => Self::NegZ,
            _ => panic!("Not a unit vector"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Texturing {
    Uniform,
    Distinct(Positioning),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Positioning {
    Axis,
    Facing4,
    Facing6,
    Upright,
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
        for side in block.sides.iter() {
            let Some(orient) = side.try_orient(block.texturing, normal) else {
                continue;
            };

            let dist = side.distance(color);
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
    }

    palette
}
