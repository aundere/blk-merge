use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{alpha1, char, digit1, multispace0}, combinator::recognize, multi::{many0, many1}, sequence::{delimited, terminated}, IResult, Parser};
use crate::types::*;

/// Represents the different types of BLK properties.
enum BlkType { Text, Boolean, Integer, Real, Point2, Point3, Point4, Color }

/// Parses a BLK type identifier from the input string.
fn parse_blk_type(input: &str) -> IResult<&str, BlkType> {
    alt((
        tag("t").map(|_| BlkType::Text),
        tag("b").map(|_| BlkType::Boolean),
        tag("i").map(|_| BlkType::Integer),
        tag("r").map(|_| BlkType::Real),
        tag("p2").map(|_| BlkType::Point2),
        tag("p3").map(|_| BlkType::Point3),
        tag("p4").map(|_| BlkType::Point4),
        tag("c").map(|_| BlkType::Color)
    )).parse(input)
}

/// Parses a BLK property value based on its type.
fn parse_property_value(ty: BlkType) -> impl Fn(&str) -> IResult<&str, BlkPropertyValue> {
    move |input: &str| {
        match ty {
            BlkType::Text => parse_string
                .map(|text| BlkPropertyValue::Text(text.to_string()))
                .parse(input),
            BlkType::Boolean => parse_boolean
                .map(BlkPropertyValue::Boolean)
                .parse(input),
            BlkType::Integer => parse_integer
                .map(BlkPropertyValue::Integer)
                .parse(input),
            BlkType::Real => parse_real
                .map(BlkPropertyValue::Real)
                .parse(input),
            BlkType::Point2 => {
                let (rest, (x, _, y)) =
                    (parse_real, parse_vector_delimiter, parse_real).parse(input)?;
                Ok((rest, BlkPropertyValue::Vector2(x, y)))
            }
            BlkType::Point3 => {
                let (rest, (x, y, z)) =
                    (terminated(parse_real, parse_vector_delimiter), terminated(parse_real, parse_vector_delimiter), parse_real).parse(input)?;
                Ok((rest, BlkPropertyValue::Vector3(x, y, z)))
            }
            BlkType::Point4 => {
                let (rest, (x, y, z, w)) = (
                    terminated(parse_real, parse_vector_delimiter),
                    terminated(parse_real, parse_vector_delimiter),
                    terminated(parse_real, parse_vector_delimiter),
                    parse_real
                ).parse(input)?;
                Ok((rest, BlkPropertyValue::Vector4(x, y, z, w)))
            }
            BlkType::Color => {
                let (rest, (r, g, b, a)) = (
                    terminated(parse_integer, parse_vector_delimiter),
                    terminated(parse_integer, parse_vector_delimiter),
                    terminated(parse_integer, parse_vector_delimiter),
                    parse_integer
                ).parse(input)?;
                Ok((rest, BlkPropertyValue::Color(r, g, b, a)))
            }
        }
    }
}

/// Parses a newline character, supporting both Unix and Windows formats.
fn newline_multiplatform(input: &str) -> IResult<&str, ()> {
    alt((tag("\r\n"), tag("\n"))).map(|_| ()).parse(input)
}

/// Parses an identifier from the input string.
fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, digit1, tag("_"))))).parse(input)
}

/// Parses a line separator, which can be either a newline or a semicolon.
fn parse_separator(input: &str) -> IResult<&str, ()> {
    many1(alt((newline_multiplatform, char(';').map(|_| ())))).map(|_| ()).parse(input)
}

/// Parses a boolean value from the input string.
fn parse_boolean(input: &str) -> IResult<&str, bool> {
    alt((
        alt((tag("true"), tag("yes"))).map(|_| true),
        alt((tag("false"), tag("no"))).map(|_| false)
    )).parse(input)
}

/// Parses an integer value from the input string.
fn parse_integer(input: &str) -> IResult<&str, i32> {
    nom::character::complete::i32(input)
}

/// Parses a real (floating-point) value from the input string.
fn parse_real(input: &str) -> IResult<&str, f32> {
    nom::number::complete::float(input)
}

/// Parses a vector delimiter (comma followed by optional whitespace) from the input string.
fn parse_vector_delimiter(input: &str) -> IResult<&str, ()> {
    (char(','), multispace0).map(|_| ()).parse(input)
}

/// Parses a string value enclosed in double quotes from the input string.
fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

/// Parses a BLK property from the input string.
fn parse_property(input: &str) -> IResult<&str, BlkEntry> {
    let (remaining, (identifier, ty)) = (parse_identifier, delimited(char(':'), parse_blk_type, char('='))).parse(input)?;
    let (remaining, value) = parse_property_value(ty).parse(remaining)?;

    Ok((remaining, BlkEntry::Property(BlkProperty { key: identifier.to_string(), value })))
}

/// Parses a BLK section from the input string.
fn parse_section(input: &str) -> IResult<&str, BlkEntry> {
    (parse_identifier, delimited(char('{'), parse_block, char('}')))
        .map(|(name, block)| BlkEntry::Section(BlkSection { name: name.to_string(), entries: block.entries }))
        .parse(input)
}

/// Parses a single entry in a BLK configuration, which can be either a section or a property.
fn parse_entry(input: &str) -> IResult<&str, BlkEntry> {
    delimited(multispace0, alt((parse_section, parse_property)), parse_separator).parse(input)
}

/// Parses a block of entries in a BLK configuration.
fn parse_block(input: &str) -> IResult<&str, BlkBlock> {
    terminated(many0(parse_entry), multispace0).map(|entries| BlkBlock { entries }).parse(input)
}

