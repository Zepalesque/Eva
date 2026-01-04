pub type RegId = u16;

pub enum OpCodeExpr {
    Const8(RegId, u8),
    Const16(RegId, u16),
    Const32(RegId, u32),
    Const64(RegId, u64),
    SignExt64(u16)
}
impl OpCodeExpr {


}