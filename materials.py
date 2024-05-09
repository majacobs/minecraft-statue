#!/usr/bin/env python

import json

from enum import Enum
from glob import iglob
from pathlib import Path
from typing import Iterator, Tuple, Optional

from PIL import Image

ROOT_TEXTURE_DIR = Path("minecraft/1.20.1/assets/minecraft/textures")
OUTPUT_FILE = Path("materials.json")


def main():
    blocks = []
    for block_id, texturing in SUPPORTED_BLOCKS:
        for suffix, path in get_textures(block_id):
            try:
                color = average_color(path)
            except:
                print(path)
                raise
            blocks.append(
                {
                    "block_id": block_id,
                    "texturing": texturing,
                    "part": suffix,
                    "avg_color": color,
                }
            )

    with OUTPUT_FILE.open("w") as f:
        json.dump(blocks, f, cls=JSONEncoder, indent=2)


def get_textures(block_id: str) -> Iterator[Tuple[Optional[str], Path]]:
    texture_dir = ROOT_TEXTURE_DIR / "block"
    for path in iglob(f"{block_id}*.png", root_dir=texture_dir):
        suffix = path.removeprefix(block_id).removesuffix(".png")
        if not suffix:
            yield (None, texture_dir / path)
            continue

        cleaned = suffix.removeprefix("_")
        if cleaned == suffix:
            # Skip cases like block_id="stone" suffix="cutter_saw"
            continue

        if (
            "_" in cleaned
            or (block_id.endswith("concrete") and cleaned == "powder")
            or (block_id == "stone" and cleaned == "bricks")
            or (block_id == "sculk" and cleaned == "vein")
            or (block_id == "melon" and cleaned == "stem")
            or cleaned == "block"
            or cleaned == "bricks"
            or cleaned == "tiles"
        ):
            continue

        yield (cleaned, texture_dir / path)


