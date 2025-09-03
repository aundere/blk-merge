/// Represents the possible values a property can have in a BLK configuration.
#[derive(Debug, PartialEq)]
pub enum BlkPropertyValue {
    Text(String),
    Boolean(bool),
    Integer(i32),
    Real(f32),
    Vector2(f32, f32),
    Vector3(f32, f32, f32),
    Vector4(f32, f32, f32, f32),
    Color(i32, i32, i32, i32)
}

/// Represents a property in a BLK configuration.
#[derive(Debug, PartialEq)]
pub struct BlkProperty {
    pub key: String,
    pub value: BlkPropertyValue
}

/// Represents a section in a BLK configuration.
#[derive(Debug, PartialEq)]
pub struct BlkSection {
    pub name: String,
    pub entries: Vec<BlkEntry>
}

/// Represents an entry in a BLK configuration, which can be either a section or a property.
#[derive(Debug, PartialEq)]
pub enum BlkEntry {
    Section(BlkSection),
    Property(BlkProperty)
}

/// Represents a block in a BLK configuration.
#[derive(Debug, PartialEq)]
pub struct BlkBlock {
    pub entries: Vec<BlkEntry>
}

/// Represents a BLK configuration, which consists of multiple entries.
#[derive(Debug, PartialEq)]
pub struct BlkConfig {
    pub block: BlkBlock
}
