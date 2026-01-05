#![feature(explicit_tail_calls)]
#![feature(decl_macro)]
#![feature(portable_simd)]
#![feature(variant_count)]
#![feature(ergonomic_clones)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
use crate::structures::decode::{Decode, Decoder};
use crate::structures::header::BytecodeHeader;
use clap::Parser;
use eva::core::opcode::OpCode;
use memmap2::Mmap;
use memmap2::MmapOptions;
use size::Size;
use std::fs::File;
use std::path::PathBuf;
use structures::registers::*;
use dispatch::*;

mod structures;
mod dispatch;

pub type Res = Option<u32>;

#[derive(Copy, Clone)]
struct VmCtx {
    pub decoder: Decoder,
    regs: *mut Register,
}

impl VmCtx {
    pub fn get_reg(&mut self, idx: u16) -> &mut Register {
        unsafe { &mut *self.regs.add(idx as usize) }
    }

    pub fn read_reg(&mut self) -> &mut Register {
        let idx = self.decoder.read::<u16>();
        self.get_reg(idx)
    }

    pub fn reg_as<T: Registrant>(&mut self) -> T {
        let idx = self.decoder.read::<u16>();
        self.get_reg(idx).get::<T>()
    }
}

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    run: PathBuf,
    #[clap(short, long, default_value = "16MiB", help = "The maximum virtual register stack size")]
    reg_size: Size,
    #[clap(short, long, default_value = "", help = "Program args to pass to the program")]
    args: Vec<String>,
}

fn main() { unsafe {
    let args: Args = Args::parse();

    let file = File::open(&args.run).expect("file not found");
    let binary = Mmap::map(&file).expect("Failed to map file");

    let size: i64 = args.reg_size.bytes();
    if size < 0 || size as u64 > usize::MAX as u64 {
        panic!();
    }
    let regs = MmapOptions::new()
        .len(size.try_into()
            .expect(format!("Invalid register size {}, should be in the range of [0, {}]", size, usize::MAX)
                .as_str())
        ).map_anon().expect("Failed to create register block").as_ptr();
    let mut decoder: Decoder = Decoder { start: binary.as_ptr(), offs: 0 };
    // PointerSize const param is irrelevant at this point
    let header: BytecodeHeader = BytecodeHeader::read(&mut decoder).clone();

    let x64 = header.is_x64();
    if size_of::<usize>() < 8 && x64 {
        panic!("Tried to run x64 program on x86 system!")
    }

    let ctx: VmCtx = VmCtx {
        decoder,
        regs: regs as *mut Register,
    };

    start(ctx);
}}

