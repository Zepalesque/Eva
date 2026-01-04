pub struct Stack {
    pub scratch: *mut u64,
}


pub struct Register {
    value: u64,
}

impl Register {
    #[inline(always)]
    pub fn get<T: Registrant>(&self) -> T {
        T::from_reg(self)
    }

    #[inline(always)]
    pub fn set<T: Registrant>(&mut self, value: T) {
        value.to_reg(self);
    }
}

pub trait Registrant {
    fn from_reg(reg: &Register) -> Self;
    fn to_reg(&self, reg: &mut Register);
}

impl Registrant for u64 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self;
    }
}

impl Registrant for u32 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as u32
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for u16 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as u16
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for u8 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as u8
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for usize {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as usize
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for i64 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as i64
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for i32 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as i32
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for i16 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as i16
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64
    }
}

impl Registrant for i8 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as i8
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for isize {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value as isize
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64;
    }
}

impl Registrant for f32 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        f32::from_bits(reg.value as u32)
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = self.to_bits() as u64;
    }
}

impl Registrant for f64 {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        f64::from_bits(reg.value)
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = self.to_bits();
    }
}

impl Registrant for bool {
    #[inline(always)]
    fn from_reg(reg: &Register) -> Self {
        reg.value != 0
    }

    #[inline(always)]
    fn to_reg(&self, reg: &mut Register) {
        reg.value = *self as u64
    }
}