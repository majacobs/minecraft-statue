use crate::items::get_item;
use crate::model::{JsonModel, Model};
use crate::nbt::{DataVersion, Structure};
use std::fs::File;

mod drawing;
mod items;
mod materials;
mod model;
mod nbt;
mod transform;

fn main() -> std::io::Result<()> {
    let mut structure = Structure::new(DataVersion::Minecraft1_20_1);

    let file = File::open("models.json")?;
    let models: Vec<JsonModel> = serde_json::from_reader(file)?;

    let Some(args) = Args::parse() else {
        println!("Invalid arguments");
        return Ok(());
    };

    match args.what {
        What::Player {
            player,
            alternatives,
        } => {
            let Some(mut model) = models.into_iter().find(|m| m.name == "player") else {
                println!("Player model not found");
                return Ok(());
            };
            model.texture = format!("{player}.png");
            for (part, alt) in alternatives.into_iter() {
                model.use_alternate(part, alt);
            }
            model.draw(&mut structure, args.model_scale)?;
        }
        What::Item { name } => {
            let Some(item) = get_item(&name) else {
                println!("Unsupported item \"{name}\"");
                return Ok(());
            };

            item.draw(&mut structure, args.model_scale)?;
        }
        What::Mob { name } => {
            let Some(model) = models.into_iter().find(|m| m.name == name) else {
                println!("Unsupported mob \"{name}\"");
                return Ok(());
            };
            model.draw(&mut structure, args.model_scale)?;
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
        let mut args = std::env::args().peekable();
        args.next(); // Skip program name

        let mut x = Args {
            model_scale: 1,
            texture_scale: 1,
            what: match args.next()?.as_str() {
                "player" => {
                    let player = args.next()?;
                    let mut alternatives = vec![];
                    while args.peek().map_or(false, |a| !a.starts_with("--")) {
                        let arg = args.next().unwrap();
                        let mut parts = arg.splitn(2, ':');
                        let part_name = parts.next()?.to_string();
                        let alt_name = parts.next()?.to_string();
                        alternatives.push((part_name, alt_name));
                    }
                    What::Player {
                        player,
                        alternatives,
                    }
                }
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
    Player {
        player: String,
        alternatives: Vec<(String, String)>,
    },
    Item {
        name: String,
    },
    Mob {
        name: String,
    },
}