fn start(mut ctx: VmCtx) -> u32 {
    loop {
        match ctx.decoder.read::<OpCode>() {
            OpCode::Noop => { continue; }
            OpCode::Const8 => {
                read!(ctx -> u8);
            }
            OpCode::Const16 => {
                read!(ctx -> u16);
            }
            OpCode::Const32 => {
                read!(ctx -> u32);
            }
            OpCode::Const64 => {
                read!(ctx -> u64);
            }
            OpCode::SignExt32To64 => {
                conv!(ctx -> i32 as i64);
            }
            OpCode::SignExt16To32 => {
                conv!(ctx -> i16 as i32);
            }
            OpCode::SignExt16To64 => {
                conv!(ctx -> i16 as i64);
            }
            OpCode::SignExt8To16 => {
                conv!(ctx -> i8 as i16);
            }
            OpCode::SignExt8To32 => {
                conv!(ctx -> i8 as i32);
            }
            OpCode::SignExt8To64 => {
                conv!(ctx -> i8 as i64);
            }
            OpCode::Trunc32 => {
                conv!(ctx -> u64 as u32);
            }
            OpCode::Trunc16 => {
                conv!(ctx -> u64 as u16);
            }
            OpCode::Trunc8 => {
                conv!(ctx -> u64 as u8);
            }
            OpCode::U32ToF32 => {
                conv!(ctx -> u32 as f32);
            },
            OpCode::I32ToF32 => {
                conv!(ctx -> i32 as f32);
            },
            OpCode::U64ToF64 => {
                conv!(ctx -> u64 as f64);
            },
            OpCode::I64ToF64 => {
                conv!(ctx -> i64 as f64);
            },
            OpCode::F32ToU32 => {
                conv!(ctx -> f32 as u32);
            },
            OpCode::F64ToU64 => {
                conv!(ctx -> f64 as u64);
            },
            OpCode::F32ToI32 => {
                conv!(ctx -> f32 as i32);
            },
            OpCode::F64ToI64 => {
                conv!(ctx -> f64 as i64);
            },
            OpCode::FProm => {
                conv!(ctx -> f32 as f64);
            },
            OpCode::FDemo => {
                conv!(ctx -> f64 as f32);
            },
            OpCode::IAdd32 => {
                binary_op!(ctx -> u32: wrapping_add);
            },
            OpCode::IAdd64 => {
                binary_op!(ctx -> u64: wrapping_add);
            },
            OpCode::ISub32 => {
                binary_op!(ctx -> u32: wrapping_sub);
            },
            OpCode::ISub64 => {
                binary_op!(ctx -> u64: wrapping_sub);
            },
            OpCode::FAdd32 => {
                binary_op!(ctx -> f32: +);
            },
            OpCode::FAdd64 => {
                binary_op!(ctx -> f64: +);
            },
            OpCode::FSub32 => {
                binary_op!(ctx -> f32: -);
            },
            OpCode::FSub64 => {
                binary_op!(ctx -> f64: -);
            },
            OpCode::IMul32 => {
                binary_op!(ctx -> u32: wrapping_mul);
            },
            OpCode::IMul64 => {
                binary_op!(ctx -> u64: wrapping_mul);
            },
            OpCode::FMul32 => {
                binary_op!(ctx -> f32: *);
            },
            OpCode::FMul64 => {
                binary_op!(ctx -> f64: *);
            },
            OpCode::IDivI32 => {
                binary_op!(ctx -> i32: /);
            },
            OpCode::IDivI64 => {
                binary_op!(ctx -> i64: /);
            },
            OpCode::IDivU32 => {
                binary_op!(ctx -> u32: /);
            },
            OpCode::IDivU64 => {
                binary_op!(ctx -> u64: /);
            },
            OpCode::FDiv32 => {
                binary_op!(ctx -> f32: /);
            },
            OpCode::FDiv64 => {
                binary_op!(ctx -> f64: /);
            },
            OpCode::IModI32 => {
                binary_op!(ctx -> i32: %);
            },
            OpCode::IModI64 => {
                binary_op!(ctx -> i64: %);
            },
            OpCode::IModU32 => {
                binary_op!(ctx -> u32: %);
            },
            OpCode::IModU64 => {
                binary_op!(ctx -> u64: %);
            },
            OpCode::FMod32 => {
                binary_op!(ctx -> f32: %);
            },
            OpCode::FMod64 => {
                binary_op!(ctx -> f64: %);
            },
            OpCode::IAnd32 => {
                binary_op!(ctx -> u32: &);
            },
            OpCode::IAnd64 => {
                binary_op!(ctx -> u64: &);
            },
            OpCode::IOr32 => {
                binary_op!(ctx -> u32: |);
            },
            OpCode::IOr64 => {
                binary_op!(ctx -> u64: |);
            },
            OpCode::IXor32 => {
                binary_op!(ctx -> u32: ^);
            },
            OpCode::IXor64 => {
                binary_op!(ctx -> u64: ^);
            },
            OpCode::INot32 => {
                unary_op!(ctx -> u32: !);
            },
            OpCode::INot64 => {
                unary_op!(ctx -> u64: !);
            },
            OpCode::ILsh8 => {
                binary_op!(ctx -> u8, u32: <<);
            },
            OpCode::ILsh16 => {
                binary_op!(ctx -> u16, u32: <<);
            },
            OpCode::ILsh32 => {
                binary_op!(ctx -> u32, u32: <<);
            },
            OpCode::ILsh64 => {
                binary_op!(ctx -> u64, u32: <<);
            },
            OpCode::IRshU8 => {
                binary_op!(ctx -> u8, u32: >>);
            },
            OpCode::IRshU16 => {
                binary_op!(ctx -> u16, u32: >>);
            },
            OpCode::IRshU32 => {
                binary_op!(ctx -> u32, u32: >>);
            },
            OpCode::IRshU64 => {
                binary_op!(ctx -> u64, u32: >>);
            },
            OpCode::IRshI8 => {
                binary_op!(ctx -> i8, u32: >>);
            },
            OpCode::IRshI16 => {
                binary_op!(ctx -> i16, u32: >>);
            },
            OpCode::IRshI32 => {
                binary_op!(ctx -> i32, u32: >>);
            },
            OpCode::IRshI64 => {
                binary_op!(ctx -> i64, u32: >>);
            },
            OpCode::ILrot8 => {
                binary_op!(ctx -> u8, u32: rotate_left);
            },
            OpCode::ILrot16 => {
                binary_op!(ctx -> u16, u32: rotate_left);
            },
            OpCode::ILrot32 => {
                binary_op!(ctx -> u32, u32: rotate_left);
            },
            OpCode::ILrot64 => {
                binary_op!(ctx -> u64, u32: rotate_left);
            },
            OpCode::IRrot8 => {
                binary_op!(ctx -> u8, u32: rotate_right);
            },
            OpCode::IRrot16 => {
                binary_op!(ctx -> u16, u32: rotate_right);
            },
            OpCode::IRrot32 => {
                binary_op!(ctx -> u32, u32: rotate_right);
            },
            OpCode::IRrot64 => {
                binary_op!(ctx -> u64, u32: rotate_right);
            },
            OpCode::RegAddr => {
                let start = ctx.regs as usize;
                let idx = ctx.decoder.read::<u16>() as usize;
                let reg = ctx.read_reg();
                reg.set(start + idx);
            }
            OpCode::U32ToF64 => {
                conv!(ctx -> u32 as f64);
            },
            OpCode::I32ToF64 => {
                conv!(ctx -> i32 as f64);
            },
            OpCode::U64ToF32 => {
                conv!(ctx -> u64 as f32);
            },
            OpCode::I64ToF32 => {
                conv!(ctx -> i64 as f32);
            },
            OpCode::F32ToU64 => {
                conv!(ctx -> f32 as u64);
            },
            OpCode::F64ToU32 => {
                conv!(ctx -> f64 as u32);
            },
            OpCode::F32ToI64 => {
                conv!(ctx -> f32 as i64);
            },
            OpCode::F64ToI32 => {
                conv!(ctx -> f64 as i32);
            },
            OpCode::Move => {
                let aux = ctx.reg_as::<u64>();
                ctx.read_reg().set(aux);
            }
        }
    }
}