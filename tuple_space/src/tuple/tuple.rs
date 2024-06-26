use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

use crate::tuple::consts::*;
use crate::util::Serializable;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TupleField {
    Int(Option<i32>),
    Float(Option<f32>),
    Undefined,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Tuple {
    pub name: String,
    pub fields: Vec<TupleField>,
}

impl Tuple {
    pub fn new(name: &str) -> Self {
        Tuple {
            name: name.to_string(),
            fields: vec![],
        }
    }

    pub fn with_capacity(name: &str, size: u8) -> Self {
        Tuple {
            name: name.to_string(),
            fields: vec![TupleField::Undefined; size as usize],
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

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
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

impl std::str::FromStr for Tuple {
    type Err = TupleParseError;

    /*
     *  Return a [tuple_template] created from a given tuple_string.
     *  tuple_string: a string with format: `("[name]", [type] [value]/?, ...)`
     *  Example: `("test", int 123, float ?)`
     */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                        _ => return Err(TupleParseError::UnsupportedType),
                    }
                }
            });
        }

        Ok(Tuple {
            name: name.to_string(),
            fields,
        })
    }
}

impl Tuple {
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

    /// Compares two tuples by their binary representations
    pub fn cmp_binary(&self, other: &Tuple) -> Ordering {
        let (tuple1, tuple2) = (self.serialize(), other.serialize());
        Tuple::cmp_serialized(&tuple1, &tuple2)
    }

    /// Compares two binary representations of tuples (serialized tuples)
    #[allow(clippy::comparison_chain)]
    pub fn cmp_serialized(tuple1: &[u8], tuple2: &[u8]) -> Ordering {
        for (&t1, &t2) in tuple1.iter().zip(tuple2.iter()) {
            if t1 > t2 {
                return Ordering::Greater;
            } else if t1 < t2 {
                return Ordering::Less;
            }
        }

        if tuple1.len() > tuple2.len() {
            Ordering::Greater
        } else if tuple1.len() < tuple2.len() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
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

        let name = match name_accum.parse() {
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

#[derive(Clone, Copy, Debug)]
pub enum TupleParseError {
    InvalidFormat,
    NameError,
    UnsupportedType,
    ValueParseError,
}

impl std::fmt::Display for TupleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TupleParseError::{}",
            match self {
                TupleParseError::InvalidFormat => "InvalidFormat: The provided tuple representation has invalid format.".to_string(),
                TupleParseError::NameError => format!("NameError: The provided tuple representation has invalid name or invalid name length. Max name length: {}", TUPLE_NAME_MAX_SIZE),
                TupleParseError::UnsupportedType => "UnsupportedTypename: The provided tuple representation has a field of unsupported type.".to_string(),
                TupleParseError::ValueParseError => "ValueParseError: Error while parsing one of the provided tuple representation fields' value.".to_string(),
            }
        )
    }
}

impl std::error::Error for TupleParseError {}

#[derive(Clone, Debug, Default)]
pub struct TupleBuilder {
    tuple: Tuple,
}

impl TupleBuilder {
    pub fn new() -> Self {
        TupleBuilder {
            tuple: Default::default(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.tuple.name = name.to_owned();
        self
    }

    pub fn field(mut self, field: TupleField) -> Self {
        self.tuple.fields.push(field);
        self
    }

    pub fn build(self) -> Tuple {
        self.tuple
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tuple::tuple::{Tuple, TupleField},
        util::Serializable,
    };
    use std::str::FromStr;

    #[test]
    fn tuple_creation_test() {
        let mut tuple_manual = Tuple::new("t1");
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

        assert!(!tuple.matches(&tuple_template))
    }

    #[test]
    fn mem_layout_test() {
        let ser1 = Tuple::default().serialize();
        let ser2 = Tuple::new("").serialize();
        // println!("tuple default serialized: {:b}", SliceU8(&ser1));
        // println!("tuple t1 serialized: {:b}", SliceU8(&ser2));
        assert_eq!(ser1, ser2)
    }
}
