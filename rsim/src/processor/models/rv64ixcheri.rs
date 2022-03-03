use crate::models::Processor;
use crate::processor::exceptions::IllegalInstructionException::MiscDecodeException;
use anyhow::{Context,Result};

use crate::processor::exceptions::{IllegalInstructionException,MemoryException};
use crate::processor::decode;
use crate::processor::decode::{decode, InstructionBits};
use crate::processor::elements::memory::{Memory32};
use crate::processor::elements::registers::{RegisterTracking};
use crate::processor::elements::cheri::{CheriRV64RegisterFile,CheriAggregateMemory};
use crate::processor::isa_mods::{IsaMod, Rv64i, Rv64iConn, Zicsr64, Zicsr64Conn, CSRProvider};

/// RISC-V Processor Model where XLEN=32-bit. No CHERI support.
/// Holds scalar registers and configuration, all other configuration stored in [ProcessorModules32]
pub struct Rv64iXCheriProcessor {
    pub running: bool,
    pub memory: CheriAggregateMemory,
    pc: u64,
    sreg: CheriRV64RegisterFile,
    csrs: Rv64iXCheriProcessorCSRs,
}

pub struct Rv64iXCheriProcessorModules {
    rv64i: Rv64i,
    zicsr: Option<Zicsr64>
}

struct Rv64iXCheriProcessorCSRs {}
impl CSRProvider<u64> for Rv64iXCheriProcessorCSRs {
    fn has_csr(&self, _csr: u32) -> bool {
        false
    }
    fn csr_atomic_read_write(&mut self, _csr: u32, _need_read: bool, _write_val: u64) -> Result<Option<u64>> { todo!() }
    fn csr_atomic_read_set(&mut self, _csr: u32, _set_bits: Option<u64>) -> Result<u64> { todo!() }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<u64>) -> Result<u64> { todo!() }
}

impl Rv64iXCheriProcessor {
    /// Create a new processor and vector unit which operates on given memory.
    ///
    /// # Arguments
    /// 
    /// * `mem` - The memory the processor should hold. Currently a value, not a reference.
    pub fn new(mem: CheriAggregateMemory) -> (Rv64iXCheriProcessor, Rv64iXCheriProcessorModules) {
        let mut p = Rv64iXCheriProcessor {
            running: false,
            memory: mem,
            pc: 0,
            sreg: CheriRV64RegisterFile::default(),
            csrs: Rv64iXCheriProcessorCSRs{}
        };
        let mut mods = Rv64iXCheriProcessorModules {
            rv64i: Rv64i{},
            zicsr: Some(Zicsr64::default())
        };

        p.reset(&mut mods);

        (p, mods)
    }

    fn zicsr_conn<'a,'b>(&'a mut self) -> Zicsr64Conn<'b> where 'a: 'b {
        let csr_providers = vec![&mut self.csrs as &mut dyn CSRProvider<u64>];
        Zicsr64Conn {
            sreg: &mut self.sreg,
            csr_providers
        }
    }

    fn rv64i_conn<'a,'b>(&'a mut self) -> Rv64iConn<'b> where 'a: 'b {
        Rv64iConn {
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
    fn process_inst(&mut self, mods: &mut Rv64iXCheriProcessorModules, inst_bits: u32, opcode: decode::Opcode, inst: InstructionBits) -> Result<u64> {
        let mut next_pc = self.pc + 4;
        
        if mods.rv64i.will_handle(opcode, inst) {
            let requested_pc = mods.rv64i.execute(opcode, inst, inst_bits, self.rv64i_conn())?;
            if let Some(requested_pc) = requested_pc {
                next_pc = requested_pc;
            }
            return Ok(next_pc);
        }
        if let Some(zicsr) = mods.zicsr.as_mut() {
            if zicsr.will_handle(opcode, inst) {
                let requested_pc = zicsr.execute(opcode, inst, inst_bits, self.zicsr_conn())?;
                if let Some(requested_pc) = requested_pc {
                    next_pc = requested_pc;
                }
                return Ok(next_pc);
            }
        }

        bail!(MiscDecodeException("Unexpected opcode/InstructionBits pair".to_string()))
    }
}
impl Processor<Rv64iXCheriProcessorModules> for Rv64iXCheriProcessor {
    /// Reset the processor and associated vector unit
    fn reset(&mut self, _mods: &mut Rv64iXCheriProcessorModules) {
        self.running = false;
        self.pc = 0;
        self.sreg.reset();
    }

    /// Run a fetch-decode-execute step on the processor, executing a single instruction
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    fn exec_step(&mut self, mods: &mut Rv64iXCheriProcessorModules) -> Result<()> {
        self.running = true;

        self.sreg.start_tracking()?;

        let next_pc_res: Result<u64> = {
            // Fetch
            let inst_bits = self.memory.load_u32(self.pc as u64).context("Couldn't load next instruction")?;

            // Decode
            let (opcode, inst) = decode(inst_bits)
                .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

            // Execute
            let next_pc = self.process_inst(mods, inst_bits, opcode, inst)
                .with_context(|| format!("Failed to execute decoded instruction {:?} {:?}", opcode, inst))?;

            if next_pc % 4 != 0 {
                Err(MemoryException::JumpMisaligned{addr: next_pc as usize, expected: 4})?
            } else {
                Ok(next_pc)
            }
        };

        // TODO use this for something
        let _register_file_actions = self.sreg.end_tracking()?;

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
    fn dump(&self, _mods: &Rv64iXCheriProcessorModules) {
        println!("running: {:?}\npc: 0x{:08x}", self.running, self.pc);
        self.sreg.dump();
    }

    fn running(&self) -> bool {
        self.running
    }
}