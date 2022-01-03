#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

use std::cmp::min;
use std::fmt;
use std::fmt::{Display};
use thiserror::Error;
use anyhow::{Context,Result};

use bitutils::sign_extend32;
use std::convert::TryInto;
use std::io::Read;
use std::path::Path;
use std::fs::{File,metadata};

mod decode;
use decode::{decode, InstructionBits};

mod memory;
use memory::Memory;

static XLEN: usize = 32;
type uXLEN = u32;
static ELEN: usize = 32;
type uELEN = u32;
static VLEN: usize = 128; // ELEN * 4
type uVLEN = u128;

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
enum Sew {
    e8,
    e16,
    e32,
    e64
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
enum Lmul {
    eEighth,
    eQuarter,
    eHalf,
    e1,
    e2,
    e4,
    e8
}

fn elements_in(s: Sew, l: Lmul) -> u32 {
    val_times_lmul_over_sew(VLEN as u32, s, l)
}
fn val_times_lmul_over_sew(x: u32, s: Sew, l: Lmul) -> u32 {
    let mut bits_per_group: u32 = x;
    match l {
        Lmul::eEighth => {
            bits_per_group /= 8;
        },
        Lmul::eQuarter => {
            bits_per_group /= 4;
        },
        Lmul::eHalf => {
            bits_per_group /= 2;
        },
        Lmul::e1 => {},
        Lmul::e2 => {
            bits_per_group *= 2;
        },
        Lmul::e4 => {
            bits_per_group *= 4;
        },
        Lmul::e8 => {
            bits_per_group *= 8;
        },
    };
    
    bits_per_group / match s {
        Sew::e8 => 8,
        Sew::e16 => 16,
        Sew::e32 => 32,
        Sew::e64 => 64,
    }
}

#[derive(Debug,PartialEq,Eq)]
enum RunState {
    Stopped,
    Running
}

static register_names: [&str; 32] = [
    "zero", "ra", "sp", "gp",
    "tp", "t0", "t1", "t2",
    "fp", "s1", "a0", "a1",
    "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3",
    "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6"
];

struct Processor {
    run_state: RunState,
    memory: Memory,

    pc: uXLEN,

    sreg: [uXLEN; 32],
    vreg: [uVLEN; 32],

    sew: Sew,
    lmul: Lmul,
    vl: u32,
    vtype_reg: u32,
    // VMA, VTA not required.
    // agnostic = undisturbed || overwrite with ones, so just assume undisturbed
    // vma: bool,
    // vta: bool,
}

impl Processor {
    fn new(mem: Memory) -> Processor {
        Processor {
            run_state: RunState::Stopped,
            memory: mem,

            pc: 0,

            sreg: [0; 32],
            vreg: [0; 32],

            sew: Sew::e64,
            lmul: Lmul::e1,
            vl: 0,
            vtype_reg: 0,
        }
    }

    fn reset(&mut self) {
        self.run_state = RunState::Stopped;

        self.pc = 0;
        self.sreg = [0; 32];
        self.vreg = [0; 32];

        self.sew = Sew::e64;
        self.lmul = Lmul::e1;
        self.vl = 0;
        self.vtype_reg = 0;
    }

    fn exec_step(&mut self) -> Result<()> {
        self.run_state = RunState::Running;

        // self.dump();

        let inst_bits = self.memory.load_u32(self.pc).context("Couldn't load next InstructionBits")?;
        // dbg!(format!("0x{:08x}", self.pc));
        // dbg!(format!("{:08x}", inst_bits));
        let (opcode, inst) = decode(inst_bits)?;

        // println!("executing {:?} {:?}", opcode, inst);

        let mut next_pc = self.pc + 4;

        use decode::Opcode::*;
        match (opcode, inst) {
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                self.sreg[rd as usize] = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(self.memory.load_u8(addr)? as u32, 8) as u32, // LB
                    0b001 => sign_extend32(self.memory.load_u16(addr)? as u32, 16) as u32, // LH
                    0b010 => self.memory.load_u32(addr)?, // LW
                    // LBU, LHU don't sign-extend
                    0b100 => self.memory.load_u8(addr)? as u32, // LBU
                    0b101 => self.memory.load_u16(addr)? as u32, // LBU

                    _ => bail!("Unexpected Load funct3 {:03b}", funct3)
                };
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                match funct3 {
                    0b000 => self.memory.store_u8(addr, (self.sreg[rs2 as usize] & 0xFF) as u8)?,
                    0b001 => self.memory.store_u16(addr, (self.sreg[rs2 as usize] & 0xFFFF) as u16)?,
                    0b010 => self.memory.store_u32(addr, self.sreg[rs2 as usize])?,
                    
                    _ => bail!("Unexpected Store funct3 {:03b}", funct3)
                };
            }

