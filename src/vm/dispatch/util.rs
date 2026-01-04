use crate::dispatch::*;
use crate::{Res, VmCtx};
use eva::core::OpCode;
use pastey::paste;
use std::mem;

pub type FnPtr = fn(VmCtx) -> Res;

pub macro mkdisp {
   {$name:ident { $( $start:ident $(. $next:ident)* := $fnptr:expr;)* }} => {
      const $name: [FnPtr; COUNT] = create_disp_table(&[
         $(Entry {opcode: paste!{
            OpCode::[< _$start$(__$next)* >]
        }, fnptr: $fnptr }),*], nop);
   }
}

pub macro next {
   [$ctx:expr,$table:expr] => {

   }
}

pub struct Entry {
   pub opcode: OpCode,
   pub fnptr: FnPtr
}

pub const COUNT: usize = mem::variant_count::<OpCode>();

#[inline(always)]
pub const fn create_disp_table<const N: usize>(entries: &[Entry; N], default: FnPtr) -> [FnPtr; COUNT] {
   let mut table: [FnPtr; COUNT] = [default; COUNT];

   let mut i: usize = 0;
   while i < entries.len() {
      let code = &entries[i].opcode;
      table[*code as usize] = entries[i].fnptr;

      i += 1;
   }

   return table;
}