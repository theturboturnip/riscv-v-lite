use crate::processor::elements::cheri::IntegerModeCheriAggregateMemory;
use crate::models::Processor;
use crate::processor::exceptions::IllegalInstructionException::MiscDecodeException;
use anyhow::{Context,Result};

use crate::processor::exceptions::{IllegalInstructionException,MemoryException};
use crate::processor::decode;
use crate::processor::decode::{decode, InstructionBits};
use crate::processor::elements::registers::{RegisterTracking};
use crate::processor::elements::cheri::{Cc128Cap,CheriRV64RegisterFile,CheriAggregateMemory};
use crate::processor::isa_mods::{IsaMod, Rv64i, Rv64iConn, XCheri64, XCheri64Conn, Zicsr64, Zicsr64Conn, CSRProvider};

/// RISC-V Processor Model where XLEN=32-bit. No CHERI support.
/// Holds scalar registers and configuration, all other configuration stored in [ProcessorModules32]
pub struct Rv64iXCheriProcessor {
    pub running: bool,
    pub memory: CheriAggregateMemory,
    pcc: Cc128Cap,
    ddc: Cc128Cap,
    max_cap: Cc128Cap,
    sreg: CheriRV64RegisterFile,
    csrs: Rv64iXCheriProcessorCSRs,
}

pub struct Rv64iXCheriProcessorModules {
    rv64i: Rv64i,
    xcheri: XCheri64,
    zicsr: Option<Zicsr64>
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum CheriExecMode {
    Integer,
    Capability,
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
        let full_range_cap = mem.get_full_range_cap();
        let mut pcc = full_range_cap.clone();
        // Set the flag on pcc to 1 so we're in Capability Mode
        // TR-951$5.3
        pcc.set_flags(1);

        let mut p = Rv64iXCheriProcessor {
            running: false,
            memory: mem,
            pcc: pcc,
            ddc: full_range_cap,
            max_cap: full_range_cap,
            sreg: CheriRV64RegisterFile::default(),
            csrs: Rv64iXCheriProcessorCSRs{}
        };
        let mut mods = Rv64iXCheriProcessorModules {
            rv64i: Rv64i{},
            xcheri: XCheri64{},
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

    fn xcheri64_conn<'a,'b>(&'a mut self) -> XCheri64Conn<'b> where 'a: 'b {
        XCheri64Conn {
            pcc: self.pcc,
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
    fn process_inst(&mut self, mods: &mut Rv64iXCheriProcessorModules, inst_bits: u32, opcode: decode::Opcode, inst: InstructionBits) -> Result<Cc128Cap> {
        let mode = match self.pcc.flags() {
            0 => CheriExecMode::Integer,
            1 => CheriExecMode::Capability,
            _ => bail!("invalid flag in PC")
        };
        
        // Copy self.pcc, set address to address + 4
        let mut next_pcc = self.pcc;
        next_pcc.set_address_unchecked(next_pcc.address() + 4);
        
        if mode == CheriExecMode::Capability && mods.xcheri.will_handle(opcode, inst) {
            let requested_pcc = mods.xcheri.execute(opcode, inst, inst_bits, self.xcheri64_conn())?;
            if let Some(requested_pcc) = requested_pcc {
                next_pcc = requested_pcc;
            }
            return Ok(next_pcc);
        }
        if mods.rv64i.will_handle(opcode, inst) {
            let requested_pc = {
                // Create the integer-mode memory wrapper, 
                // only keep it alive for the duration of
                // rv64i.execute()
                let mut mem_wrap = IntegerModeCheriAggregateMemory::wrap(&mut self.memory, self.ddc);
                mods.rv64i.execute(opcode, inst, inst_bits, Rv64iConn {
                    pc: self.pcc.address(),
                    sreg: &mut self.sreg,
                    memory: &mut mem_wrap,
                })?
            };
            if let Some(requested_pc) = requested_pc {
                next_pcc.set_address_unchecked(requested_pc);
            }
            return Ok(next_pcc);
        }
        if let Some(zicsr) = mods.zicsr.as_mut() {
            if zicsr.will_handle(opcode, inst) {
                let requested_pc = zicsr.execute(opcode, inst, inst_bits, self.zicsr_conn())?;
                if let Some(requested_pc) = requested_pc {
                    next_pcc.set_address_unchecked(requested_pc);
                }
                return Ok(next_pcc);
            }
        }

        bail!(MiscDecodeException("Unexpected opcode/InstructionBits pair".to_string()))
    }
}
impl Processor<Rv64iXCheriProcessorModules> for Rv64iXCheriProcessor {
    /// Reset the processor and associated vector unit
    fn reset(&mut self, _mods: &mut Rv64iXCheriProcessorModules) {
        self.running = false;
        self.pcc = self.max_cap;
        self.pcc.set_flags(1);
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

        let next_pcc_res: Result<Cc128Cap> = {
            // Fetch
            let inst_bits = self.memory.fetch_inst_u32(self.pcc).context("Couldn't load next instruction")?;

            // Decode
            let (opcode, inst) = decode(inst_bits)
                .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

            // Execute
            let next_pcc = self.process_inst(mods, inst_bits, opcode, inst)
                .with_context(|| format!("Failed to execute decoded instruction {:?} {:?}", opcode, inst))?;

            if next_pcc.address() % 4 != 0 {
                Err(MemoryException::JumpMisaligned{addr: next_pcc.address() as usize, expected: 4})?
            } else {
                Ok(next_pcc)
            }
        };

        // TODO use this for something
        let _register_file_actions = self.sreg.end_tracking()?;

        let next_pcc = match next_pcc_res {
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
        self.pcc = next_pcc;

        Ok(())
    }

    /// Dump processor and vector unit state to standard output.
    fn dump(&self, _mods: &Rv64iXCheriProcessorModules) {
        println!("running: {:?}\npc: {:?}", self.running, self.pcc);
        self.sreg.dump();
    }

    fn running(&self) -> bool {
        self.running
    }
}