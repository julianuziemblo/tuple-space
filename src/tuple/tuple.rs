use core::num;
use std::ops::{Index, IndexMut};

const UNDEFINED: u8 = 0b00;
const INT: u8 = 0b01;
const FLOAT: u8 = 0b10;

const TUPLE_FIELD_OCCUPIED_YES: u8 = 0b1;
const TUPLE_FIELD_OCCUPIED_NO: u8 = 0b0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TupleField {
    Int(Option<i32>),
    Float(Option<f32>),
    Undefined,
}

#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    name: &'a str,
    fields: Vec<TupleField>,
}

impl<'a> Tuple<'a> {
    pub fn new(name: &'a str, size: usize) -> Self {
        Tuple {
            name,
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

impl<'a> Index<usize> for Tuple<'a> {
    type Output = TupleField;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl<'a> IndexMut<usize> for Tuple<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}

#[derive(Clone, Debug)]
pub enum TupleStringError {
    FormatError,
    NameLengthError,
    TupleLengthError,
    TypenameError,
}

impl<'a> Tuple<'a> {
    /*
     *  Return a [tuple_template] created from a given tuple_string.
     *  tuple_string: a string with format: `("[name]", [type] [value]/?, ...)`
     *  Example: `("test", int 123, float ?)`
     */
    pub fn from_str(s: &'a str) -> Result<Self, TupleStringError> {
        if s.is_empty() {
            return Err(TupleStringError::NameLengthError);
        }
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(TupleStringError::FormatError);
        }

        let s = &s[1..s.len() - 1];
        let mut tokens = s.split(',');
        let name = tokens.next().ok_or(TupleStringError::NameLengthError)?;
        let name = &name[1..name.len() - 1];
        let mut fields = vec![];

        for token in tokens {
            fields.push(match token {
                "undefined" | "?" => TupleField::Undefined,
                _ => {
                    println!("token: {:?}", token);
                    let token = token.trim();
                    let mid = token.find(' ').ok_or(TupleStringError::FormatError)?;
                    let (typename, str_value) = token.split_at(mid);
                    println!("typename: {:?}, str_value: {:?}", typename, str_value);

                    match typename {
                        "int" => TupleField::Int(match str_value.trim() {
                            "?" => None,
                            _ => Some(match str_value.parse() {
                                Ok(v) => v,
                                Err(_) => return Err(TupleStringError::FormatError),
                            }),
                        }),
                        "float" => TupleField::Float(match str_value.trim() {
                            "?" => None,
                            _ => Some(match str_value.parse() {
                                Ok(v) => v,
                                Err(_) => return Err(TupleStringError::FormatError),
                            }),
                        }),
                        _ => return Err(TupleStringError::TypenameError),
                    }
                }
            });
        }

        Ok(Tuple { name, fields })
    }

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        res.extend(self.name.as_bytes());

        for field in self.fields.iter() {
            // TODO: convert fields into byte representations
            use TupleField as TF;
            let mut bytes = vec![];
            match field {
                TF::Int(val) => {
                    bytes.push(INT);
                    match val {
                        Some(v) => {bytes.push(TUPLE_FIELD_OCCUPIED_YES); bytes.extend(v.to_ne_bytes())},
                        None => bytes.push(TUPLE_FIELD_OCCUPIED_NO),
                    }; 
                },
                TF::Float(val) => {
                    bytes.push(FLOAT);
                    match val {
                        Some(v) => {bytes.push(TUPLE_FIELD_OCCUPIED_YES); bytes.extend(v.to_ne_bytes())},
                        None => bytes.push(TUPLE_FIELD_OCCUPIED_NO),
                    }; 
                },
                TF::Undefined => bytes.push(TUPLE_FIELD_OCCUPIED_NO),
            };
            res.extend(bytes);
        }

        res
    }
}
