use crate::items::get_item;
use crate::mob::get_mob;
use crate::model::Model;
use crate::nbt::{DataVersion, Structure};
use crate::skin::Skin;
use std::fs::File;

mod drawing;
mod items;
mod materials;
mod mob;
mod model;
mod nbt;
mod skin;
mod transform;

fn main() -> std::io::Result<()> {
    let mut structure = Structure::new(DataVersion::Minecraft1_20_1);

    let Some(args) = Args::parse() else {
        println!("Invalid arguments");
        return Ok(());
    };

    match args.what {
        What::Skin { player } => {
            let skin = Skin::player(&player);
            skin.draw(&mut structure, args.model_scale)?;
        }
        What::Item { name } => {
            let Some(item) = get_item(&name) else {
                println!("Unsupported item \"{name}\"");
                return Ok(());
            };

            item.draw(&mut structure, args.model_scale)?;
        }
        What::Mob { name } => {
            let Some(item) = get_mob(&name) else {
                println!("Unsupported mob \"{name}\"");
                return Ok(());
            };

            item.draw(&mut structure, args.model_scale)?;
        }
    }

    let mut f = File::create("output.nbt")?;
    structure.normalize();
    structure.write_out(&mut f)?;

    Ok(())
}

struct Args {
    model_scale: u32,
    texture_scale: u32,
    what: What,
}

impl Args {
    fn parse() -> Option<Self> {
        let mut args = std::env::args();
        args.next(); // Skip program name

        let mut x = Args {
            model_scale: 1,
            texture_scale: 1,
            what: match args.next()?.as_str() {
                "skin" => What::Skin {
                    player: args.next()?,
                },
                "item" => What::Item { name: args.next()? },
                "mob" => What::Mob { name: args.next()? },
                _ => return None,
            },
        };

        for a in args {
            if let Some(n) = a.strip_prefix("--model-scale=") {
                x.model_scale = n.parse().ok()?;
            } else if let Some(n) = a.strip_prefix("--texture-scale=") {
                x.texture_scale = n.parse().ok()?;
            }
        }

        Some(x)
    }
}

enum What {
    Skin { player: String },
    Item { name: String },
    Mob { name: String },
}
