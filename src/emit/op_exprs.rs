use crate::emit::encode::{Encode, Encoder};
use crate::emit::monomorph::*;
use ConstExpr::*;
use BinExpr::*;

pub type RegId = u16;



pub enum BinExpr<T: Monomorphize> {
    Add  { first: RegId, second: RegId, dest: RegId, },
    Sub  { first: RegId, second: RegId, dest: RegId, },
    Mul  { first: RegId, second: RegId, dest: RegId, },
    Div  { first: RegId, second: RegId, dest: RegId, },
    Mod  { first: RegId, second: RegId, dest: RegId, },
    Lsh  { first: RegId, second: RegId, dest: RegId, },
    Rsh  { first: RegId, second: RegId, dest: RegId, },
    Lrot { first: RegId, second: RegId, dest: RegId, },
    Rrot { first: RegId, second: RegId, dest: RegId, },
    ToU8 { operand: RegId, dest: RegId, },
    ToU16 { operand: RegId, dest: RegId, },
    ToU32 { operand: RegId, dest: RegId, },
    ToU64 { operand: RegId, dest: RegId, },
    ToI8 { operand: RegId, dest: RegId, },
    ToI16 { operand: RegId, dest: RegId, },
    ToI32 { operand: RegId, dest: RegId, },
    ToI64 { operand: RegId, dest: RegId, },
    ToF32 { operand: RegId, dest: RegId, },
    ToF64 { operand: RegId, dest: RegId, },
}

impl<T: Encode + Monomorphize> Encode for BinExpr<T> {
    fn write(&self, encoder: &mut Encoder) {
        match self {
            Add { first, second, dest } => {
                T::resolve(GenericOp::Add).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Sub { first, second, dest } => {
                T::resolve(GenericOp::Sub).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Mul { first, second, dest } => {
                T::resolve(GenericOp::Mul).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            }
        }
    }
}

pub enum ConstExpr<T: Encode + Monomorphize> {
    Const{dest: RegId, val: T},
}
impl<T: Encode + Monomorphize> Encode for ConstExpr<T> {
    fn write(&self, encoder: &mut Encoder) {
        match self {
            Const{dest, val} => {
                T::resolve(GenericOp::Const).write(encoder);
                dest.write(encoder);
                val.write(encoder);
            }
        }
    }
}