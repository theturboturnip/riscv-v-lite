#[macro_use]
extern crate bitutils;

use bitutils::sign_extend32;
use std::convert::TryInto;
use std::io::Read;
use std::path::Path;
use std::fs::{File,metadata};

mod bitpattern;
use bitpattern::BitPattern;

mod instrs;
use instrs::{decode, Opcode, Instruction};

struct Memory {
    _data: Vec<u8>,
}

impl Memory {
    fn new_from_file(path_s: &str, pad_memory_to: u64) -> Memory {
        let path = Path::new(path_s);

        let mut f = File::open(&path).expect("no file found");
        let metadata = metadata(&path).expect("unable to read metadata");
        assert_eq!(metadata.len() % 4, 0);
        assert_eq!(metadata.len() <= pad_memory_to, true);
        
        let mut buffer = vec![0; pad_memory_to as usize];
        f.read(&mut buffer).expect("buffer overflow");

        // let mut buffer32 = vec![0; metadata.len() as usize / 4];
        // for i in 0..buffer32.len() {
        //     buffer32[i] = 
        //         ((buffer[i * 4 + 3] as u32) << 24) |
        //         ((buffer[i * 4 + 2] as u32) << 16) |
        //         ((buffer[i * 4 + 1] as u32) << 8)  |
        //         ((buffer[i * 4 + 0] as u32) << 0)
        //     ;
        // }

        Memory {
            _data: buffer,
        }
    }

    fn len(&self) -> usize {
        self._data.len()
    }

    fn load_u32(&self, addr: u32) -> Option<u32> {
        let addr: usize = addr.try_into().unwrap();
        if addr & 0x03 == 0 // Aligned address
            && addr + 3 < self._data.len() { // In-bounds
            Some(
                ((self._data[addr+3] as u32) << 24) | 
                ((self._data[addr+2] as u32) << 16) | 
                ((self._data[addr+1] as u32) << 8) | 
                ((self._data[addr+0] as u32))
            )
        } else {
            None
        }
    }
    fn load_u16(&self, addr: u32) -> Option<u16> {
        let addr: usize = addr.try_into().unwrap();
        if addr & 0x01 == 0 // Aligned address
            && addr + 1 < self._data.len() { // In-bounds
                Some(
                    ((self._data[addr+1] as u16) << 8) | 
                    ((self._data[addr+0] as u16))
                )
        } else {
            None
        }
    }
    fn load_u8(&self, addr: u32) -> Option<u8> {
        let addr: usize = addr.try_into().unwrap();
        if addr < self._data.len() { // In-bounds
            Some(self._data[addr])
        } else {
            None
        }
    }

    fn store_u32(&mut self, addr: u32, data: u32) -> Option<()> {
        let addr: usize = addr.try_into().unwrap();
        if addr == 0xf000_0000 {
            // Special case
            println!("RESULT = 0x{:08x} = {}", data, data);
            Some(())
        } else if addr & 0x03 == 0 // Aligned address
            && addr + 3 < self._data.len() { // In-bounds

            self._data[addr + 3] = ((data >> 24) & 0xff).try_into().unwrap();
            self._data[addr + 2] = ((data >> 16) & 0xff).try_into().unwrap();
            self._data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self._data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Some(())
        } else {
            None
        }
    }

    fn store_u16(&mut self, addr: u32, data: u16) -> Option<()> {
        let addr: usize = addr.try_into().unwrap();
        if addr & 0x01 == 0 // Aligned address
            && addr + 1 < self._data.len() { // In-bounds

            self._data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self._data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Some(())
        } else {
            None
        }
    }

    fn store_u8(&mut self, addr: u32, data: u8) -> Option<()> {
        let addr: usize = addr.try_into().unwrap();
        if addr < self._data.len() { // In-bounds
            self._data[addr] = data;
            Some(())
        } else {
            None
        }
    }
}

static XLEN: usize = 32;
type uXLEN = u32;
static ELEN: usize = 32;
type uELEN = u32;
static VLEN: usize = 128; // ELEN * 4
type uVLEN = u128;

#[derive(Debug,PartialEq,Eq)]
enum Sel {
    e8,
    e16,
    e32,
    e64
}

#[derive(Debug,PartialEq,Eq)]
enum Lmul {
    eEighth,
    eQuarter,
    eHalf,
    e1,
    e2,
    e4,
    e8
}

#[derive(Debug,PartialEq,Eq)]
enum ProcState {
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
    state: ProcState,
    memory: Memory,

    pc: uXLEN,

    sreg: [uXLEN; 32],
    vreg: [uVLEN; 32],

    sel: Sel,
    lmul: Lmul,
    // VMA, VTA not required.
    // agnostic = undisturbed || overwrite with ones, so just assume undisturbed
    // vma: bool,
    // vta: bool,
}