            (OpImm, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let input = self.sreg[rs1 as usize];
                self.sreg[rd as usize] = match (imm, funct3) {
                    (imm, 0b000) => input.wrapping_add(imm), // ADDI
                    (imm, 0b010) => if (input as i32) < (imm as i32) { 1 } else { 0 }, // SLTI
                    (imm, 0b011) => if input < imm { 1 } else { 0 }, // SLTU
                    (imm, 0b100) => input ^ imm, // XORI
                    (imm, 0b110) => input | imm, // ORI
                    (imm, 0b111) => input & imm, // ANDI

                    (shamt, 0b001) => input << shamt, // SLLI
                    (imm, 0b101) => {
                        // Check top bits of imm to see if arithmetic or logical
                        // shamt = bottom-five-bits
                        let shamt = imm & 0x1F;
                        if ((imm >> 10) & 1) == 1 {
                            // SRAI
                            // input as i32 => shift will be arithmetic
                            // cast back to u32 afterwards
                            ((input as i32) >> shamt) as u32
                        } else {
                            // SRLI
                            input >> shamt
                        }
                    }

                    _ => bail!("Unexpected OpImm funct3 {:03b}", funct3)
                };
            }

            (Op, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                const ALT: u8 = 0b0100000;
                let x = self.sreg[rs1 as usize];
                let y = self.sreg[rs2 as usize];
                self.sreg[rd as usize] = match (funct7, funct3) {
                    (0, 0b000) => x.wrapping_add(y), // ADD
                    (ALT, 0b000) => x.wrapping_sub(y), // SUB

                    (0, 0b001) => x << y, // SLL
                    
                    (0, 0b010) => if (x as i32) < (y as i32) { 1 } else { 0 }, // SLT
                    (0, 0b011) => if x < y { 1 } else { 0 }, // SLTU

                    (0, 0b100) => x ^ y, // XOR 
                    (0, 0b101) => x >> y, // SRL
                    (ALT, 0b101) => ((x as i32) >> y) as u32, // SRA
                    (0, 0b110) => x | y, // OR
                    (0, 0b111) => x & y, // AND

                    _ => bail!("Unexpected Op funct7/3: {:07b}, {:03b}", funct7, funct3)
                };
            }

            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = imm + self.pc;
                self.sreg[rd as usize] = addr;
            }

            (LoadUpperImm, InstructionBits::UType{rd, imm}) => {
                self.sreg[rd as usize] = imm;
            }

            (JumpAndLink, InstructionBits::JType{rd, imm}) => {
                self.sreg[rd as usize] = self.pc + 4;
                next_pc = self.pc.wrapping_add(imm);
            }
            (JumpAndLinkRegister, InstructionBits::IType{rd, funct3: 0b000, rs1, imm}) => {
                next_pc = self.sreg[rs1 as usize].wrapping_add(imm);
                // Unset bottom bit
                next_pc = next_pc & 0xFFFF_FFFE;

                self.sreg[rd as usize] = self.pc + 4;
            }

            (Branch, InstructionBits::BType{funct3, rs1, rs2, imm}) => {
                let src1 = self.sreg[rs1 as usize];
                let src2 = self.sreg[rs2 as usize];

                let take_branch = match funct3 {
                    0b000 => src1 == src2, // BEQ
                    0b001 => src1 != src2, // BNE
                    0b100 => (src1 as i32) < (src2 as i32), // BLT
                    0b101 => (src1 as i32) > (src2 as i32), // BGE
                    0b110 => (src1 as u32) < (src2 as u32), // BLTU
                    0b111 => (src1 as u32) > (src2 as u32), // BGEU

                    _ => bail!("Unexpected funct3 for branch {:03b}", funct3)
                };

                if take_branch {
                    next_pc = self.pc.wrapping_add(imm);
                }
            }

            (Vector, InstructionBits::VType{rd, funct3, rs1, rs2, vm, funct6, zimm11, zimm10}) => {
                match funct3 {
                    0b111 => {
                        // Configuration
                        let avl = match bits!(inst_bits, 30:31) {
                            0b00 | 0b01 | 0b10 => { // vsetvli, vsetvl
                                if rs1 != 0 {
                                    self.sreg[rs1 as usize]
                                } else {
                                    if rd != 0 {
                                        bail!("vsetvl{{i}} called with rd != 0, rs1 == 0, which requires VMAX. Haven't thought about that yet.");
                                        u32::MAX
                                    } else {
                                        self.vl
                                    }
                                }
                            } ,
                            0b11 => { // vsetivli
                                rs1 as u32 // Use rs1 as a 5-bit immediate
                            },
                            invalid => panic!("impossible top 2 bits {:2b}", invalid)
                        };
                        let vtype = match bits!(inst_bits, 30:31) {
                            0b00 | 0b01 => {
                                // vsetvli
                                zimm11 as u32
                            },
                            0b11 => {
                                // vsetivli
                                zimm10 as u32
                            },
                            0b10 => {
                                self.sreg[rs2 as usize] 
                            },
                            invalid => panic!("impossible top 2 bits {:2b}", invalid)
                        };

                        let req_sew = match bits!(vtype, 3:5) {
                            0b000 => Sew::e8,
                            0b001 => Sew::e16,
                            0b010 => Sew::e32,
                            0b011 => Sew::e64,

                            invalid => bail!("Bad vtype - invalid SEW selected {:b}", invalid)
                        };
                        let req_lmul = match bits!(vtype, 0:2) {
                            0b000 => Lmul::e1,
                            0b001 => Lmul::e2,
                            0b010 => Lmul::e4,
                            0b011 => Lmul::e8,

                            0b101 => Lmul::eEighth,
                            0b110 => Lmul::eQuarter,
                            0b111 => Lmul::eHalf,

                            invalid => bail!("Bad vtype - invalid Lmul selected {:b}", invalid),
                        };

                        let vector_elements = elements_in(req_sew, req_lmul);
                        let vtype_valid = vector_elements > 0 && 
                            req_sew != Sew::e64 && match req_lmul {
                            Lmul::eHalf | Lmul::eQuarter | Lmul::eEighth => false,
                            _ => true
                        };
                        if vtype_valid {
                            self.vtype_reg = vtype;
                            self.sew = req_sew;
                            self.lmul = req_lmul;
                            self.vl = min(vector_elements, avl);

                            self.sreg[rd as usize] = self.vl;
                        } else {
                            self.vtype_reg = 0x8000_0000;
                            bail!("Valid but unsupported vtype: {:b} -> {:?} {:?} elems {:}", vtype, req_sew, req_lmul, vector_elements);
                        }
                    }

                    _ => bail!("Vector arithmetic currently not supported")
                }
            }

