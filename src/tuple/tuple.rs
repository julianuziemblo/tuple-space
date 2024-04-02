use std::ops::{Index, IndexMut};

use crate::util::{DisplayBinary, Serializable};

const TUPLE_NAME_MAX_SIZE: usize = 31;

const TUPLE_TYPE_UNDEFINED: u8 = 0b000;
const TUPLE_TYPE_INT: u8 = 0b001;
const TUPLE_TYPE_FLOAT: u8 = 0b010;

const TUPLE_FIELD_OCCUPIED_YES: u8 = 0b1;
const TUPLE_FIELD_OCCUPIED_NO: u8 = 0b0;

const TUPLE_FIELD_OCCUPIED_SHIFT: usize = 7;
const TUPLE_FIELD_TYPE_SHIFT: usize = 4;

impl DisplayBinary for [u8] {
    fn display_bin(&self) -> Vec<String> {
        self.iter().map(|&e| format!("{e:08b}")).collect()
    }
}

impl DisplayBinary for Vec<u8> {
    fn display_bin(&self) -> Vec<String> {
        self.iter().map(|&e| format!("{e:08b}")).collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TupleField {
    Int(Option<i32>),
    Float(Option<f32>),
    Undefined,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tuple {
    name: String,
    fields: Vec<TupleField>,
}

impl Tuple {
    pub fn new(name: &str, size: usize) -> Self {
        Tuple {
            name: name.to_string(),
            fields: vec![TupleField::Undefined; size],
        }
    }

    pub fn get(&self, index: usize) -> Option<TupleField> {
        if index < self.fields.len() {
            return Some(self[index]);
        }
        None
    }

    pub fn insert(&mut self, index: usize, data: TupleField) {
        self[index] = data;
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

impl Index<usize> for Tuple {
    type Output = TupleField;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}

#[derive(Clone, Debug)]
pub enum TupleParseError {
    InvalidFormat,
    NameError,
    UnsupportedTypename,
    ValueParseError,
}

impl Tuple {
    /*
     *  Return a [tuple_template] created from a given tuple_string.
     *  tuple_string: a string with format: `("[name]", [type] [value]/?, ...)`
     *  Example: `("test", int 123, float ?)`
     */
    pub fn from_str(s: &str) -> Result<Self, TupleParseError> {
        if s.is_empty() {
            return Err(TupleParseError::NameError);
        }
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(TupleParseError::InvalidFormat);
        }

        let s = &s[1..s.len() - 1];
        let mut tokens = s.split(',');
        let name = tokens.next().ok_or(TupleParseError::NameError)?;
        let name = &name[1..name.len() - 1];
        let mut fields = vec![];

        // println!(
        //     "[FROM_STR] tokens vec: {:?}",
        //     tokens.clone().collect::<Vec<_>>()
        // );

        for token in tokens {
            let token = token.trim();
            // println!("[FROM_STR] token: {token}");
            fields.push(match token {
                "undefined" | "undef" | "UNDEFINED" | "UNDEF" | "?" => TupleField::Undefined,
                _ => {
                    let mid = token.find(' ').ok_or(TupleParseError::InvalidFormat)?;
                    let (typename, str_value) = token.split_at(mid);
                    let (typename, str_value) = (typename.trim(), str_value.trim());
                    // println!("typename: {:?}, str_value: {:?}", typename, str_value);

                    match typename {
                        "int" | "INT" => TupleField::Int(match str_value.trim() {
                            "?" => None,
                            _ => Some(match str_value.parse() {
                                Ok(v) => v,
                                Err(_) => return Err(TupleParseError::ValueParseError),
                            }),
                        }),
                        "float" | "FLOAT" => TupleField::Float(match str_value.trim() {
                            "?" => None,
                            _ => Some(match str_value.parse() {
                                Ok(v) => v,
                                Err(_) => return Err(TupleParseError::ValueParseError),
                            }),
                        }),
                        _ => return Err(TupleParseError::UnsupportedTypename),
                    }
                }
            });
        }

        Ok(Tuple {
            name: name.to_string(),
            fields,
        })
    }

    /*
     * Determines if a tuple matches another tuple (prefferably: a template one).
     */
    pub fn matches(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        if self.name != other.name {
            return false;
        }

        for (f1, f2) in self.fields.iter().zip(other.fields.iter()) {
            match (f1, f2) {
                (TupleField::Int(opt_f1), TupleField::Int(opt_f2)) => {
                    if !(opt_f1.is_some() || opt_f2.is_some()) {
                        return false;
                    }
                    if opt_f1.is_some() && opt_f2.is_some() && opt_f1.unwrap() != opt_f2.unwrap() {
                        return false;
                    }
                }

                (TupleField::Float(opt_f1), TupleField::Float(opt_f2)) => {
                    if !(opt_f1.is_some() || opt_f2.is_some()) {
                        return false;
                    }
                    if opt_f1.is_some() && opt_f2.is_some() && opt_f1.unwrap() != opt_f2.unwrap() {
                        return false;
                    }
                }
                _ => return false,
            };
        }

        true
    }
}

impl Serializable for Tuple {
    type Error = TupleParseError;
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];

        // name
        res.extend(self.name.as_bytes());
        res.push(b'\0');

        // size
        res.extend((self.len() as u32).to_be_bytes());

        // fields
        for field in self.fields.iter() {
            use TupleField as TF;
            let mut field_bytes = vec![];
            match field {
                TF::Int(val) => match val {
                    Some(v) => {
                        field_bytes.push(
                            (TUPLE_FIELD_OCCUPIED_YES << TUPLE_FIELD_OCCUPIED_SHIFT)
                                | (TUPLE_TYPE_INT << TUPLE_FIELD_TYPE_SHIFT),
                        );
                        field_bytes.extend(v.to_be_bytes());
                    }
                    None => field_bytes.push(
                        (TUPLE_FIELD_OCCUPIED_NO << TUPLE_FIELD_OCCUPIED_SHIFT)
                            | (TUPLE_TYPE_INT << TUPLE_FIELD_TYPE_SHIFT),
                    ),
                },
                TF::Float(val) => match val {
                    Some(v) => {
                        field_bytes.push(
                            (TUPLE_FIELD_OCCUPIED_YES << TUPLE_FIELD_OCCUPIED_SHIFT)
                                | (TUPLE_TYPE_FLOAT << TUPLE_FIELD_TYPE_SHIFT),
                        );
                        field_bytes.extend(v.to_be_bytes())
                    }
                    None => field_bytes.push(
                        (TUPLE_FIELD_OCCUPIED_NO << TUPLE_FIELD_OCCUPIED_SHIFT)
                            | (TUPLE_TYPE_FLOAT << TUPLE_FIELD_TYPE_SHIFT),
                    ),
                },
                TF::Undefined => field_bytes.push(
                    (TUPLE_FIELD_OCCUPIED_NO << TUPLE_FIELD_OCCUPIED_SHIFT)
                        | (TUPLE_TYPE_UNDEFINED << TUPLE_FIELD_TYPE_SHIFT),
                ),
            };
            res.extend(field_bytes);
        }

        res
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut name_accum = String::new();
        let mut bytes = bytes.iter();
        let mut name_is_valid = false;

        // name
        for (i, &byte) in bytes.by_ref().enumerate() {
            if i > TUPLE_NAME_MAX_SIZE {
                return Err(TupleParseError::NameError);
            }

            if byte == b'\0' {
                name_is_valid = true;
                break;
            }

            name_accum.push(byte as char);
        }

        if !name_is_valid {
            return Err(TupleParseError::NameError);
        }

        let name = match name_accum.parse::<String>() {
            Ok(v) => v,
            Err(_) => return Err(TupleParseError::NameError),
        };

        // size
        let mut size = 0u32;
        let mut i = 0;
        for &byte in bytes.by_ref() {
            size += (byte as u32) << (std::mem::size_of::<u32>() - 1 - i);
            i += 1;
            if i >= 4 {
                break;
            }
        }
        // println!("size is {size}");

        // fields
        let mut fields = Vec::<TupleField>::with_capacity(size as usize);
        while let Some(byte) = bytes.next() {
            // println!("Starting at byte: {byte:08b}");
            if (byte & (1 << TUPLE_FIELD_OCCUPIED_SHIFT)) > 0 {
                let mut num_accum = [0; 4];
                let mut i = 0;

                '_inner: for &byte1 in bytes.by_ref() {
                    num_accum[i] = byte1;
                    i += 1;
                    if i >= 4 {
                        break '_inner;
                    }
                }
                // println!("num_accum: {:?}", num_accum.display_bin());
                // println!("num as i32: {}", i32::from_be_bytes(num_accum));
                // println!("num as f32: {}", f32::from_be_bytes(num_accum));
                // println!(
                //     "byte & mask = {:08b}",
                //     (byte & (0b111 << TUPLE_FIELD_TYPE_SHIFT)) >> TUPLE_FIELD_TYPE_SHIFT
                // );
                fields.push(
                    match (byte & (0b111 << TUPLE_FIELD_TYPE_SHIFT)) >> TUPLE_FIELD_TYPE_SHIFT {
                        TUPLE_TYPE_INT => TupleField::Int(Some(i32::from_be_bytes(num_accum))),
                        TUPLE_TYPE_FLOAT => TupleField::Float(Some(f32::from_be_bytes(num_accum))),
                        _ => return Err(TupleParseError::InvalidFormat),
                    },
                );
            } else {
                // println!("Field was clear: {:08b}", byte);
                fields.push(
                    match (byte & (0b111 << TUPLE_FIELD_TYPE_SHIFT)) >> TUPLE_FIELD_TYPE_SHIFT {
                        TUPLE_TYPE_INT => TupleField::Int(None),
                        TUPLE_TYPE_FLOAT => TupleField::Float(None),
                        TUPLE_TYPE_UNDEFINED => TupleField::Undefined,
                        _ => return Err(TupleParseError::InvalidFormat),
                    },
                );
            }
        }

        Ok(Self { name, fields })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tuple::tuple::{Tuple, TupleField},
        util::Serializable,
    };

    #[test]
    fn tuple_creation_test() {
        let mut tuple_manual = Tuple::new("t1", 2);
        tuple_manual.insert(0, TupleField::Float(Some(6.276)));
        tuple_manual.insert(1, TupleField::Int(None));

        let tuple_auto = Tuple::from_str("('t1', float 6.276, int ?)").unwrap();

        assert_eq!(tuple_manual, tuple_auto);
    }

    #[test]
    fn tuple_serialization_test() {
        let t1 = Tuple::from_str("('t1', float 6.276, int ?)").unwrap();
        let t1_bytes = t1.serialize();
        let t1_from_bytes = Tuple::deserialize(&t1_bytes).unwrap();

        assert_eq!(t1, t1_from_bytes)
    }

    #[test]
    fn tuple_from_str_natalia_test() {
        let t1 = Tuple::from_str("('japierdole', INT 69, FLOAT 21.37, INT ?)");

        // println!("t1={t1:?}");

        assert!(t1.is_ok())
    }

    #[test]
    fn tuple_match_test() {
        let tuple = Tuple::from_str("('t1', int 123, float 213.7)").unwrap();
        let tuple_template = Tuple::from_str("('t1', int ?, float 213.7)").unwrap();

        assert!(tuple.matches(&tuple_template))
    }

    #[test]
    fn tuple_not_matches_test() {
        let tuple = Tuple::from_str("('t1', int 123, float 213.7)").unwrap();
        let tuple_template = Tuple::from_str("('t1', float 213.7, int 123)").unwrap();

        assert!(! tuple.matches(&tuple_template))
    }
}
