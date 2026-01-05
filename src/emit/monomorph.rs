use crate::core::OpCode;
use crate::core::OpCode::*;
use GenericOp::*;

struct MonomorphResult<'a> {
    codes: &'a [OpCode],
    temp_count: usize,
}

const NO_OP: MonomorphResult = MonomorphResult {
    codes: &[],
    temp_count: usize::wrapping_sub(0, 1),
};

const fn mono_res(codes: &[OpCode]) -> MonomorphResult {
    MonomorphResult {
        codes,
        temp_count: codes.len() - 1,
    }
}

pub trait Monomorphize {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a>;
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
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const8]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8     => NO_OP,
            ToI16    => NO_OP,
            ToU8     => NO_OP,
            ToU16    => NO_OP,
            ToI32    => NO_OP,
            ToI64    => NO_OP,
            ToU32    => NO_OP,
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivU32]),
            Mod      => mono_res(&[IModU32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh8]),
            Rsh      => mono_res(&[IRshU8]),
            Lrot     => mono_res(&[ILrot8]),
            Rrot     => mono_res(&[IRrot8]),
        }
    }
}

impl Monomorphize for u16 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const16]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => NO_OP,
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => NO_OP,
            ToI32    => NO_OP,
            ToI64    => NO_OP,
            ToU32    => NO_OP,
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivU32]),
            Mod      => mono_res(&[IModU32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh16]),
            Rsh      => mono_res(&[IRshU16]),
            Lrot     => mono_res(&[ILrot16]),
            Rrot     => mono_res(&[IRrot16]),
        }
    }
}

impl Monomorphize for u32 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const32]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => mono_res(&[Trunc16]),
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => mono_res(&[Trunc16]),
            ToI32    => NO_OP,
            ToI64    => NO_OP,
            ToU32    => NO_OP,
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivU32]),
            Mod      => mono_res(&[IModU32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh32]),
            Rsh      => mono_res(&[IRshU32]),
            Lrot     => mono_res(&[ILrot32]),
            Rrot     => mono_res(&[IRrot32]),
        }
    }
}

impl Monomorphize for u64 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const64]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => mono_res(&[Trunc16]),
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => mono_res(&[Trunc16]),
            ToI32    => mono_res(&[Trunc32]),
            ToI64    => NO_OP,
            ToU32    => mono_res(&[Trunc32]),
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd64]),
            Sub      => mono_res(&[ISub64]),
            Mul      => mono_res(&[IMul64]),
            Div      => mono_res(&[IDivU64]),
            Mod      => mono_res(&[IModU64]),
            And      => mono_res(&[IAnd64]),
            Or       => mono_res(&[IOr64]),
            Xor      => mono_res(&[IXor64]),
            Not      => mono_res(&[IXor64]),
            Lsh      => mono_res(&[ILsh64]),
            Rsh      => mono_res(&[IRshU64]),
            Lrot     => mono_res(&[ILrot64]),
            Rrot     => mono_res(&[IRrot64]),
        }
    }
}

impl Monomorphize for i8 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const8]),
            ToF32    => mono_res(&[I32ToF32]),
            ToF64    => mono_res(&[I32ToF32, FProm]),
            ToI8    => NO_OP,
            ToI16    => mono_res(&[SignExt8To16]),
            ToI32    => mono_res(&[SignExt8To32]),
            ToI64    => mono_res(&[SignExt8To64]),
            ToU8    => NO_OP,
            ToU16    => mono_res(&[SignExt8To16]),
            ToU32    => mono_res(&[SignExt8To32]),
            ToU64    => mono_res(&[SignExt8To64]),
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivI32]),
            Mod      => mono_res(&[IModI32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh8]),
            Rsh      => mono_res(&[IRshI8]),
            Lrot     => mono_res(&[ILrot8]),
            Rrot     => mono_res(&[IRrot8]),
        }
    }
}

impl Monomorphize for u16 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const16]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => NO_OP,
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => NO_OP,
            ToI32    => NO_OP,
            ToI64    => NO_OP,
            ToU32    => NO_OP,
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivU32]),
            Mod      => mono_res(&[IModU32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh16]),
            Rsh      => mono_res(&[IRshU16]),
            Lrot     => mono_res(&[ILrot16]),
            Rrot     => mono_res(&[IRrot16]),
        }
    }
}

impl Monomorphize for u32 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const32]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => mono_res(&[Trunc16]),
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => mono_res(&[Trunc16]),
            ToI32    => NO_OP,
            ToI64    => NO_OP,
            ToU32    => NO_OP,
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd32]),
            Sub      => mono_res(&[ISub32]),
            Mul      => mono_res(&[IMul32]),
            Div      => mono_res(&[IDivU32]),
            Mod      => mono_res(&[IModU32]),
            And      => mono_res(&[IAnd32]),
            Or       => mono_res(&[IOr32]),
            Xor      => mono_res(&[IXor32]),
            Not      => mono_res(&[IXor32]),
            Lsh      => mono_res(&[ILsh32]),
            Rsh      => mono_res(&[IRshU32]),
            Lrot     => mono_res(&[ILrot32]),
            Rrot     => mono_res(&[IRrot32]),
        }
    }
}

impl Monomorphize for u64 {
    fn resolve<'a>(generic: GenericOp) -> MonomorphResult<'a> {
        match generic {
            Const    => mono_res(&[Const64]),
            ToF32    => mono_res(&[U32ToF32]),
            ToF64    => mono_res(&[U32ToF32, FProm]),
            ToI8    => mono_res(&[Trunc8]),
            ToI16    => mono_res(&[Trunc16]),
            ToU8    => mono_res(&[Trunc8]),
            ToU16    => mono_res(&[Trunc16]),
            ToI32    => mono_res(&[Trunc32]),
            ToI64    => NO_OP,
            ToU32    => mono_res(&[Trunc32]),
            ToU64    => NO_OP,
            Add      => mono_res(&[IAdd64]),
            Sub      => mono_res(&[ISub64]),
            Mul      => mono_res(&[IMul64]),
            Div      => mono_res(&[IDivU64]),
            Mod      => mono_res(&[IModU64]),
            And      => mono_res(&[IAnd64]),
            Or       => mono_res(&[IOr64]),
            Xor      => mono_res(&[IXor64]),
            Not      => mono_res(&[IXor64]),
            Lsh      => mono_res(&[ILsh64]),
            Rsh      => mono_res(&[IRshU64]),
            Lrot     => mono_res(&[ILrot64]),
            Rrot     => mono_res(&[IRrot64]),
        }
    }
}