            (LoadFP, InstructionBits::FLdStType{rd, width, rs1, rs2, funct7, vm, mew, mop, nf}) => {
                if mew { bail!("LoadFP with mew = 1 is reserved") }

                let eew = match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP uses width for actual floats, not vectors"),
                    0b1000..=0b1111 => bail!("LoadFP using reserved width {}", width),

                    0b0000 => 8,
                    0b0101 => 16,
                    0b0110 => 32,
                    0b0111 => 64,

                    _ => bail!("LoadFP has impossible width {}", width)
                };

                let emul_times_8 = val_times_lmul_over_sew(eew * 8, self.sew, self.lmul);

                if emul_times_8 > 64 || emul_times_8 <= 1 {
                    bail!("emul * 8 too big or too small: {}", emul_times_8);
                }

                let nf = nf + 1;

                if (emul_times_8 * nf as u32) > 64 {
                    bail!("emul * nf too big: {}", emul_times_8 * (nf as u32) / 8);
                }

                let mop = match mop {
                    0b00 => Mop::UnitStride,
                    0b10 => Mop::Strided(self.sreg[rs2 as usize]),
                    0b01 | 0b11 => Mop::Indexed, // Unordered | Ordered

                    _ => panic!("impossible mop bits {:2b}", mop)
                };

                let base_addr = self.sreg[rs1 as usize];

                match mop {
                    Mop::UnitStride => {
                        let lumop = match rs2 {
                            0b00000 => UnitStrideLoadOp::Load,
                            0b01000 => UnitStrideLoadOp::WholeRegister,
                            0b01011 => UnitStrideLoadOp::ByteMaskLoad,
                            0b10000 => UnitStrideLoadOp::FaultOnlyFirst,
    
                            _ => bail!("invalid unit stride {:05b}", rs2)
                        };
                        bail!("Vector Load not fully implemented yet")
                    }
                    Mop::Strided(stride) => {
                        bail!("Vector Load not fully implemented yet")
                    }
                    Mop::Indexed => {
                        bail!("Vector Load not fully implemented yet")
                    }
                }
                
            }

            _ => bail!("Unexpected opcode/InstructionBits pair ({:?}, {:?})", opcode, inst)
        }

        self.sreg[0] = 0;

        if next_pc % 4 != 0 {
            bail!("PC was set to a misaligned value {:?}", next_pc);
        }
        self.pc = next_pc;

        Ok(())
    }

    fn dump(&self) {
        println!("{:?}\npc: 0x{:08x}", self.run_state, self.pc);
        for i in 0..32 {
            println!("x{} = {} = 0x{:08x}", i, register_names[i], self.sreg[i]);
        }
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
        println!("sew: {:?}\nlmul: {:?}\nvl: {}\nvtype_reg: {:08x}", self.sew, self.lmul, self.vl, self.vtype_reg);
    }
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Mop {
    UnitStride,
    Strided(u32),
    Indexed,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum UnitStrideLoadOp {
    Load,
    WholeRegister,
    ByteMaskLoad,
    FaultOnlyFirst
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum UnitStrideStoreOp {
    Store,
    WholeRegister,
    ByteMaskStore
}

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("risc-v-v-lite")
        .version("1.0")
        .author("Samuel S. <popgoestoast@gmail.com>")
        .about("Simplistic RISC-V emulator for Vector extension")
        .arg(
            Arg::with_name("memory_bin")
            .required(true)
            .index(1)
        )
        .get_matches();

    let memory_bin = matches.value_of("memory_bin").unwrap();
    let mem = Memory::new_from_file(memory_bin, 640_000);
    let mut processor = Processor::new(mem);

    loop {
        let res = processor.exec_step();

        match res {
            Err(e) => {
                processor.dump();
                println!("Encountered error: {:#}", e);
                break
            },
            Ok(()) => {}
        }
        if processor.run_state == RunState::Stopped {
            break
        }
    }
}
