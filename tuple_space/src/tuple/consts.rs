#[allow(unused)]
pub const TUPLE_NAME_MAX_SIZE: usize = 31;
#[allow(unused)]
pub const TUPLE_FIELD_MAX_SIZE: usize = std::mem::size_of::<u8>() + std::mem::size_of::<u32>();
#[allow(unused)]
pub const TUPLE_MAX_FIELDS: usize = u8::MAX as usize;

#[allow(unused)]
pub const TUPLE_TYPE_UNDEFINED: u8 = 0b000;
#[allow(unused)]
pub const TUPLE_TYPE_INT: u8 = 0b001;
#[allow(unused)]
pub const TUPLE_TYPE_FLOAT: u8 = 0b010;

#[allow(unused)]
pub const TUPLE_FIELD_OCCUPIED_YES: u8 = 0b1;
#[allow(unused)]
pub const TUPLE_FIELD_OCCUPIED_NO: u8 = 0b0;

#[allow(unused)]
pub const TUPLE_FIELD_OCCUPIED_SHIFT: usize = 7;
#[allow(unused)]
pub const TUPLE_FIELD_TYPE_SHIFT: usize = 4;
