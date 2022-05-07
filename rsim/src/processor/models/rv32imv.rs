use crate::models::Processor;
use crate::processor::exceptions::IllegalInstructionException::MiscDecodeException;
use anyhow::{Context,Result};

use crate::processor::exceptions::{IllegalInstructionException,MemoryException};
use crate::processor::decode;
use crate::processor::decode::{decode, InstructionBits};
use crate::processor::elements::memory::{AggregateMemory,Memory};
use crate::processor::elements::registers::RvRegisterFile;
use crate::processor::isa_mods::{IsaMod, Rv32im, Rv32imConn, Rv32v, Rv32vConn, IntVectorRegisterFile, Zicsr32, Zicsr32Conn, CSRProvider};

/// RISC-V Processor Model where XLEN=32-bit. No CHERI support.
/// Holds scalar registers and configuration, all other configuration stored in [ProcessorModules32]
pub struct Processor32 {
    pub running: bool,
    pub memory: AggregateMemory,
    pc: u32,
    sreg: RvRegisterFile<u32>,
    csrs: ProcessorCSRs32,
}

pub struct ProcessorModules32 {
    rv32im: Rv32im,
    rvv: Option<Rv32v>,
    zicsr: Option<Zicsr32>
}

struct ProcessorCSRs32 {}
impl CSRProvider<u32> for ProcessorCSRs32 {
    fn has_csr(&self, _csr: u32) -> bool {
        false
    }
    fn csr_atomic_read_write(&mut self, _csr: u32, _need_read: bool, _write_val: u32) -> Result<Option<u32>> { todo!() }
    fn csr_atomic_read_set(&mut self, _csr: u32, _set_bits: Option<u32>) -> Result<u32> { todo!() }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<u32>) -> Result<u32> { todo!() }
}

impl Processor32 {
    /// Create a new processor and vector unit which operates on given memory.
    ///
    /// # Arguments
    /// 
    /// * `mem` - The memory the processor should hold. Currently a value, not a reference.
    pub fn new(mem: AggregateMemory) -> (Processor32, ProcessorModules32) {
        let mut p = Processor32 {
            running: false,
            memory: mem,
            pc: 0,
            sreg: RvRegisterFile::<u32>::default(),
            csrs: ProcessorCSRs32{}
        };
        let mut mods = ProcessorModules32 {
            rv32im: Rv32im{},
            rvv: Some(Rv32v::new(Box::new(IntVectorRegisterFile::default()))),
            zicsr: Some(Zicsr32::default())
        };

        p.reset(&mut mods);

        (p, mods)
    }

    /// Get a short-lived connection to scalar resources, usable by the vector unit.
    /// This connection holds mutable references to fields in the Processor32.
    /// 
    /// # Associated Lifetimes
    /// 
    /// * `'a` - The lifetime of the Processor32
    /// * `'b` - The lifetime of the references held in the RvvConn
    /// 
    /// `'a : 'b` => `'a` outlives `'b`, e.g. the Processor32 will live longer than the references to its fields.
    /// Rust needs this guarantee.
    /// 
    /// Because Rust isn't smart enough to understand *which* fields in the processor are referenced,
    /// and the references inside the [RvvConn] are mutable,
    /// holding a [RvvConn] is equivalent to holding a *mutable reference to the Processor32 and all its fields*.
    /// This means you couldn't, say, store the [VectorUnit] inside the Processor32 and do `processor.v_unit.exec_inst(connection)`,
    /// because [VectorUnit::exec_inst()] tries to take a mutable reference to [VectorUnit], but the `connection` holds that reference already.
    fn vector_conn<'a,'b>(&'a mut self) -> Rv32vConn<'b> where 'a: 'b {
        Rv32vConn {
            sreg: &mut self.sreg,
            memory: &mut self.memory,
        }
    }

    fn zicsr_conn<'a,'b>(&'a mut self, rvv: &'a mut Option<Rv32v>) -> Zicsr32Conn<'b> where 'a: 'b {
        let mut csr_providers = vec![&mut self.csrs as &mut dyn CSRProvider<u32>];
        if let Some(rvv) = rvv.as_mut() {
            csr_providers.push(rvv as &mut dyn CSRProvider<u32>)
        }
        Zicsr32Conn {
            sreg: &mut self.sreg,
            csr_providers
        }
    }

