use crate::core::{OpCode, IS_BIG_ENDIAN};

pub struct Encoder {
    x64: bool,
    bytes: Vec<u8>,
}

impl Encoder {
    fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn write_slice(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    fn write_arr<const N: usize>(&mut self, bytes: &[u8; N]) {
        self.bytes.extend_from_slice(bytes);
    }

    fn write<T: Encodee>(&mut self, val: T) {
        val.write(self);
    }
}

pub trait Encodee {
    fn write(&self, encoder: &mut Encoder);
}

impl Encodee for u8 {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self);
    }
}

impl Encodee for bool {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self as u8);
    }
}

impl Encodee for u16 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for u32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for u64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for usize {
    fn write(&self, encoder: &mut Encoder) {
        if encoder.x64 {
            (*self as u64).write(encoder);
        } else {
            (*self as u32).write(encoder);
        }
    }
}

impl Encodee for i8 {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self as u8);
    }
}

impl Encodee for i16 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for i32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for i64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for isize {
    fn write(&self, encoder: &mut Encoder) {
        if encoder.x64 {
            (*self as i64).write(encoder);
        } else {
            (*self as i32).write(encoder);
        }
    }
}

impl Encodee for f32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for f64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encodee for OpCode {
    fn write(&self, encoder: &mut Encoder) {
        (*self as u8).write(encoder)
    }
}