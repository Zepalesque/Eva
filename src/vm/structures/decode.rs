use std::slice;
use eva::core::{OpCode, IS_BIG_ENDIAN};

#[derive(Copy, Clone)]
pub struct Decoder {
    pub(crate) start: *const u8,
    pub(crate) offs: usize,
}

impl Decoder {
    unsafe fn curr(&self) -> *const u8 {
        unsafe { self.start.add(self.offs) }
    }

    pub fn read_byte(&mut self) -> u8 {
        let val: u8 = unsafe { *self.curr() };
        self.offs += 1;
        val
    }

    pub fn read_slice<'a>(&mut self, n: usize) -> &'a [u8] {
        if n == 0 { return &[] }
        let slice = unsafe { slice::from_raw_parts::<'a>(self.curr(), n) };
        self.offs += n;
        slice
    }

    pub fn read_arr<'a, const N: usize>(&mut self) -> [u8; N] {
        if N == 0 { return []; }
        let arr = unsafe {std::ptr::read_unaligned(self.curr() as *const [u8; N]) };
        self.offs += N;
        arr
    }

    pub fn read<T: Decode, const PS: usize>(&mut self) -> T {
        T::read::<PS>(self)
    }

    pub fn fork_abs(&self, offs: usize) -> Self {
        Self {
            start: self.start,
            offs,
        }
    }

    pub fn fork_inc(&self, offs: usize) -> Self {
        Self {
            start: self.start,
            offs: self.offs + offs,
        }
    }

    pub fn fork_dec(&self, offs: usize) -> Self {
        Self {
            start: self.start,
            offs: self.offs - offs,
        }
    }
}


pub trait Decode {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self;
}

impl Decode for u8 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        decoder.read_byte()
    }
}

impl Decode for bool {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        decoder.read_byte() != 0
    }
}

impl Decode for u16 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<2>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<2>())
        }
    }
}

impl Decode for u32 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<4>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<4>())
        }
    }
}

impl Decode for u64 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<8>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<8>())
        }
    }
} 

impl Decode for usize {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<PS>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<PS>())
        }
    }
}

impl Decode for i8 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        decoder.read_byte() as i8
    }
}

impl Decode for i16 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<2>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<2>())
        }
    }
}

impl Decode for i32 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<4>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<4>())
        }
    }
}

impl Decode for i64 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<8>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<8>())
        }
    }
}

impl Decode for isize {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<PS>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<PS>())
        }
    }
}

impl Decode for f32 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<4>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<4>())
        }    }
}

impl Decode for f64 {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        if IS_BIG_ENDIAN {
            Self::from_be_bytes(decoder.read_arr::<8>())
        } else {
            Self::from_le_bytes(decoder.read_arr::<8>())
        }
    }
}

impl Decode for OpCode {
    fn read<const PS: usize>(decoder: &mut Decoder) -> Self {
        let num = u8::read(decoder);
        OpCode::try_from(num).expect("Invalid OpCode found: {}")
    }
}