#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const foo: &'static [u8; 4] = b"bar\0";
pub const CHAR: u8 = 98u8;
pub const CHARR: u8 = 0u8;
pub const FLOAT: f64 = 5.09;
pub const FLOAT_EXPR: f64 = 0.005;
pub const LONG: u32 = 3;
pub const INVALID_UTF8: &'static [u8; 5] = b"\xF0(\x8C(\0";
