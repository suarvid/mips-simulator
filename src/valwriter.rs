pub trait NumValWriter {
    fn write(&self, val: Option<i32>) -> String;
    fn write_unsigned(&self, val: Option<u32>) -> String;
}

pub struct HexValWriter {}

impl NumValWriter for HexValWriter {
    fn write(&self, val: Option<i32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{:#0x}", val.unwrap())
        }
    }

    fn write_unsigned(&self, val: Option<u32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{:#0x}", val.unwrap())
        }
    }
}

pub struct DecValWriter {}

impl NumValWriter for DecValWriter {
    fn write(&self, val: Option<i32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{}", val.unwrap())
        }
    }
    fn write_unsigned(&self, val: Option<u32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{}", val.unwrap())
        }
    }
}

pub struct BinValWriter {}

impl NumValWriter for BinValWriter {
    fn write(&self, val: Option<i32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{:#0b}", val.unwrap())
        }
    }

    fn write_unsigned(&self, val: Option<u32>) -> String {
        if val.is_none() {
            format!("-")
        } else {
            format!("{:#0b}", val.unwrap())
        }
    }
}
