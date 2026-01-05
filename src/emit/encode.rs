use crate::core::{OpCode, OpCodeRepr, IS_BIG_ENDIAN};

pub struct Encoder {
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

    fn write<T: Encode>(&mut self, val: T) {
        val.write(self);
    }
}

pub trait Encode {
    fn write(&self, encoder: &mut Encoder);
}

impl Encode for u8 {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self);
    }
}

impl Encode for bool {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self as u8);
    }
}

impl Encode for u16 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for u32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for u64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for usize {
    fn write(&self, encoder: &mut Encoder) {
        (*self as u64).write(encoder);
    }
}

impl Encode for i8 {
    fn write(&self, encoder: &mut Encoder) {
        encoder.write_byte(*self as u8);
    }
}

impl Encode for i16 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for i32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for i64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for isize {
    fn write(&self, encoder: &mut Encoder) {
        (*self as i64).write(encoder);
    }
}

impl Encode for f32 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for f64 {
    fn write(&self, encoder: &mut Encoder) {
        if IS_BIG_ENDIAN {
            encoder.write_arr(&self.to_be_bytes());
        } else {
            encoder.write_arr(&self.to_le_bytes());
        }
    }
}

impl Encode for OpCode {
    fn write(&self, encoder: &mut Encoder) {
        (*self as OpCodeRepr).write(encoder)
    }
}