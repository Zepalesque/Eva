use ConstExpr::*;
use crate::core::OpCode;
use crate::emit::encode::{Encode, Encoder};
use crate::emit::monomorph::*;

pub type RegId = u16;



pub enum BinExpr<T: Monomorphize> {
    Add {
        dest: RegId,
        first: RegId,
        second: RegId,
    },
    Sub {
        dest: RegId,
        first: RegId,
        second: RegId,
    }
}

pub enum ConstExpr<T: Encode> {
    Const(RegId, T),
    SignExt64(RegId, u32),
    SignExt32(RegId, u16),
}
impl<T: Encode> Encode for ConstExpr<T> {
    fn write(&self, encoder: &mut Encoder) {
        match self {
            Const(id, val) => {
                OpCode::Const8.write(encoder);
                id.write(encoder);
                val.write(encoder);
            }
            SignExt64(_, _) => {}
            SignExt32(_, _) => {}
        }
    }
}