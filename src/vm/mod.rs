#![feature(explicit_tail_calls)]
#![feature(decl_macro)]
#![feature(portable_simd)]
#![feature(variant_count)]
#![feature(ergonomic_clones)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]

use crate::structures::decode::{Decodee, Decoder};
use crate::structures::header::BytecodeHeader;
use clap::Parser;
use eva::core::OpCode;
use memmap2::Mmap;
use memmap2::MmapOptions;
use size::Size;
use std::fs::File;
use std::path::PathBuf;
use structures::registers::*;

mod structures;

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
        self.get_reg(self.decoder.read())
    }

    pub fn reg_as<T: Registrant>(&mut self) -> T {
        self.get_reg(self.decoder.read()).get::<T>()
    }
}

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    run: PathBuf,
    #[clap(short, long, default_value = "16MiB", help = "The maximum virtual register stack size")]
    reg_size: Size,
}

fn main() -> u32 { unsafe {
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
    let header: BytecodeHeader = BytecodeHeader::read(&mut decoder).clone();

    let x64 = header.is_x64();
    if size_of::<usize>() < 8 && x64 {
        panic!("Tried to run x64 program on x86 system!")
    }

    let ctx: VmCtx = VmCtx {
        decoder,
        regs: regs as *mut Register,
    };



    start(ctx)
}}

