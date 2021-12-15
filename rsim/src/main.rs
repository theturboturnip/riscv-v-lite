use std::io::Read;
use std::path::Path;
use std::fs::{File,metadata};

mod bitpattern;
use bitpattern::BitPattern;

struct Memory {
    _data: Vec<u32>,
    _byte_len: usize,
}

impl Memory {
    fn new_from_file(path_s: &str) -> Memory {
        let path = Path::new(path_s);

        let mut f = File::open(&path).expect("no file found");
        let metadata = metadata(&path).expect("unable to read metadata");
        assert_eq!(metadata.len() % 4, 0);
        
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        let mut buffer32 = vec![0; metadata.len() as usize / 4];
        for i in 0..buffer32.len() {
            buffer32[i] = 
                ((buffer[i * 4 + 3] as u32) << 24) |
                ((buffer[i * 4 + 2] as u32) << 16) |
                ((buffer[i * 4 + 1] as u32) << 8)  |
                ((buffer[i * 4 + 0] as u32) << 0)
            ;
        } 

        Memory {
            _data: buffer32,
            _byte_len: buffer.len(),
        }
    }

    fn len(&self) -> usize {
        self._byte_len
    }

    fn load(&self, addr: usize) -> Option<u32> {
        if addr & 0x03 == 0 // Aligned address
            && addr < self._byte_len { // In-bounds
            Some(self._data[addr])
        } else {
            None
        }
    }

    fn store(&mut self, addr: usize, data: u32) -> Option<()> {
        if addr == 0xf000_0000 {
            // Special case
            println!("RESULT = 0x{:08x} = {}", data, data);
            Some(())
        } else if addr & 0x03 == 0 // Aligned address
            && addr < self._byte_len { // In-bounds

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
        // TODO
        self.state = ProcState::Stopped;
    }

    fn dump(&self) {
        println!("{:?}\npc:0x{:08x}", self.state, self.pc);
        for i in 0..32 {
            println!("x{} = {} = 0x{:08x}", i, register_names[i], self.sreg[i]);
        }
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
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
    let mem = Memory::new_from_file(memory_bin);
    let mut processor = Processor::new(mem);

    loop {
        processor.exec_step();

        if processor.state == ProcState::Stopped {
            break
        }
    }
}
