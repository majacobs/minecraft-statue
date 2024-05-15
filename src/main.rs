use crate::draw::Draw;
use crate::items::get_item;
use crate::model::JsonModel;
use crate::nbt::{DataVersion, Structure};
use image::io::Reader as ImageReader;
use image::ImageError;
use std::fs::File;

mod draw;
mod drawing;
mod items;
mod materials;
mod model;
mod nbt;
mod transform;

const SKIN_DIR: &str = ".";
const TEXTURE_DIR: &str = "minecraft/1.20.1/assets/minecraft/textures";

fn main() -> Result<(), Error> {
    let mut structure = Structure::new(DataVersion::Minecraft1_20_1);

    let file = File::open("models.json")?;
    let models: Vec<JsonModel> = serde_json::from_reader(file)?;

    let Some(args) = Args::parse() else {
        println!("Invalid arguments");
        return Ok(());
    };

    let (drawable, texture): (Box<dyn Draw>, String) = match args.what {
        What::Player {
            player,
            alternatives,
        } => {
            let Some(mut model) = models.into_iter().find(|m| m.name == "player") else {
                println!("Player model not found");
                return Ok(());
            };

            for (part, alt) in alternatives.into_iter() {
                model.use_alternate(part, alt);
            }

            let texture = format!("{}/{}.png", SKIN_DIR, player);
            (Box::new(model), texture)
        }
        What::Item { name } => {
            let Some(item) = get_item(&name) else {
                println!("Unsupported item \"{name}\"");
                return Ok(());
            };

            let texture = format!("{}/{}", TEXTURE_DIR, item.texture);
            (Box::new(item), texture)
        }
        What::Mob { name } => {
            let Some(model) = models.into_iter().find(|m| m.name == name) else {
                println!("Unsupported mob \"{name}\"");
                return Ok(());
            };

            let texture = format!("{}/{}", TEXTURE_DIR, model.texture);
            (Box::new(model), texture)
        }
    };

    let image = ImageReader::open(texture)?.decode()?;
    let image = image.as_rgba8().ok_or(Error::NotRgba8)?;

    drawable.draw(&mut structure, args.model_scale, image);

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

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    Image(ImageError),
    Io(std::io::Error),
    Serde(serde_json::Error),
    NotRgba8,
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
