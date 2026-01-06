use crate::core::opcode::OpCode;
use crate::core::opcode::OpCode::*;
use GenericOp::*;
use crate::emit::encode::{Encode, Encoder};
use crate::emit::op_exprs::OpArgs;

pub trait Monomorphize {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) where Self: Sized + Encode;
}

pub enum GenericOp {
    Const,
    ToF32, ToF64,
    ToI8, ToI16, ToI32, ToI64,
    ToU8, ToU16, ToU32, ToU64,
    Add, Sub, Mul, Div, Mod,
    And, Or, Xor, Not,
    Lsh, Rsh, Lrot, Rrot,
}

impl Monomorphize for u8 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshU8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for u16 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshU16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for u32 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for u64 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U64ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    U64ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivU64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModU64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshU64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for i8 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt8To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshI8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for i16 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt16To32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt16To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt16To32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt16To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshI16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for i32 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I32ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt32To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    SignExt32To64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for i64 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I64ToF32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    I64ToF64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToU8 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc8.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU16 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc16.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    Trunc32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAdd64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ISub64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IMul64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IDivI64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IModI64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IAnd64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Or => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IOr64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Xor => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IXor64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Not => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    INot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Lsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILsh64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rsh => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRshI64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Lrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    ILrot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Rrot => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    IRrot64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
        }
    }
}

impl Monomorphize for f64 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    FDemo.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToI8     => panic!("No monomorphization for f64 -> i8"),
            ToI16    => panic!("No monomorphization for f64 -> i16"),
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F64ToI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F64ToI64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8     => panic!("No monomorphization for f64 -> u8"),
            ToU16    => panic!("No monomorphization for f64 -> u16"),
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F64ToU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F64ToU64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FAdd64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FSub64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FMul64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FDiv64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FMod64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And      => panic!("No monomorphization for f64 & f64"),
            Or       => panic!("No monomorphization for f64 | f64"),
            Xor      => panic!("No monomorphization for f64 ^ f64"),
            Not      => panic!("No monomorphization for !f64"),
            Lsh      => panic!("No monomorphization for f64 << f64"),
            Rsh      => panic!("No monomorphization for f64 >> f64"),
            Lrot     => panic!("No monomorphization for f64 <<< f64"),
            Rrot     => panic!("No monomorphization for f64 >>> f64"),
        }
    }
}

impl Monomorphize for f32 {
    fn resolve(encoder: &mut Encoder, generic: GenericOp, args: OpArgs<Self>) {
        match generic {
            Const => {
                single_match!(args: OpArgs::Const { val, dest } => {
                    Const32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    val.write(encoder);
                });
            },
            ToF32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    encoder.vars().alias(operand.as_str(), dest);
                });
            },
            ToF64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    FProm.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI8     => panic!("No monomorphization for f32 -> i8"),
            ToI16    => panic!("No monomorphization for f32 -> i16"),
            ToI32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F32ToI32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToI64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F32ToI64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU8     => panic!("No monomorphization for f32 -> u8"),
            ToU16    => panic!("No monomorphization for f32 -> u16"),
            ToU32 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F32ToU32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            ToU64 => {
                single_match!(args: OpArgs::Unary { operand, dest } => {
                    F32ToU64.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(operand.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", operand)).write(encoder);
                });
            },
            Add => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FAdd32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Sub => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FSub32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mul => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FMul32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Div => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FDiv32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            Mod => {
                single_match!(args: OpArgs::Binary { first, second, dest } => {
                    FMod32.write(encoder);
                    encoder.vars().add_or_shadow(dest.as_str()).write(encoder);
                    encoder.vars().get(first.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", first)).write(encoder);
                    encoder.vars().get(second.as_str()).unwrap_or_else(|| panic!("Undefined variable: {}", second)).write(encoder);
                });
            },
            And      => panic!("No monomorphization for f32 & f32"),
            Or       => panic!("No monomorphization for f32 | f32"),
            Xor      => panic!("No monomorphization for f32 ^ f32"),
            Not      => panic!("No monomorphization for !f32"),
            Lsh      => panic!("No monomorphization for f32 << f32"),
            Rsh      => panic!("No monomorphization for f32 >> f32"),
            Lrot     => panic!("No monomorphization for f32 <<< f32"),
            Rrot     => panic!("No monomorphization for f32 >>> f32"),
        }
    }
}

macro single_match {
    ($id:ident: $p:pat => $b:block) => {
        match $id {
            $p => $b,
            _ => panic!("Invalid arg type!")
        }
    }
}