fn start(mut ctx: VmCtx) -> u32 {
    loop {
        match ctx.decoder.read::<OpCode>() {
            OpCode::Nop => { continue; }
            OpCode::Const8 => {
                ctx.read_reg().set(ctx.decoder.read::<u8>());
            }
            OpCode::Const16 => {
                ctx.read_reg().set(ctx.decoder.read::<u16>());
            }
            OpCode::Const32 => {
                ctx.read_reg().set(ctx.decoder.read::<u32>());
            }
            OpCode::Const64 => {
                ctx.read_reg().set(ctx.decoder.read::<u64>());
            }
            OpCode::SignExt64 => {
                ctx.read_reg().set(ctx.reg_as::<i32>() as i64);
            },
            OpCode::SignExt32 => {
                ctx.read_reg().set(ctx.reg_as::<i16>() as i32);
            },
            OpCode::SignExt16 => {
                ctx.read_reg().set(ctx.reg_as::<i8>() as i16);
            },
            OpCode::ZeroExt64 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() as u64);
            },
            OpCode::ZeroExt32 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() as u32);
            },
            OpCode::ZeroExt16 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() as u16);
            },
            OpCode::Truncat32 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() as u32);
            },
            OpCode::Truncat16 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() as u16);
            },
            OpCode::Truncat8 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() as u8);
            },
            OpCode::U32ToF32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() as f32);
            },
            OpCode::I32ToF32 => {
                ctx.read_reg().set(ctx.reg_as::<i32>() as f32);
            },
            OpCode::U64ToF64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() as f64);
            },
            OpCode::I64ToF64 => {
                ctx.read_reg().set(ctx.reg_as::<i64>() as f64);
            },
            OpCode::F32ToU32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() as u32);
            },
            OpCode::F64ToU64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() as u64);
            },
            OpCode::F32ToI32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() as i32);
            },
            OpCode::F64ToI64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() as i64);
            },
            OpCode::FProm => {
                ctx.read_reg().set(ctx.reg_as::<f32>() as f64);
            },
            OpCode::FDemo => {
                ctx.read_reg().set(ctx.reg_as::<f64>() as f32);
            },
            OpCode::IAdd8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() + ctx.reg_as::<u8>());
            },
            OpCode::IAdd16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() + ctx.reg_as::<u16>());
            },
            OpCode::IAdd32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() + ctx.reg_as::<u32>());
            },
            OpCode::IAdd64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() + ctx.reg_as::<u64>());
            },
            OpCode::ISub8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>()  -ctx.reg_as::<u8>());
            },
            OpCode::ISub16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() - ctx.reg_as::<u16>());
            },
            OpCode::ISub32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() - ctx.reg_as::<u32>());
            },
            OpCode::ISub64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() - ctx.reg_as::<u64>());
            },
            OpCode::FAdd32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() + ctx.reg_as::<f32>());
            },
            OpCode::FAdd64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() + ctx.reg_as::<f64>());
            },
            OpCode::FSub32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() - ctx.reg_as::<f32>());
            },
            OpCode::FSub64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() - ctx.reg_as::<f64>());
            },
            OpCode::IMul8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() * ctx.reg_as::<u8>());
            },
            OpCode::IMul16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() * ctx.reg_as::<u16>());
            },
            OpCode::IMul32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() * ctx.reg_as::<u32>());
            },
            OpCode::IMul64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() * ctx.reg_as::<u64>());
            },
            OpCode::FMul32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() * ctx.reg_as::<f32>());
            },
            OpCode::FMul64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() * ctx.reg_as::<f64>());
            },
            OpCode::IDivI8  => {
                ctx.read_reg().set(ctx.reg_as::<i8>() / ctx.reg_as::<i8>());
            },
            OpCode::IDivI16 => {
                ctx.read_reg().set(ctx.reg_as::<i16>() / ctx.reg_as::<i16>());
            },
            OpCode::IDivI32 => {
                ctx.read_reg().set(ctx.reg_as::<i32>() / ctx.reg_as::<i32>());
            },
            OpCode::IDivI64 => {
                ctx.read_reg().set(ctx.reg_as::<i64>() / ctx.reg_as::<i64>());
            },
            OpCode::IDivU8  => {
                ctx.read_reg().set(ctx.reg_as::<u8>() / ctx.reg_as::<u8>());
            },
            OpCode::IDivU16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() / ctx.reg_as::<u16>());
            },
            OpCode::IDivU32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() / ctx.reg_as::<u32>());
            },
            OpCode::IDivU64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() / ctx.reg_as::<u64>());
            },
            OpCode::FDiv32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() / ctx.reg_as::<f32>());
            },
            OpCode::FDiv64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() / ctx.reg_as::<f64>());
            },
            OpCode::IModI8 => {
                ctx.read_reg().set(ctx.reg_as::<i8>() % ctx.reg_as::<i8>());
            },
            OpCode::IModI16 => {
                ctx.read_reg().set(ctx.reg_as::<i16>() % ctx.reg_as::<i16>());
            },
            OpCode::IModI32 => {
                ctx.read_reg().set(ctx.reg_as::<i32>() % ctx.reg_as::<i32>());
            },
            OpCode::IModI64 => {
                ctx.read_reg().set(ctx.reg_as::<i64>() % ctx.reg_as::<i64>());
            },
            OpCode::IModU8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() % ctx.reg_as::<u8>());
            },
            OpCode::IModU16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() % ctx.reg_as::<u16>());
            },
            OpCode::IModU32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() % ctx.reg_as::<u32>());
            },
            OpCode::IModU64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() % ctx.reg_as::<u64>());
            },
            OpCode::FMod32 => {
                ctx.read_reg().set(ctx.reg_as::<f32>() % ctx.reg_as::<f32>());
            },
            OpCode::FMod64 => {
                ctx.read_reg().set(ctx.reg_as::<f64>() % ctx.reg_as::<f64>());
            },
            OpCode::IAnd8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() & ctx.reg_as::<u8>());
            },
            OpCode::IAnd16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() & ctx.reg_as::<u16>());
            },
            OpCode::IAnd32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() & ctx.reg_as::<u32>());
            },
            OpCode::IAnd64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() & ctx.reg_as::<u64>());
            },
            OpCode::IOr8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() | ctx.reg_as::<u8>());
            },
            OpCode::IOr16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() | ctx.reg_as::<u16>());
            },
            OpCode::IOr32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() | ctx.reg_as::<u32>());
            },
            OpCode::IOr64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() | ctx.reg_as::<u64>());
            },
            OpCode::IXor8 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() ^ ctx.reg_as::<u64>());
            },
            OpCode::IXor16 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() ^ ctx.reg_as::<u64>());
            },
            OpCode::IXor32 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() ^ ctx.reg_as::<u64>());
            },
            OpCode::IXor64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() ^ ctx.reg_as::<u64>());
            },
            OpCode::INot8 => {
                ctx.read_reg().set(!ctx.reg_as::<u8>());
            },
            OpCode::INot16 => {
                ctx.read_reg().set(!ctx.reg_as::<u16>());
            },
            OpCode::INot32 => {
                ctx.read_reg().set(!ctx.reg_as::<u32>());
            },
            OpCode::INot64 => {
                ctx.read_reg().set(!ctx.reg_as::<u64>());
            },
            OpCode::ILsh8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() << ctx.reg_as::<u32>());
            },
            OpCode::ILsh16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() << ctx.reg_as::<u32>());
            },
            OpCode::ILsh32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() << ctx.reg_as::<u32>());
            },
            OpCode::ILsh64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() << ctx.reg_as::<u32>());
            },
            OpCode::IRshU8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshU16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshU32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshU64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshI8 => {
                ctx.read_reg().set(ctx.reg_as::<i8>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshI16 => {
                ctx.read_reg().set(ctx.reg_as::<i16>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshI32 => {
                ctx.read_reg().set(ctx.reg_as::<i32>() >> ctx.reg_as::<u32>());
            },
            OpCode::IRshI64 => {
                ctx.read_reg().set(ctx.reg_as::<i64>() >> ctx.reg_as::<u32>());
            },
            OpCode::ILrot8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>().rotate_left(ctx.reg_as::<u32>()));
            },
            OpCode::ILrot16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>().rotate_left(ctx.reg_as::<u32>()));
            },
            OpCode::ILrot32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>().rotate_left(ctx.reg_as::<u32>()));
            },
            OpCode::ILrot64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>().rotate_left(ctx.reg_as::<u32>()));
            },
            OpCode::IRrot8 => {
                ctx.read_reg().set(ctx.reg_as::<u8>().rotate_right(ctx.reg_as::<u32>()));
            },
            OpCode::IRrot16 => {
                ctx.read_reg().set(ctx.reg_as::<u16>().rotate_right(ctx.reg_as::<u32>()));
            },
            OpCode::IRrot32 => {
                ctx.read_reg().set(ctx.reg_as::<u32>().rotate_right(ctx.reg_as::<u32>()));
            },
            OpCode::IRrot64 => {
                ctx.read_reg().set(ctx.reg_as::<u64>().rotate_right(ctx.reg_as::<u32>()));
            },
            OpCode::RegAddr => {
                ctx.read_reg().set(ctx.regs as usize + ctx.decoder.read::<u16>() as usize);
            }
        }
    }
}