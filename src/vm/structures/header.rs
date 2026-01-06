use crate::structures::decode::{Decode, Decoder};

#[derive(Copy, Clone)]
pub struct BytecodeHeader {
    format: Format,
}

type Format = u8;

const IS_EXECUTABLE: Format = 1;
const IS_X64: Format = 2;


impl BytecodeHeader {
    pub fn is_x64(&self) -> bool {
        self.format & IS_X64 != 0
    }

    pub fn is_exec(&self) -> bool {
        self.format & IS_EXECUTABLE != 0
    }
}

impl Decode for BytecodeHeader {
    fn read(decoder: &mut Decoder) -> Self {
        let fmt: Format = decoder.read_byte();
        Self { format: fmt }
    }
}


