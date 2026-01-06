use clap::builder::Str;
use crate::emit::encode::{Encode, Encoder};
use crate::emit::monomorph::*;
use OpExpr::*;




#[derive(Clone, Debug, PartialEq)]
pub enum OpExpr<T: Monomorphize + Encode + Clone> {
    Const{ val: T, dest: String },
    Add  { first: String, second: String, dest: String, },
    Sub  { first: String, second: String, dest: String, },
    Mul  { first: String, second: String, dest: String, },
    Div  { first: String, second: String, dest: String, },
    Mod  { first: String, second: String, dest: String, },
    Lsh  { first: String, second: String, dest: String, },
    Rsh  { first: String, second: String, dest: String, },
    Lrot { first: String, second: String, dest: String, },
    Rrot { first: String, second: String, dest: String, },
    ToU8 { operand: String, dest: String, },
    ToU16 { operand: String, dest: String, },
    ToU32 { operand: String, dest: String, },
    ToU64 { operand: String, dest: String, },
    ToI8 { operand: String, dest: String, },
    ToI16 { operand: String, dest: String, },
    ToI32 { operand: String, dest: String, },
    ToI64 { operand: String, dest: String, },
    ToF32 { operand: String, dest: String, },
    ToF64 { operand: String, dest: String, },
}

impl<T: Encode + Monomorphize + Clone> OpExpr<T> {
    fn args(self) -> OpArgs<T> {
        match self {
            Const { val, dest } => { OpArgs::Const { val, dest } },
            Add { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Sub { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Mul { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Div { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Mod { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Lsh { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Rsh { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Lrot { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            Rrot { first, second, dest } => { OpArgs::Binary { first, second, dest } },
            ToU8 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToU16 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToU32 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToU64 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToI8 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToI16 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToI32 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToI64 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToF32 { operand, dest } => { OpArgs::Unary { operand, dest } },
            ToF64 { operand, dest } => { OpArgs::Unary { operand, dest } },
        }
    }
}
impl<T: Encode + Monomorphize + Clone> Encode for OpExpr<T> {



    fn write(&self, encoder: &mut Encoder) {
        match self {
            Const { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Const, args);
            } Add { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Add, args);
            }
            Sub { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Sub, args);
            }
            Mul { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Mul, args);
            }
            Div { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Div, args);
            }
            Mod { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Mod, args);
            }
            Lsh { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Lsh, args);
            }
            Rsh { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Rsh, args);
            }
            Lrot { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Lrot, args);
            }
            Rrot { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::Rrot, args);
            }
            ToU8 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToU8, args);
            }
            ToU16 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToU16, args);
            }
            ToU32 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToU32, args);
            }
            ToU64 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToU64, args);
            }
            ToI8 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToI8, args);
            }
            ToI16 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToI16, args);
            }
            ToI32 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToI32, args);
            }
            ToI64 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToI64, args);
            }
            ToF32 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToF32, args);
            }
            ToF64 { .. } => {
                let args = self.clone().args();
                T::resolve(encoder, GenericOp::ToF64, args);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpArgs<T: Monomorphize + Encode + Sized> {
    Const { val: T, dest: String },
    Unary { operand: String, dest : String },
    Binary { first: String, second: String, dest : String },
}