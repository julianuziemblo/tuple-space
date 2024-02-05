use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub enum TupleField {
    Int(Option<i32>),
    Float(Option<f32>),
}

#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    pub name: &'a str,
    pub fields: Vec<TupleField>,
}

impl<'a> Tuple<'a> {
    pub fn new(name: &'a str, size: usize) -> Self {
        Tuple {
            name,
            fields: Vec::with_capacity(size)
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

pub enum TupleStringError {
    InvalidFormatError,
    InvalidNameLength,
    InvalidType(usize),
}

/*
 *  Return a tuple_template created from a given tuple_string.
 *  tuple_string: a string with format: `("[name]", [type] [value]/?, ...)`
 *  Example: `("test", int 123, float ?)` 
*/
impl<'a> Tuple<'a> {
    fn from_str(s: &'a str) -> Result<Self, TupleStringError> {
        if s.len() == 0 {
            return Err(TupleStringError::InvalidNameLength);
        }
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(TupleStringError::InvalidFormatError);
        }
        let s = &s[1..s.len()-1];
        let mut tokens = s.split(',');
        let name = tokens.next().unwrap();
        let mut size = 0;
        let mut fields = vec![];

        for token in tokens {
            let (typename, value) = token.trim()
                .split_at(token.find(' ').unwrap());

            fields.push(match typename {
                "int" => TupleField::Int(value.parse().ok()),
                "float" => TupleField::Float(value.parse().ok()),
                _ => return Err(TupleStringError::InvalidType(size))
            });
            size += 1;
        }

        Ok(Tuple { name, fields })
    }
}