    fn rv32im_conn<'a,'b>(&'a mut self) -> Rv32imConn<'b> where 'a: 'b {
        Rv32imConn {
            pc: self.pc,
            sreg: &mut self.sreg,
            memory: &mut self.memory,
        }
    }

    /// Process an instruction, returning the new PC value or any execution error
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    /// * `inst_bits` - The raw instruction bits
    /// * `opcode` - The major opcode of the decoded instruction
    /// * `inst` - The fields of the decoded instruction
    fn process_inst(&mut self, mods: &mut ProcessorModules32, inst_bits: u32, opcode: decode::Opcode, inst: InstructionBits) -> Result<u32> {
        let mut next_pc = self.pc + 4;
        
        if mods.rv32im.will_handle(opcode, inst) {
            let requested_pc = mods.rv32im.execute(opcode, inst, inst_bits, self.rv32im_conn())?;
            if let Some(requested_pc) = requested_pc {
                next_pc = requested_pc;
            }
            return Ok(next_pc);
        }
        if let Some(zicsr) = mods.zicsr.as_mut() {
            if zicsr.will_handle(opcode, inst) {
                let requested_pc = zicsr.execute(opcode, inst, inst_bits, self.zicsr_conn(&mut mods.rvv))?;
                if let Some(requested_pc) = requested_pc {
                    next_pc = requested_pc;
                }
                return Ok(next_pc);
            }
        }
        if let Some(rvv) = mods.rvv.as_mut() {
            if rvv.will_handle(opcode, inst) {
                rvv.execute(opcode, inst, inst_bits, &mut self.vector_conn())?;
                return Ok(next_pc);
            }
        }

        bail!(MiscDecodeException("Unexpected opcode/InstructionBits pair".to_string()))
    }
}
impl Processor<ProcessorModules32> for Processor32 {
    /// Reset the processor and associated vector unit
    fn reset(&mut self, mods: &mut ProcessorModules32) {
        self.running = false;
        self.pc = 0;
        self.sreg.reset();

        if let Some(v_unit) = mods.rvv.as_mut() {
            v_unit.reset();
        }
    }

    /// Run a fetch-decode-execute step on the processor, executing a single instruction
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    fn exec_step(&mut self, mods: &mut ProcessorModules32) -> Result<()> {
        self.running = true;

        let next_pc_res: Result<u32> = {
            // Fetch
            let inst_bits = self.memory.load_u32(self.pc as u64).context("Couldn't load next instruction")?;

            // Decode
            let (opcode, inst) = decode(inst_bits)
                .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

            // Execute
            let next_pc = self.process_inst(mods, inst_bits, opcode, inst)
                .with_context(|| format!("Failed to execute decoded instruction {:?} {:x?}", opcode, inst))?;

            if next_pc % 4 != 0 {
                Err(MemoryException::JumpMisaligned{addr: next_pc as usize, expected: 4})?
            } else {
                Ok(next_pc)
            }
        };

        let next_pc = match next_pc_res {
            Ok(val) => val,
            Err(err) => {
                if let Some(_iie) = err.downcast_ref::<IllegalInstructionException>() {
                    // TODO - trap, return new PC
                    println!("Found Illegal Instruction error");
                    return Err(err)
                } else if let Some(_mem) = err.downcast_ref::<MemoryException>() {
                    // TODO - trap, return new PC
                    println!("Found Memory error");
                    return Err(err)
                } else {
                    println!("Untrappable error");
                    return Err(err)
                }
            }
        };

        // Increment PC
        self.pc = next_pc;

        Ok(())
    }

    /// Dump processor and vector unit state to standard output.
    fn dump(&self, mods: &ProcessorModules32) {
        println!("running: {:?}\npc: 0x{:08x}", self.running, self.pc);
        self.sreg.dump();
        if let Some(rvv) = mods.rvv.as_ref() {
            rvv.dump();
        }
    }

    fn running(&self) -> bool {
        self.running
    }

    fn get_io_values(&self) -> Vec<Option<u64>> {
        self.memory.get_io_values()
    }
}