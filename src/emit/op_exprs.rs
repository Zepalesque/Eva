use OpCodeExpr::*;
use crate::core::OpCode;
use crate::emit::encode::{Encode, Encoder};

pub type RegId = u16;

pub enum OpCodeExpr {
    Const8(RegId, u8),
    Const16(RegId, u16),
    Const32(RegId, u32),
    Const64(RegId, u64),
    SignExt64(RegId, u32),
    SignExt32(RegId, u16),

}
impl Encode for OpCodeExpr {
    fn write(&self, encoder: &mut Encoder) {
        match self {
            Const8(id, val) => {
                OpCode::Const8.write(encoder);
                id.write(encoder);
                val.write(encoder);
            }
            Const16(_, _) => {}
            Const32(_, _) => {}
            Const64(_, _) => {}
            SignExt64(_, _) => {}
            SignExt32(_, _) => {}
        }
    }
}