impl Processor {
    fn new(mem: Memory) -> Processor {
        Processor {
            state: ProcState::Stopped,
            memory: mem,

            pc: 0,

            sreg: [0; 32],
            vreg: [0; 32],

            sel: Sel::e64,
            lmul: Lmul::e1,
        }
    }

    fn reset(&mut self) {
        self.state = ProcState::Stopped;

        self.pc = 0;
        self.sreg = [0; 32];
        self.vreg = [0; 32];

        self.sel = Sel::e64;
        self.lmul = Lmul::e1;
    }

    fn exec_step(&mut self) {
        self.state = ProcState::Running;

        self.dump();

        let inst_bits = self.memory.load_u32(self.pc).expect("Couldn't load next instruction");
        // dbg!(format!("0x{:08x}", self.pc));
        dbg!(format!("{:08x}", inst_bits));
        let (opcode, inst) = decode(inst_bits);

        println!("executing {:?} {:?}", opcode, inst);

        let mut next_pc = self.pc + 4;

        use instrs::Opcode::*;
        match (opcode, inst) {
            (Load, Instruction::IType{rd, funct3, rs1, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                self.sreg[rd as usize] = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(self.memory.load_u8(addr).unwrap() as u32, 8) as u32, // LB
                    0b001 => sign_extend32(self.memory.load_u16(addr).unwrap() as u32, 16) as u32, // LH
                    0b010 => self.memory.load_u32(addr).unwrap(), // LW
                    // LBU, LHU don't sign-extend
                    0b100 => self.memory.load_u8(addr).unwrap() as u32, // LBU
                    0b101 => self.memory.load_u16(addr).unwrap() as u32, // LBU

                    _ => panic!("Unexpected Load funct3 {:03b}", funct3)
                };
            }
            (Store, Instruction::SType{funct3, rs1, rs2, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                match funct3 {
                    0b000 => self.memory.store_u8(addr, (self.sreg[rs2 as usize] & 0xFF) as u8).unwrap(),
                    0b001 => self.memory.store_u16(addr, (self.sreg[rs2 as usize] & 0xFFFF) as u16).unwrap(),
                    0b010 => self.memory.store_u32(dbg!(addr), self.sreg[rs2 as usize]).unwrap(),
                    
                    _ => panic!("Unexpected Store funct3 {:03b}", funct3)
                };
            }

            (OpImm, Instruction::IType{rd, funct3, rs1, imm}) => {
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

                    _ => panic!("Unexpected OpImm funct3 {:03b}", funct3)
                };
            }

            (Op, Instruction::RType{rd, funct3, rs1, rs2, funct7}) => {
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

                    _ => panic!("Unexpected Op funct7/3: {:07b}, {:03b}", funct7, funct3)
                };
            }

            (AddUpperImmPC, Instruction::UType{rd, imm}) => {
                let addr = imm + self.pc;
                self.sreg[rd as usize] = addr;
            }

            (LoadUpperImm, Instruction::UType{rd, imm}) => {
                self.sreg[rd as usize] = imm;
            }

            (JumpAndLink, Instruction::JType{rd, imm}) => {
                self.sreg[rd as usize] = self.pc + 4;
                next_pc = self.pc.wrapping_add(imm);
            }
            (JumpAndLinkRegister, Instruction::IType{rd, funct3: 0b000, rs1, imm}) => {
                next_pc = self.sreg[rs1 as usize].wrapping_add(imm);
                // Unset bottom bit
                next_pc = next_pc & 0xFFFF_FFFE;

                self.sreg[rd as usize] = self.pc + 4;
            }

            _ => panic!("Unexpected opcode/instruction pair ({:?}, {:?})", opcode, inst)
        }

        self.sreg[0] = 0;

        if next_pc % 4 != 0 {
            panic!("PC was set to a misaligned value {:?}", next_pc);
        }
        self.pc = next_pc;
    }

    fn dump(&self) {
        println!("{:?}\npc:0x{:08x}", self.state, self.pc);
        for i in 0..32 {
            println!("x{} = {} = 0x{:08x}", i, register_names[i], self.sreg[i]);
        }
        // for i in 0..32 {
        //     println!("v{} = 0x{:032x}", i, self.vreg[i]);
        // }
        println!("sel: {:?}\nlmul: {:?}", self.sel, self.lmul);
    }
}

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("risc-v-v-lite")
        .version("1.0")
        .author("Samuel S. <popgoestoast@gmail.com>")
        .about("simplistic RISC-V emulator for Vector extension")
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
        processor.exec_step();

        if processor.state == ProcState::Stopped {
            break
        }
    }
}
