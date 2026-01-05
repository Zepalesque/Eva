use crate::core::opcode::OpCode;
use crate::core::opcode::OpCode::*;
use GenericOp::*;


pub trait Monomorphize {
    fn resolve(generic: GenericOp) -> OpCode;
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
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const8,
            ToF32    => U32ToF32,
            ToF64    => U32ToF64,
            ToI8     => Move,
            ToI16    => Move,
            ToU8     => Move,
            ToU16    => Move,
            ToI32    => Move,
            ToI64    => Move,
            ToU32    => Move,
            ToU64    => Move,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivU32,
            Mod      => IModU32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh8,
            Rsh      => IRshU8,
            Lrot     => ILrot8,
            Rrot     => IRrot8,
        }
    }
}

impl Monomorphize for u16 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const16,
            ToF32    => U32ToF32,
            ToF64    => U32ToF64,
            ToI8     => Trunc8,
            ToI16    => Move,
            ToU8     => Trunc8,
            ToU16    => Move,
            ToI32    => Move,
            ToI64    => Move,
            ToU32    => Move,
            ToU64    => Move,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivU32,
            Mod      => IModU32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh16,
            Rsh      => IRshU16,
            Lrot     => ILrot16,
            Rrot     => IRrot16,
        }
    }
}

impl Monomorphize for u32 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const32,
            ToF32    => U32ToF32,
            ToF64    => U32ToF64,
            ToI8     => Trunc8,
            ToI16    => Trunc16,
            ToU8     => Trunc8,
            ToU16    => Trunc16,
            ToI32    => Move,
            ToI64    => Move,
            ToU32    => Move,
            ToU64    => Move,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivU32,
            Mod      => IModU32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh32,
            Rsh      => IRshU32,
            Lrot     => ILrot32,
            Rrot     => IRrot32,
        }
    }
}

impl Monomorphize for u64 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const64,
            ToF32    => U64ToF32,
            ToF64    => U64ToF64,
            ToI8     => Trunc8,
            ToI16    => Trunc16,
            ToU8     => Trunc8,
            ToU16    => Trunc16,
            ToI32    => Trunc32,
            ToI64    => Move,
            ToU32    => Trunc32,
            ToU64    => Move,
            Add      => IAdd64,
            Sub      => ISub64,
            Mul      => IMul64,
            Div      => IDivU64,
            Mod      => IModU64,
            And      => IAnd64,
            Or       => IOr64,
            Xor      => IXor64,
            Not      => IXor64,
            Lsh      => ILsh64,
            Rsh      => IRshU64,
            Lrot     => ILrot64,
            Rrot     => IRrot64,
        }
    }
}

impl Monomorphize for i8 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const8,
            ToF32    => I32ToF32,
            ToF64    => I32ToF64,
            ToI8     => Move,
            ToI16    => SignExt8To16,
            ToI32    => SignExt8To32,
            ToI64    => SignExt8To64,
            ToU8     => Move,
            ToU16    => SignExt8To16,
            ToU32    => SignExt8To32,
            ToU64    => SignExt8To64,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivI32,
            Mod      => IModI32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh8,
            Rsh      => IRshI8,
            Lrot     => ILrot8,
            Rrot     => IRrot8,
        }
    }
}

impl Monomorphize for i16 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const16,
            ToF32    => I32ToF32,
            ToF64    => I32ToF64,
            ToI8     => Trunc8,
            ToI16    => Move,
            ToI32    => SignExt16To32,
            ToI64    => SignExt16To64,
            ToU8     => Trunc8,
            ToU16    => Move,
            ToU32    => SignExt16To32,
            ToU64    => SignExt16To64,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivI32,
            Mod      => IModI32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh16,
            Rsh      => IRshI16,
            Lrot     => ILrot16,
            Rrot     => IRrot16,
        }
    }
}

impl Monomorphize for i32 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const32,
            ToF32    => I32ToF32,
            ToF64    => I32ToF64,
            ToI8     => Trunc8,
            ToI16    => Trunc16,
            ToI32    => Move,
            ToI64    => SignExt32To64,
            ToU8     => Trunc8,
            ToU16    => Trunc16,
            ToU32    => Move,
            ToU64    => SignExt32To64,
            Add      => IAdd32,
            Sub      => ISub32,
            Mul      => IMul32,
            Div      => IDivI32,
            Mod      => IModI32,
            And      => IAnd32,
            Or       => IOr32,
            Xor      => IXor32,
            Not      => IXor32,
            Lsh      => ILsh32,
            Rsh      => IRshI32,
            Lrot     => ILrot32,
            Rrot     => IRrot32,
        }
    }
}

impl Monomorphize for i64 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const64,
            ToF32    => I64ToF32,
            ToF64    => I64ToF64,
            ToI8     => Trunc8,
            ToI16    => Trunc16,
            ToI32    => Trunc32,
            ToI64    => Move,
            ToU8     => Trunc8,
            ToU16    => Trunc16,
            ToU32    => Trunc32,
            ToU64    => Move,
            Add      => IAdd64,
            Sub      => ISub64,
            Mul      => IMul64,
            Div      => IDivI64,
            Mod      => IModI64,
            And      => IAnd64,
            Or       => IOr64,
            Xor      => IXor64,
            Not      => IXor64,
            Lsh      => ILsh64,
            Rsh      => IRshI64,
            Lrot     => ILrot64,
            Rrot     => IRrot64,
        }
    }
}

impl Monomorphize for f64 {
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const64,
            ToF32    => FDemo,
            ToF64    => Move,
            ToI8     => panic!("No monomorphization for f64 -> i8"),
            ToI16    => panic!("No monomorphization for f64 -> i16"),
            ToI32    => F64ToI32,
            ToI64    => F64ToI64,
            ToU8     => panic!("No monomorphization for f64 -> u8"),
            ToU16    => panic!("No monomorphization for f64 -> u16"),
            ToU32    => F64ToU32,
            ToU64    => F64ToU64,
            Add      => FAdd64,
            Sub      => FSub64,
            Mul      => FMul64,
            Div      => FDiv64,
            Mod      => FMod64,
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
    fn resolve(generic: GenericOp) -> OpCode {
        match generic {
            Const    => Const32,
            ToF32    => Move,
            ToF64    => FProm,
            ToI8     => panic!("No monomorphization for f32 -> i8"),
            ToI16    => panic!("No monomorphization for f32 -> i16"),
            ToI32    => F32ToI32,
            ToI64    => F32ToI64,
            ToU8     => panic!("No monomorphization for f32 -> u8"),
            ToU16    => panic!("No monomorphization for f32 -> u16"),
            ToU32    => F32ToU32,
            ToU64    => F32ToU64,
            Add      => FAdd32,
            Sub      => FSub32,
            Mul      => FMul32,
            Div      => FDiv32,
            Mod      => FMod32,
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