use crate::emit::encode::{Encode, Encoder};
use crate::emit::monomorph::*;
use OpExpr::*;

pub type RegId = u16;



pub enum OpExpr<T: Monomorphize + Encode> {
    Const{dest: RegId, val: T},
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

impl<T: Encode + Monomorphize> Encode for OpExpr<T> {
    fn write(&self, encoder: &mut Encoder) {
        match self {
            Const{dest, val} => {
                T::resolve(GenericOp::Const).write(encoder);
                dest.write(encoder);
                val.write(encoder);
            }
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
            Div { first, second, dest } => {
                T::resolve(GenericOp::Div).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Mod { first, second, dest } => {
                T::resolve(GenericOp::Mod).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Lsh { first, second, dest } => {
                T::resolve(GenericOp::Lsh).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Rsh { first, second, dest } => {
                T::resolve(GenericOp::Rsh).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Lrot { first, second, dest } => {
                T::resolve(GenericOp::Lrot).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            Rrot { first, second, dest } => {
                T::resolve(GenericOp::Rrot).write(encoder);
                first.write(encoder);
                second.write(encoder);
                dest.write(encoder);
            },
            ToU8 { operand, dest } => {
                T::resolve(GenericOp::ToU8).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToU16 { operand, dest } => {
                T::resolve(GenericOp::ToU16).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToU32 { operand, dest } => {
                T::resolve(GenericOp::ToU32).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToU64 { operand, dest } => {
                T::resolve(GenericOp::ToU64).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToI8 { operand, dest } => {
                T::resolve(GenericOp::ToI8).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToI16 { operand, dest } => {
                T::resolve(GenericOp::ToI16).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToI32 { operand, dest } => {
                T::resolve(GenericOp::ToI32).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToI64 { operand, dest } => {
                T::resolve(GenericOp::ToI64).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToF32 { operand, dest } => {
                T::resolve(GenericOp::ToF32).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
            ToF64 { operand, dest } => {
                T::resolve(GenericOp::ToF64).write(encoder);
                operand.write(encoder);
                dest.write(encoder);
            },
        }
    }
}