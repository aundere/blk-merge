use std::io::Write;

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

/// Ugly function to convert a BLK configuration into a string representation.
pub fn stringify_config(config: &BlkConfig, writer: &mut dyn Write) -> Result<(), std::io::Error> {
    fn stringify_config_inner(writer: &mut dyn Write, entry: &BlkEntry, recurse_step: i32) -> Result<(), std::io::Error> {
        write!(writer, "{}", &"    ".repeat(recurse_step as usize))?;

        match entry {
            BlkEntry::Section(section) => {
                write!(writer, "{}{{\n", section.name)?;

                for entry in &section.entries {
                    stringify_config_inner(writer, entry, recurse_step + 1)?;
                }

                write!(writer, "{}}}\n", &"    ".repeat(recurse_step as usize))?;
            },
            BlkEntry::Property(property) => {
                write!(writer, "{}", property.key)?;

                match &property.value {
                    BlkPropertyValue::Text(text) => {
                        write!(writer, ":t=\"{}\"", text)?;
                    },
                    BlkPropertyValue::Boolean(boolean) => {
                        write!(writer, ":b={}", if *boolean { "yes" } else { "no" })?;
                    },
                    BlkPropertyValue::Integer(integer) => {
                        write!(writer, ":i={}", integer)?;
                    },
                    BlkPropertyValue::Real(real) => {
                        write!(writer, ":r={}", real)?;
                    },
                    BlkPropertyValue::Vector2(x, y) => {
                        write!(writer, ":p2={}, {}", x, y)?;
                    },
                    BlkPropertyValue::Vector3(x, y, z) => {
                        write!(writer, ":p3={}, {}, {}", x, y, z)?;
                    },
                    BlkPropertyValue::Vector4(x, y, z, w) => {
                        write!(writer, ":p4={}, {}, {}, {}", x, y, z, w)?;
                    },
                    BlkPropertyValue::Color(r, g, b, a) => {
                        write!(writer, ":c={}, {}, {}, {}", r, g, b, a)?;
                    }
                }

                write!(writer, "\n")?;
            }
        }

        Ok(())
    }

    for entry in &config.block.entries {
        stringify_config_inner(writer, entry, 0)?;
    }

    Ok(())
}
