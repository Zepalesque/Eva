use num_enum::{IntoPrimitive, TryFromPrimitive};
pub const IS_BIG_ENDIAN: bool = true;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Nop, // noop
    Const8, // const
    Const16, // const
    Const32, // const
    Const64, // const
    SignExt64, // sext<64>
    SignExt32, // sext<32>
    SignExt16, // sext<16>
    ZeroExt64, // zext<...
    ZeroExt32,
    ZeroExt16,
    Truncat32, // trnc<...
    Truncat16,
    Truncat8,
    U32ToF32, // utof<32>
    I32ToF32, // itof<32>
    U64ToF64, // utof...
    I64ToF64,
    F32ToU32, // ftou<32>
    F64ToU64, // ...
    F32ToI32, // ftoi<...
    F64ToI64,
    FProm, // fprom (f32 -> f64)
    FDemo, // fdemo (f64 -> f32)
    IAdd32,
    IAdd64,
    ISub32,
    ISub64,
    FAdd32,
    FAdd64,
    FSub32,
    FSub64,
    IMul32,
    IMul64,
    FMul32,
    FMul64,
    IDivI8,
    IDivI16,
    IDivI32,
    IDivI64,
    IDivU8,
    IDivU16,
    IDivU32,
    IDivU64,
    FDiv32,
    FDiv64,
    IModI8,
    IModI16,
    IModI32,
    IModI64,
    IModU8,
    IModU16,
    IModU32,
    IModU64,
    FMod32,
    FMod64,
    IAnd8,
    IAnd16,
    IAnd32,
    IAnd64,
    IOr8,
    IOr16,
    IOr32,
    IOr64,
    IXor8,
    IXor16,
    IXor32,
    IXor64,
    INot8,
    INot16,
    INot32,
    INot64,
    ILsh8,
    ILsh16,
    ILsh32,
    ILsh64,
    IRshU8,
    IRshU16,
    IRshU32,
    IRshU64,
    IRshI8,
    IRshI16,
    IRshI32,
    IRshI64,
    ILrot8,
    ILrot16,
    ILrot32,
    ILrot64,
    IRrot8,
    IRrot16,
    IRrot32,
    IRrot64,
    RegAddr,
}