/// Parses a BLK configuration from the input string.
pub fn parse_config(input: &str) -> IResult<&str, BlkConfig> {
    parse_block.map(|block| BlkConfig { block }).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_config() {
        let input = "";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert!(config.block.entries.is_empty());
    }

    #[test]
    fn test_parse_integer() {
        let input = "age:i=30;";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert!(config.block.entries.len() == 1);

        if let BlkEntry::Property(prop) = &config.block.entries[0] {
            assert_eq!(prop.key, "age");
            assert_eq!(prop.value, BlkPropertyValue::Integer(30));
        } else {
            panic!("Expected a property entry");
        }
    }

    #[test]
    fn test_parse_with_crlf() {
        let input = "meow:t=\"uwu\";\r\nuwu{owo:i=32;};";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert_eq!(config, BlkConfig {
            block: BlkBlock {
                entries: vec![
                    BlkEntry::Property(BlkProperty {
                        key: "meow".to_string(),
                        value: BlkPropertyValue::Text("uwu".to_string())
                    }),
                    BlkEntry::Section(BlkSection {
                        name: "uwu".to_string(),
                        entries: vec![
                            BlkEntry::Property(BlkProperty {
                                key: "owo".to_string(),
                                value: BlkPropertyValue::Integer(32)
                            })
                        ]
                    })
                ]
            }
        })
    }

    #[test]
    fn test_parse_sections() {
        let input = "meow:t=\"uwu\";uwu{owo:i=32;};";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert_eq!(config, BlkConfig {
            block: BlkBlock {
                entries: vec![
                    BlkEntry::Property(BlkProperty {
                        key: "meow".to_string(),
                        value: BlkPropertyValue::Text("uwu".to_string())
                    }),
                    BlkEntry::Section(BlkSection {
                        name: "uwu".to_string(),
                        entries: vec![
                            BlkEntry::Property(BlkProperty {
                                key: "owo".to_string(),
                                value: BlkPropertyValue::Integer(32)
                            })
                        ]
                    })
                ]
            }
        })
    }

    #[test]
    fn test_parse_config_with_whitespaces() {
        let input = "    wuu:i=23;    uuw:t=\"UwU\";    ";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert_eq!(config.block.entries.len(), 2);
    }

    #[test]
    fn test_parse_sections_with_whitespace() {
        let input = "   meow{  uwu:i=1;      owo:i=5;   };";
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert_eq!(config.block.entries.len(), 1);
    }

    #[test]
    fn test_parse_multiline_config() {
        let input = r#"
            input{
                owo:i=32
                uwu:t="uwu"

                output{
                    someText:t="OwO"
                }
            }
        "#;
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");
        assert_eq!(config, BlkConfig {
            block: BlkBlock { entries: vec![
                BlkEntry::Section(BlkSection {
                    name: "input".to_string(),
                    entries: vec![
                        BlkEntry::Property(BlkProperty {
                            key: "owo".to_string(),
                            value: BlkPropertyValue::Integer(32)
                        }),
                        BlkEntry::Property(BlkProperty {
                            key: "uwu".to_string(),
                            value: BlkPropertyValue::Text("uwu".to_string())
                        }),
                        BlkEntry::Section(BlkSection {
                            name: "output".to_string(),
                            entries: vec![
                                BlkEntry::Property(BlkProperty {
                                    key: "someText".to_string(),
                                    value: BlkPropertyValue::Text("OwO".to_string())
                                })
                            ]
                        })
                    ]
                })
            ] }
        })
    }

    #[test]
    pub fn test_parse_example_1() {
        let input = r#"
            cloudsQuality:t="medium"
            use_gamepad_cursor_control:b=no
            use_gamepad_interface:b=no
            hdClient:b=no
            clientType:t="32bit"

            graphics{
              enableSuspensionAnimation:b=no
              rendinstDistMul:r=0.5
              grassRadiusMul:r=0.1
              shadowQuality:t="ultralow"
              tireTracksQuality:i=0
              skyQuality:i=2
              cloudsQuality:i=2
            }
        "#;
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");

        // asserting full structure is too cumbersome here so just check key parts
        assert_eq!(config.block.entries.len(), 6); // 1 section + 5 properties
    }

    #[test]
    pub fn test_parse_example_2() {
        let input = r#"
            drawLines{
                line{ line:p4=0.35, -1, 0.35, 0; move:b=no; }
                line{ line:p4=115, +10000, 117, 0; move:b=no; thousandth:b=yes; }
            }
        "#;
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");

        // asserting full structure is too cumbersome here so just check key parts
        assert_eq!(config.block.entries.len(), 1); // 1 section
    }

    #[test]
    pub fn test_parse_example_3() {
        let input = r#"
            controls{
                version:i=200

                basePresetPaths{
                    default:t="base/preset/path.blk"
                }

                hotkeys{
                    ID_AAM{
                      mouseButton:i=1
                    }

                    ID_ACTION_BAR_ITEM_1{
                        keyboardKey:i=1
                    }
                }

                axes{
                    ailerons{
                        mouseAxisId:i=0
                    }
                }

                params{
                    trackIrZoom:b=no
                }

                deviceMapping{
                    joystick{
                        connected:b=no
                        devId:t="1234:ABCD"
                        axesOffset:i=0
                    }
                }
            }

            settings{
                aileronsMultiplier:r=0.9
            }
        "#;
        let result = parse_config(input);

        assert!(result.is_ok());

        let (remaining, config) = result.unwrap();

        assert_eq!(remaining, "");

        // asserting full structure is too cumbersome here so just check key parts
        assert_eq!(config.block.entries.len(), 2); // 2 sections
    }
}
