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
    const SCALING: i32 = 1;
    let mut structure = Structure::new(DataVersion::Minecraft1_20_1);

    let mut args = std::env::args();
    args.next(); // Skip program name
    match args.next().as_deref() {
        Some("skin") => {
            let Some(name) = args.next() else {
                println!("Missing name");
                return Ok(());
            };

            let skin = Skin::player(&name);
            skin.draw(&mut structure, SCALING)?;
        }
        Some("item") => {
            let Some(name) = args.next() else {
                println!("Missing item name");
                return Ok(());
            };
            let Some(item) = get_item(&name) else {
                println!("Unsupported item \"{name}\"");
                return Ok(());
            };

            item.draw(&mut structure, SCALING)?;
        }
        Some("mob") => {
            let Some(name) = args.next() else {
                println!("Missing mob name");
                return Ok(());
            };
            let Some(item) = get_mob(&name) else {
                println!("Unsupported mob \"{name}\"");
                return Ok(());
            };

            item.draw(&mut structure, SCALING)?;
        }
        Some(s) => {
            println!("Unsupported option \"{s}\"");
            return Ok(());
        }
        None => {
            println!("Missing option");
            return Ok(());
        }
    }

    let mut f = File::create("output.nbt")?;
    structure.normalize();
    structure.write_out(&mut f)?;

    Ok(())
}
