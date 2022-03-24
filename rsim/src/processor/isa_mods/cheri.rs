use crate::processor::isa_mods::rv64i::sign_extend64;
use crate::processor::isa_mods::*;
use crate::processor::exceptions::{CapabilityException,CapOrRegister};
use crate::processor::elements::cheri::{Cc128,CompressedCapability,Cc128Cap,CheriAggregateMemory,CheriRVFuncs,SafeTaggedCap};
use crate::processor::elements::cheri::CheriRV64RegisterFile;
use crate::processor::elements::registers::RegisterFile;

pub struct XCheri64Conn<'a> {
    pub pcc: Cc128Cap,
    pub sreg: &'a mut CheriRV64RegisterFile,
    pub memory: &'a mut CheriAggregateMemory,
}
impl<'a> IsaModConn for XCheri64Conn<'a> {}

pub struct XCheri64 {}
impl XCheri64 {
    fn handle_cjalr(&mut self, cd: u8, cs1: u8, conn: XCheri64Conn) -> ProcessorResult<Cc128Cap> {
        let cs1_reg = CapOrRegister::Reg(cs1);
        let cs1_val = conn.sreg.read_maybe_cap(cs1)?;
        match cs1_val {
            SafeTaggedCap::RawData{..} => bail!(CapabilityException::TagViolation{ cap: cs1_reg }),
            SafeTaggedCap::ValidCap(cs1_val) => {
                // We're jumping to CS1, so it should be a SENTRY
                if cs1_val.is_sealed() && cs1_val.otype() != Cc128::OTYPE_SENTRY {
                    bail!(CapabilityException::SealViolation{ cap: cs1_reg });
                }
                // The Sail code does other checks: permission to execute, bounds checks, alignment.
                // I'm leaving these to be handled by the memory module after the jump completes.
                // TODO put a public function on the memory module so we can check those issues here?

                // Set the link-capability to the next instruction
                let next_pc = conn.pcc.address() + 4; // TODO account for compressed instructions?
                let (success, link_cap) = Cc128::setCapAddr(&conn.pcc, next_pc);
                assert!(success, "Link cap should always be representable.");
                assert!(!link_cap.is_sealed(), "Link cap should always be unsealed.");
                let link_cap = Cc128::sealCap(&link_cap, Cc128::OTYPE_SENTRY);
                assert!(link_cap.tag());
                conn.sreg.write_maybe_cap(cd, SafeTaggedCap::from_cap(link_cap))?;

                // Zero out bottom bit of the address we're jumping to
                let new_pc = cs1_val.address() & (u64::MAX << 1);
                // TODO the sail seems to handle the PC and the PCC separately - I thought these were the same thing?
                let mut new_pcc = Cc128::unsealCap(&cs1_val);
                new_pcc.set_address_unchecked(new_pc);
                Ok(new_pcc)
            }
        }
    }
}
impl IsaMod<XCheri64Conn<'_>> for XCheri64 {
    type Pc = Cc128Cap;

    fn will_handle(&self, opcode: Opcode, inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (AddUpperImmPC, _) | (Custom2CHERI, _) => true,
            // LC (reuses RV128-LQ, which itself is on top of the Misc-Mem opcode)
            (MiscMem, InstructionBits::IType{funct3: 0x2, ..}) => true,
            // SC (reuses RV128-Sq, which is an extra instruction in Store)
            (Store, InstructionBits::SType{funct3: 0x4, ..}) => true,
            // CJALR?
            (JumpAndLinkRegister, _) => true,
            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: XCheri64Conn) -> ProcessorResult<Option<Self::Pc>> {
        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = (sign_extend64(imm as u64, 32) as u64) + conn.pcc.address();
                let (representable, mut new_cap) = Cc128::setCapAddr(&conn.pcc, addr);
                if !representable {
                    new_cap = Cc128::invalidateCap(&new_cap);
                }
                conn.sreg.write(rd, SafeTaggedCap::from_cap(new_cap))?;
            }
            (MiscMem, InstructionBits::IType{rd, funct3: 0x2, rs1, imm}) => {
                // LC = Load Capability
                /*
                let offset : xlenbits = EXTS(imm);
                let (auth_val, vaddr, auth_idx) = get_cheri_mode_cap_addr(rs1_cs1, offset);
                handle_load_cap_via_cap(cd, auth_idx, auth_val, vaddr)
                */
                let offset = sign_extend64(imm as u64, 32) as u64;
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address() + offset);

                // Load a capability from cap_with_addr
                let loaded_cap = conn.memory.load_maybe_cap(cap)?;
                // Store it into the register file
                conn.sreg.write_maybe_cap(rd, loaded_cap)?;
            },
            (Store, InstructionBits::SType{funct3: 0x4, rs1, rs2, imm}) => {
                // SC = Store Capability

                let offset = sign_extend64(imm as u64, 32) as u64;
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address() + offset);

                let cap_to_store = conn.sreg.read_maybe_cap(rs2)?;
                conn.memory.store_maybe_cap(cap, cap_to_store)?;
            },
            (JumpAndLinkRegister, _) => {
                // Vanilla JALR jumps with an immediate offset, unlike CJALR.
                // It appears CHERI-Clang emits vanilla JALRs rather than CJALRs, even in capability mode.
                // To account for this, we handle vanilla JALRs as CJALRs, except we need to make sure the immediate offset = 0.
                if let InstructionBits::IType{funct3: 0, imm: 0, rd, rs1} = inst {
                    return Ok(Some(self.handle_cjalr(rd, rs1, conn)?))
                } else {
                    bail!("Vanilla nonzero-offset JumpAndLinkRegister in Capability Mode")
                }
            }
            (Custom2CHERI, InstructionBits::ROrIType{rd, funct3, rs1, rs2, funct7, imm}) => {
                if funct3 == 0x0 && funct7 == 0x7f {
                    match rs2 {
                        0x0 => {
                            // CGetPerm
                            todo!()
                        }
                        0x1 => {
                            // CGetType
                            todo!()
                        }
                        0x2 => {
                            // CGetBase
                            todo!()
                        }
                        0x3 => {
                            // CGetLen
                            todo!()
                        }
                        0x4 => {
                            // CGetTag
                            todo!()
                        }
                        0x5 => {
                            // CGetSealed
                            todo!()
                        }
                        0x6 => {
                            // CGetOffset
                            todo!()
                        }
                        0x7 => {
                            // CGetFlags
                            todo!()
                        }
                        0xa => {
                            // CMove
                            todo!()
                        }
                        0xb => {
                            // CClearTag
                            todo!()
                        }
                        0xc => {
                            // CJALR
                            return Ok(Some(self.handle_cjalr(rd, rs1, conn)?))
                        }
                        0xd => {
                            // Clear
                            todo!()
                        }
                        0xe => {
                            // CClear
                            todo!()
                        }
                        0xf => {
                            // CGetAddr
                            todo!()
                        }
                        0x10 => {
                            // FPClear
                            bail!("FPClear called when floating-point not supported")
                        }
                        0x11 => {
                            // CSealEntry
                            todo!()
                        }
                        0x12 => {
                            // CLoadTags
                            todo!()
                        }
                        _ => bail!("Invalid rs2 value {:x} for CHERI funct3=0x0,funct7=0x7f", rs2)
                    }
                }
                bail!("instruction not been handled: {:?}", inst)
            }

            _ => bail!("Invalid opcode/instruction pair passed to XCheri64")   
        }
        Ok(None)
    }
}
