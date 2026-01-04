use crate::structures::decode::{Decode, Decoder};

#[derive(Copy, Clone)]
pub struct BytecodeHeader {
    format: Format,
}

type Format = u8;

const IS_X64: Format = 1;


impl BytecodeHeader {
    pub fn is_x64(&self) -> bool {
        self.format == IS_X64
    }
}

impl Decode for BytecodeHeader {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        let fmt: Format = decoder.read_byte();
        Self { format: fmt }
    }
}


