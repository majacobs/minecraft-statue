use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn serialize(&self, name: &str, data: &mut Vec<u8>) {
        data.push(self.id());

        let name_len = name.bytes().len() as u16;
        data.extend_from_slice(&name_len.to_be_bytes());
        data.extend(name.bytes());

        self.raw_serialize(data);
    }

    fn raw_serialize(&self, data: &mut Vec<u8>) {
        match self {
            Tag::End => {}
            Tag::Byte(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::Short(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::Int(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::Long(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::Float(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::Double(v) => {
                data.extend_from_slice(&v.to_be_bytes());
            }
            Tag::ByteArray(v) => {
                let len = v.len() as i32;
                data.extend_from_slice(&len.to_be_bytes());
                for i in v.iter() {
                    data.extend(i.to_be_bytes());
                }
            }
            Tag::String(v) => {
                let len = v.bytes().len() as u16;
                data.extend_from_slice(&len.to_be_bytes());
                data.extend(v.bytes());
            }
            Tag::List(v) => {
                data.push(v[0].id()); // Assumed to be homogenous
                let len = v.len() as i32;
                data.extend_from_slice(&len.to_be_bytes());
                for t in v.iter() {
                    t.raw_serialize(data);
                }
            }
            Tag::Compound(v) => {
                for (name, tag) in v.iter() {
                    tag.serialize(name, data);
                }
                data.push(Tag::End.id());
            }
            Tag::IntArray(v) => {
                let len = v.len() as i32;
                data.extend_from_slice(&len.to_be_bytes());
                for i in v.iter() {
                    data.extend(i.to_be_bytes());
                }
            }
            Tag::LongArray(v) => {
                let len = v.len() as i32;
                data.extend_from_slice(&len.to_be_bytes());
                for i in v.iter() {
                    data.extend(i.to_be_bytes());
                }
            }
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }
}

pub struct Structure {
    pub data_version: DataVersion,
    pub size: Coords,
    pub palette: Vec<Palette>,
    pub blocks: Vec<Block>,
}

impl Structure {
    pub fn new(data_version: DataVersion) -> Self {
        Self {
            data_version,
            size: Coords::new(0, 0, 0),
            palette: vec![],
            blocks: vec![],
        }
    }

    pub fn to_nbt(&self) -> Tag {
        let map = HashMap::from([
            ("DataVersion".into(), Tag::Int(self.data_version as i32)),
            ("size".into(), self.size.to_nbt()),
            (
                "palette".into(),
                Tag::List(self.palette.iter().map(Palette::to_nbt).collect()),
            ),
            (
                "blocks".into(),
                Tag::List(self.blocks.iter().map(Block::to_nbt).collect()),
            ),
        ]);
        Tag::Compound(map)
    }

    pub fn set(&mut self, pos: impl Into<Coords>, block_id: impl Into<Palette>) {
        self.set_block(pos.into(), block_id.into());
    }

    pub fn set_block(&mut self, pos: Coords, block_id: Palette) {
        let state = match self.palette.iter().position(|p| p == &block_id) {
            Some(index) => index as i32,
            None => {
                self.palette.push(block_id);
                (self.palette.len() - 1) as i32
            }
        };

        if let Some(block) = self.blocks.iter_mut().find(|b| b.pos == pos) {
            block.state = state;
        } else {
            self.blocks.push(Block { state, pos });
        }
    }

    pub fn normalize(&mut self) {
        if self.blocks.is_empty() {
            panic!("Empty structure");
        }

        let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
        let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
        let (mut min_z, mut max_z) = (i32::MAX, i32::MIN);
        for block in self.blocks.iter() {
            min_x = block.pos.x.min(min_x);
            max_x = block.pos.x.max(max_x);

            min_y = block.pos.y.min(min_y);
            max_y = block.pos.y.max(max_y);

            min_z = block.pos.z.min(min_z);
            max_z = block.pos.z.max(max_z);
        }

        self.size = Coords {
            x: max_x - min_x + 1,
            y: max_y - min_y + 1,
            z: max_z - min_z + 1,
        };

        for block in self.blocks.iter_mut() {
            block.pos.x += -min_x;
            block.pos.y += -min_y;
            block.pos.z += -min_z;
        }
    }

    pub fn write_out(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut data = vec![];
        self.to_nbt().serialize("", &mut data);
        let _ = w.write(data.as_slice())?;
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn to_nbt(&self) -> Tag {
        Tag::List(vec![Tag::Int(self.x), Tag::Int(self.y), Tag::Int(self.z)])
    }
}

impl From<(i32, i32, i32)> for Coords {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Palette {
    pub name: String,
    pub properties: HashMap<String, Tag>,
}

impl Palette {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, name: &str, value: impl ToString) -> Self {
        let entry = self.properties.entry(name.to_string()).or_insert(Tag::End);
        *entry = Tag::String(value.to_string());
        self
    }

    fn to_nbt(&self) -> Tag {
        let map = HashMap::from([
            ("Name".into(), Tag::String(self.name.to_owned())),
            ("Properties".into(), Tag::Compound(self.properties.clone())),
        ]);
        Tag::Compound(map)
    }
}

pub struct Block {
    pub state: i32,
    pub pos: Coords,
}

impl Block {
    pub fn to_nbt(&self) -> Tag {
        let map = HashMap::from([
            ("state".into(), Tag::Int(self.state)),
            ("pos".into(), self.pos.to_nbt()),
        ]);
        Tag::Compound(map)
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum DataVersion {
    /// Minecraft 1.20.1
    Minecraft1_20_1 = 3465,
    /// Minecraft 1.20.2
    Minecraft1_20_2 = 3578,
    /// Minecraft 1.20.3
    Minecraft1_20_3 = 3698,
    /// Minecraft 1.20.4
    Minecraft1_20_4 = 3700,
    /// Minecraft 1.20.5
    Minecraft1_20_5 = 3837,
    /// Minecraft 1.20.6
    Minecraft1_20_6 = 3839,
}