def average_color(path: Path) -> Tuple[int, int, int]:
    im = Image.open(path).convert("RGBA")
    rs, gs, bs = 0, 0, 0
    count = 0
    asdf = list(im.getdata())
    for r, g, b, a in asdf:
        if a < 128:
            continue
        rs += r
        gs += g
        bs += b
        count += 1
    return (rs // count, gs // count, bs // count)


class Texturing(Enum):
    UNIFORM = 1
    AXIS = 2
    FACING_4 = 3
    FACING_6 = 4
    UPRIGHT = 5


SUPPORTED_BLOCKS = [
    ("acacia_log", Texturing.AXIS),
    ("acacia_stem", Texturing.UNIFORM),
    ("acacia_wood", Texturing.UNIFORM),
    ("amethyst_block", Texturing.UNIFORM),
    ("ancient_debris", Texturing.UPRIGHT),
    ("andesite", Texturing.UNIFORM),
    ("bamboo_block", Texturing.AXIS),
    ("bamboo_mosaic", Texturing.UNIFORM),
    ("bamboo_stem", Texturing.UNIFORM),
    ("barrel", Texturing.FACING_6),
    ("basalt", Texturing.AXIS),
    ("bee_nest", Texturing.FACING_4),
    ("beehive", Texturing.FACING_4),
    ("birch_log", Texturing.AXIS),
    ("birch_stem", Texturing.UNIFORM),
    ("birch_wood", Texturing.UNIFORM),
    ("black_concrete", Texturing.UNIFORM),
    ("black_concrete_powder", Texturing.UNIFORM),
    ("black_glazed_terracotta", Texturing.UNIFORM),
    ("black_terracotta", Texturing.UNIFORM),
    ("black_wool", Texturing.UNIFORM),
    ("blackstone", Texturing.UPRIGHT),
    ("blast_furnace", Texturing.FACING_4),
    ("blue_concrete", Texturing.UNIFORM),
    ("blue_concrete_powder", Texturing.UNIFORM),
    ("blue_glazed_terracotta", Texturing.UNIFORM),
    ("blue_ice", Texturing.UNIFORM),
    ("blue_terracotta", Texturing.UNIFORM),
    ("blue_wool", Texturing.UNIFORM),
    ("bone_block", Texturing.AXIS),
    ("bookshelf", Texturing.UPRIGHT),
    ("bricks", Texturing.UNIFORM),
    ("brown_concrete", Texturing.UNIFORM),
    ("brown_concrete_powder", Texturing.UNIFORM),
    ("brown_glazed_terracotta", Texturing.UNIFORM),
    ("brown_mushroom_block", Texturing.UNIFORM),
    ("brown_terracotta", Texturing.UNIFORM),
    ("brown_wool", Texturing.UNIFORM),
    ("calcite", Texturing.UNIFORM),
    ("cartography_table", Texturing.UPRIGHT),
    ("carved_pumpkin", Texturing.FACING_4),
    ("cauldron", Texturing.UPRIGHT),
    ("cherry_log", Texturing.AXIS),
    ("cherry_stem", Texturing.UNIFORM),
    ("cherry_wood", Texturing.UNIFORM),
    ("chiseled_bookshelf", Texturing.FACING_4),
    ("chiseled_deepslate", Texturing.UPRIGHT),
    ("chiseled_nether_bricks", Texturing.UNIFORM),
    ("chiseled_polished_blackstone", Texturing.UPRIGHT),
    ("chiseled_quartz_block", Texturing.UPRIGHT),
    ("chiseled_red_sandstone", Texturing.UPRIGHT),
    ("chiseled_sandstone", Texturing.UPRIGHT),
    ("chiseled_stone_bricks", Texturing.UNIFORM),
    ("clay", Texturing.UNIFORM),
    ("coal_block", Texturing.UNIFORM),
    ("coal_ore", Texturing.UNIFORM),
    ("coarse_dirt", Texturing.UNIFORM),
    ("cobbled_deepslate", Texturing.UNIFORM),
    ("cobblestone", Texturing.UNIFORM),
    ("composter", Texturing.UPRIGHT),
    ("copper_ore", Texturing.UNIFORM),
    ("cracked_deepslate_bricks", Texturing.UNIFORM),
    ("cracked_deepslate_tiles", Texturing.UNIFORM),
    ("cracked_nether_bricks", Texturing.UNIFORM),
    ("cracked_polished_blackstone_bricks", Texturing.UNIFORM),
    ("cracked_stone_bricks", Texturing.UNIFORM),
    ("crafting_table", Texturing.UPRIGHT),
    ("crimson_nylium", Texturing.UNIFORM),
    ("crimson_stem", Texturing.AXIS),
    ("crying_obsidian", Texturing.UNIFORM),
    ("cut_red_sandstone", Texturing.UPRIGHT),
    ("cut_sandstone", Texturing.UPRIGHT),
    ("cyan_concrete", Texturing.UNIFORM),
    ("cyan_concrete_powder", Texturing.UNIFORM),
    ("cyan_glazed_terracotta", Texturing.UNIFORM),
    ("cyan_terracotta", Texturing.UNIFORM),
    ("cyan_wool", Texturing.UNIFORM),
    ("dark_oak_log", Texturing.AXIS),
    ("dark_oak_stem", Texturing.UNIFORM),
    ("dark_oak_wood", Texturing.UNIFORM),
    ("dark_prismarine", Texturing.UNIFORM),
    ("dead_brain_coral_block", Texturing.UNIFORM),
    ("dead_bubble_coral_block", Texturing.UNIFORM),
    ("dead_fire_coral_block", Texturing.UNIFORM),
    ("dead_horn_coral_block", Texturing.UNIFORM),
    ("dead_tube_coral_block", Texturing.UNIFORM),
    ("deepslate", Texturing.AXIS),
    ("deepslate_bricks", Texturing.UNIFORM),
    ("deepslate_coal_ore", Texturing.UNIFORM),
    ("deepslate_copper_ore", Texturing.UNIFORM),
    ("deepslate_diamond_ore", Texturing.UNIFORM),
    ("deepslate_emerald_ore", Texturing.UNIFORM),
    ("deepslate_gold_ore", Texturing.UNIFORM),
    ("deepslate_iron_ore", Texturing.UNIFORM),
    ("deepslate_lapis_ore", Texturing.UNIFORM),
    ("deepslate_redstone_ore", Texturing.UNIFORM),
    ("deepslate_tiles", Texturing.UNIFORM),
    ("diamond_block", Texturing.UNIFORM),
    ("diamond_ore", Texturing.UNIFORM),
    ("diorite", Texturing.UNIFORM),
    ("dirt", Texturing.UNIFORM),
    ("dirt_path", Texturing.UPRIGHT),
    ("dispenser", Texturing.FACING_6),
    ("dried_kelp_block", Texturing.UPRIGHT),
    ("dripstone_block", Texturing.UNIFORM),
    ("dropper", Texturing.FACING_6),
    ("emerald_block", Texturing.UNIFORM),
    ("emerald_ore", Texturing.UNIFORM),
    ("end_stone", Texturing.UNIFORM),
    ("end_stone_bricks", Texturing.UNIFORM),
    ("fletching_table", Texturing.UPRIGHT),
    ("furnace", Texturing.FACING_4),
    ("gilded_blackstone", Texturing.UNIFORM),
    ("glowstone", Texturing.UNIFORM),
    ("gold_block", Texturing.UNIFORM),
    ("gold_ore", Texturing.UNIFORM),
    ("granite", Texturing.UNIFORM),
    ("gravel", Texturing.UNIFORM),
    ("gray_concrete", Texturing.UNIFORM),
    ("gray_concrete_powder", Texturing.UNIFORM),
    ("gray_glazed_terracotta", Texturing.UNIFORM),
    ("gray_terracotta", Texturing.UNIFORM),
    ("gray_wool", Texturing.UNIFORM),
    ("green_concrete", Texturing.UNIFORM),
    ("green_concrete_powder", Texturing.UNIFORM),
    ("green_glazed_terracotta", Texturing.UNIFORM),
    ("green_terracotta", Texturing.UNIFORM),
    ("green_wool", Texturing.UNIFORM),
    ("hay_block", Texturing.AXIS),
    ("honey_block", Texturing.UNIFORM),
    ("honeycomb_block", Texturing.UNIFORM),
    ("iron_block", Texturing.UNIFORM),
    ("iron_ore", Texturing.UNIFORM),
    ("jack_o_lantern", Texturing.FACING_4),
    ("jukebox", Texturing.UPRIGHT),
    ("jungle_log", Texturing.AXIS),
    ("jungle_stem", Texturing.UNIFORM),
    ("jungle_wood", Texturing.UNIFORM),
    ("lapis_block", Texturing.UNIFORM),
    ("lapis_ore", Texturing.UNIFORM),
    ("light_blue_concrete", Texturing.UNIFORM),
    ("light_blue_concrete_powder", Texturing.UNIFORM),
    ("light_blue_glazed_terracotta", Texturing.UNIFORM),
    ("light_blue_terracotta", Texturing.UNIFORM),
    ("light_blue_wool", Texturing.UNIFORM),
    ("light_gray_concrete", Texturing.UNIFORM),
    ("light_gray_concrete_powder", Texturing.UNIFORM),
    ("light_gray_glazed_terracotta", Texturing.UNIFORM),
    ("light_gray_terracotta", Texturing.UNIFORM),
    ("light_gray_wool", Texturing.UNIFORM),
    ("lime_concrete", Texturing.UNIFORM),
    ("lime_concrete_powder", Texturing.UNIFORM),
    ("lime_glazed_terracotta", Texturing.UNIFORM),
    ("lime_terracotta", Texturing.UNIFORM),
    ("lime_wool", Texturing.UNIFORM),
    ("lodestone", Texturing.UPRIGHT),
    ("loom", Texturing.FACING_4),
    ("magenta_concrete", Texturing.UNIFORM),
    ("magenta_concrete_powder", Texturing.UNIFORM),
    ("magenta_glazed_terracotta", Texturing.UNIFORM),
    ("magenta_terracotta", Texturing.UNIFORM),
    ("magenta_wool", Texturing.UNIFORM),
    ("magma_block", Texturing.UNIFORM),
    ("mangrove_log", Texturing.AXIS),
    ("mangrove_stem", Texturing.UNIFORM),
    ("mangrove_roots", Texturing.UPRIGHT),
    ("mangrove_wood", Texturing.UNIFORM),
    ("melon", Texturing.UPRIGHT),
    ("moss_block", Texturing.UNIFORM),
    ("mossy_cobblestone", Texturing.UNIFORM),
    ("mossy_stone_bricks", Texturing.UNIFORM),
    ("mud", Texturing.UNIFORM),
    ("mud_bricks", Texturing.UNIFORM),
    ("muddy_mangrove_roots", Texturing.AXIS),
    ("mycelium", Texturing.UPRIGHT),
    ("nether_bricks", Texturing.UNIFORM),
    ("nether_gold_ore", Texturing.UNIFORM),
    ("nether_quartz_ore", Texturing.UNIFORM),
    ("nether_wart_block", Texturing.UNIFORM),
    ("netherite_block", Texturing.UNIFORM),
    ("netherrack", Texturing.UNIFORM),
    ("note_block", Texturing.UNIFORM),
    ("oak_log", Texturing.AXIS),
    ("oak_stem", Texturing.UNIFORM),
    ("oak_wood", Texturing.UNIFORM),
    ("observer", Texturing.FACING_6),
    ("obsidian", Texturing.UNIFORM),
    ("ochre_froglight", Texturing.AXIS),
    ("orange_concrete", Texturing.UNIFORM),
    ("orange_concrete_powder", Texturing.UNIFORM),
    ("orange_glazed_terracotta", Texturing.UNIFORM),
    ("orange_terracotta", Texturing.UNIFORM),
    ("orange_wool", Texturing.UNIFORM),
    ("packed_ice", Texturing.UNIFORM),
    ("packed_mud", Texturing.UNIFORM),
    ("pearlescent_froglight", Texturing.AXIS),
    ("pink_concrete", Texturing.UNIFORM),
    ("pink_concrete_powder", Texturing.UNIFORM),
    ("pink_glazed_terracotta", Texturing.UNIFORM),
    ("pink_terracotta", Texturing.UNIFORM),
    ("pink_wool", Texturing.UNIFORM),
    ("podzol", Texturing.UPRIGHT),
    ("piston", Texturing.FACING_6),
    ("polished_andesite", Texturing.UNIFORM),
    ("polished_basalt", Texturing.AXIS),
    ("polished_blackstone", Texturing.UNIFORM),
    ("polished_blackstone_bricks", Texturing.UNIFORM),
    ("polished_deepslate", Texturing.UNIFORM),
    ("polished_diorite", Texturing.UNIFORM),
    ("polished_granite", Texturing.UNIFORM),
    ("powder_snow", Texturing.UNIFORM),
    ("prismarine", Texturing.UNIFORM),
    ("prismarine_bricks", Texturing.UNIFORM),
    ("pumpkin", Texturing.FACING_4),
    ("purple_concrete", Texturing.UNIFORM),
    ("purple_concrete_powder", Texturing.UNIFORM),
    ("purple_glazed_terracotta", Texturing.UNIFORM),
    ("purple_terracotta", Texturing.UNIFORM),
    ("purple_wool", Texturing.UNIFORM),
    ("purpur_block", Texturing.UNIFORM),
    ("purpur_pillar", Texturing.AXIS),
    ("quartz_block", Texturing.UPRIGHT),
    ("quartz_bricks", Texturing.UNIFORM),
    ("quartz_pillar", Texturing.AXIS),
    ("raw_copper_block", Texturing.UNIFORM),
    ("raw_gold_block", Texturing.UNIFORM),
    ("raw_iron_block", Texturing.UNIFORM),
    ("red_concrete", Texturing.UNIFORM),
    ("red_concrete_powder", Texturing.UNIFORM),
    ("red_glazed_terracotta", Texturing.UNIFORM),
    ("red_mushroom_block", Texturing.UNIFORM),
    ("red_nether_bricks", Texturing.UNIFORM),
    ("red_sand", Texturing.UNIFORM),
    ("red_sandstone", Texturing.UPRIGHT),
    ("red_terracotta", Texturing.UNIFORM),
    ("red_wool", Texturing.UNIFORM),
    ("redstone_block", Texturing.UNIFORM),
    ("redstone_ore", Texturing.UNIFORM),
    ("respawn_anchor", Texturing.UPRIGHT),
    ("rooted_dirt", Texturing.UNIFORM),
    ("sand", Texturing.UNIFORM),
    ("sandstone", Texturing.UPRIGHT),
    ("sculk", Texturing.UNIFORM),
    ("sculk_catalyst", Texturing.UPRIGHT),
    ("sea_lantern", Texturing.UNIFORM),
    ("shroomlight", Texturing.UNIFORM),
    ("smithing_table", Texturing.UPRIGHT),
    ("smoker", Texturing.FACING_4),
    ("smooth_basalt", Texturing.UNIFORM),
    ("smooth_quartz", Texturing.UNIFORM),
    ("smooth_red_sandstone", Texturing.UNIFORM),
    ("smooth_sandstone", Texturing.UNIFORM),
    ("smooth_stone", Texturing.UNIFORM),
    ("snow_block", Texturing.UNIFORM),
    ("soul_sand", Texturing.UNIFORM),
    ("soul_soil", Texturing.UNIFORM),
    ("sponge", Texturing.UNIFORM),
    ("spruce_log", Texturing.AXIS),
    ("spruce_stem", Texturing.UNIFORM),
    ("spruce_wood", Texturing.UNIFORM),
    ("stone", Texturing.UNIFORM),
    ("stone_bricks", Texturing.UNIFORM),
    ("stripped_acacia_log", Texturing.AXIS),
    ("stripped_acacia_wood", Texturing.UNIFORM),
    ("stripped_bamboo_block", Texturing.AXIS),
    ("stripped_birch_log", Texturing.AXIS),
    ("stripped_birch_wood", Texturing.UNIFORM),
    ("stripped_cherry_log", Texturing.AXIS),
    ("stripped_cherry_wood", Texturing.UNIFORM),
    ("stripped_crimson_stem", Texturing.AXIS),
    ("stripped_dark_oak_log", Texturing.AXIS),
    ("stripped_dark_oak_wood", Texturing.UNIFORM),
    ("stripped_jungle_log", Texturing.AXIS),
    ("stripped_jungle_wood", Texturing.UNIFORM),
    ("stripped_mangrove_log", Texturing.AXIS),
    ("stripped_mangrove_wood", Texturing.UNIFORM),
    ("stripped_oak_log", Texturing.AXIS),
    ("stripped_oak_wood", Texturing.UNIFORM),
    ("stripped_spruce_log", Texturing.AXIS),
    ("stripped_spruce_wood", Texturing.UNIFORM),
    ("stripped_warped_stem", Texturing.AXIS),
    ("target", Texturing.UPRIGHT),
    ("terracotta", Texturing.UNIFORM),
    ("tnt", Texturing.UPRIGHT),
    ("tuff", Texturing.UNIFORM),
    ("verdant_froglight", Texturing.AXIS),
    ("warped_stem", Texturing.AXIS),
    ("warped_wart_block", Texturing.UNIFORM),
    ("waxed_copper_block", Texturing.UNIFORM),
    ("waxed_cut_copper", Texturing.UNIFORM),
    ("waxed_exposed_copper", Texturing.UNIFORM),
    ("waxed_exposed_cut_copper", Texturing.UNIFORM),
    ("waxed_oxidized_copper", Texturing.UNIFORM),
    ("waxed_oxidized_cut_copper", Texturing.UNIFORM),
    ("waxed_weathered_copper", Texturing.UNIFORM),
    ("waxed_weathered_cut_copper", Texturing.UNIFORM),
    ("wet_sponge", Texturing.UNIFORM),
    ("white_concrete", Texturing.UNIFORM),
    ("white_concrete_powder", Texturing.UNIFORM),
    ("white_glazed_terracotta", Texturing.UNIFORM),
    ("white_terracotta", Texturing.UNIFORM),
    ("white_wool", Texturing.UNIFORM),
    ("yellow_concrete", Texturing.UNIFORM),
    ("yellow_concrete_powder", Texturing.UNIFORM),
    ("yellow_glazed_terracotta", Texturing.UNIFORM),
    ("yellow_terracotta", Texturing.UNIFORM),
    ("yellow_wool", Texturing.UNIFORM),
]


class JSONEncoder(json.JSONEncoder):
    def default(self, o):
        match o:
            case Texturing.UNIFORM:
                return "uniform"
            case Texturing.AXIS:
                return "axis"
            case Texturing.FACING_4:
                return "facing4"
            case Texturing.FACING_6:
                return "facing6"
            case Texturing.UPRIGHT:
                return "upright"
            case _:
                return super().default(o)


if __name__ == "__main__":
    main()
