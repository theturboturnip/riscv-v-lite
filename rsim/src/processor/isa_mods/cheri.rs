use crate::models::CheriExecMode;
use crate::processor::exceptions::IllegalInstructionException::UnsupportedParam;
use crate::processor::isa_mods::*;
use crate::processor::exceptions::{CapabilityException,CapOrRegister};
use crate::processor::elements::cheri::{Cc128,CompressedCapability,Cc128Cap,CheriMemory,CheriAggregateMemory,CheriRVFuncs,SafeTaggedCap};
use crate::processor::elements::cheri::CheriRV64RegisterFile;
use crate::processor::elements::registers::RegisterFile;

/// Connection to register state for 64-bit CHERI-aware ISA modules.
/// Used by [XCheri64] and [Rv64imCapabilityMode]. 
pub struct XCheri64Conn<'a> {
    pub pcc: Cc128Cap,
    pub sreg: &'a mut CheriRV64RegisterFile,
    pub memory: &'a mut CheriAggregateMemory,
    pub mode: CheriExecMode,
    pub ddc: Cc128Cap,
}

/// ISA module implementing the new CHERI instructions for 64-bit ISAs.
/// Does not include capability-mode overrides for legacy instructions.
pub struct XCheri64 {}
impl XCheri64 {
    fn handle_cjalr(&mut self, cd: u8, cs1: u8, offset: u64, conn: XCheri64Conn) -> ProcessorResult<Cc128Cap> {
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
                let mut new_pc = cs1_val.address() & (u64::MAX << 1);
                // NOTE this isn't in the CJALR spec, but this instruction needs to support JALR functionality i.e. immediate offset
                new_pc = new_pc.wrapping_add(offset);
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
            (Custom2CHERI, _) => true,
            // LC (reuses RV128-LQ, which itself is on top of the Misc-Mem opcode)
            (MiscMem, InstructionBits::IType{funct3: 0x2, ..}) => true,
            // SC (reuses RV128-Sq, which is an extra instruction in Store)
            (Store, InstructionBits::SType{funct3: 0x4, ..}) => true,
            // CJALR is under the Custom CHERI opcode
            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: XCheri64Conn) -> ProcessorResult<Option<Self::Pc>> {
        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (MiscMem, InstructionBits::IType{rd, funct3: 0x2, rs1, imm}) => {
                // LC = Load Capability
                /*
                let offset : xlenbits = EXTS(imm);
                let (auth_val, vaddr, auth_idx) = get_cheri_mode_cap_addr(rs1_cs1, offset);
                handle_load_cap_via_cap(cd, auth_idx, auth_val, vaddr)
                */
                // The address we store to depends on the mode
                let origin = match conn.mode {
                    CheriExecMode::Capability => {
                        let offset = imm.sign_extend_u64();
                        let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        cap.set_address_unchecked(cap.address().wrapping_add(offset));
                        cap
                    }
                    CheriExecMode::Integer => {
                        let reg_rs1 = conn.sreg.read_u64(rs1)?;
                        let offset = imm.sign_extend_u64().wrapping_add(reg_rs1);
                        let mut cap = conn.ddc.clone();
                        cap.set_address_unchecked(cap.address().wrapping_add(offset));
                        cap
                    }
                };

                // Load a capability from cap_with_addr
                let loaded_cap = conn.memory.load_maybe_cap(origin)?;
                // Store it into the register file
                conn.sreg.write_maybe_cap(rd, loaded_cap)?;
            },
            (Store, InstructionBits::SType{funct3: 0x4, rs1, rs2, imm}) => {
                // SC = Store Capability

                // The address we store to depends on the mode
                let destination = match conn.mode {
                    CheriExecMode::Capability => {
                        let offset = imm.sign_extend_u64();
                        let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        cap.set_address_unchecked(cap.address().wrapping_add(offset));
                        cap
                    }
                    CheriExecMode::Integer => {
                        let reg_rs1 = conn.sreg.read_u64(rs1)?;
                        let offset = imm.sign_extend_u64().wrapping_add(reg_rs1);
                        let mut cap = conn.ddc.clone();
                        cap.set_address_unchecked(cap.address().wrapping_add(offset));
                        cap
                    }
                };
                
                let cap_to_store = conn.sreg.read_maybe_cap(rs2)?;
                conn.memory.store_maybe_cap(destination, cap_to_store)?;
            },
            // (JumpAndLinkRegister, InstructionBits::IType{funct3, imm, rd, rs1}) => {
            //     // Vanilla JALR jumps with an immediate offset, unlike CJALR.
            //     // It appears CHERI-Clang emits vanilla JALRs rather than CJALRs, even in capability mode.
            //     // To account for this, we handle vanilla JALRs as CJALRs.
            //     // JALR supports an immediate offset (although technically CJALR doesn't?)
            //     if funct3 == 0 {
            //         return Ok(Some(self.handle_cjalr(rd, rs1, imm.sign_extend_u64(), conn)?))
            //     } else {
            //         bail!("Vanilla nonzero-offset JumpAndLinkRegister in Capability Mode")
            //     }
            // }
            (Custom2CHERI, InstructionBits::ROrIType{rd, funct3, rs1, rs2, funct7, imm}) => {
                match (funct7, funct3) {
                    (0x1, 0x0) => {
                        // CSpecialRW
                        bail!("Haven't implemented CSpecialRW")
                    }
                    (0x8, 0x0) => {
                        // CSetBounds
                        let cs1_reg = CapOrRegister::Reg(rs1);
                        let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        let rs2_val = conn.sreg.read_u64(rs2)?;

                        let new_base = cs1_val.address();
                        let new_top = new_base as u128 + rs2_val as u128;
                       
                        if !cs1_val.tag() {
                            bail!(CapabilityException::TagViolation{ cap: cs1_reg });
                        } else if cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: cs1_reg });
                        } else if !Cc128::inCapBounds(&cs1_val, new_base, rs2_val as u128) {
                            bail!(CapabilityException::LengthViolation{ cap: cs1_reg, base: new_base, top: new_top });
                        }

                        let (_, new_cap) = Cc128::setCapBounds(&cs1_val, new_base, new_top);
                        conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                    }
                    (0x9, 0x0) => {
                        // CSetBoundsExact
                        let cs1_reg = CapOrRegister::Reg(rs1);
                        let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        let rs2_val = conn.sreg.read_u64(rs2)?;

                        let new_base = cs1_val.address();
                        let new_top = new_base as u128 + rs2_val as u128;
                       
                        if !cs1_val.tag() {
                            bail!(CapabilityException::TagViolation{ cap: cs1_reg });
                        } else if cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: cs1_reg });
                        } else if !Cc128::inCapBounds(&cs1_val, new_base, rs2_val as u128) {
                            bail!(CapabilityException::LengthViolation{ cap: cs1_reg, base: new_base, top: new_top });
                        }

                        let (exact, new_cap) = Cc128::setCapBounds(&cs1_val, new_base, new_top);
                        if !exact {
                            bail!(CapabilityException::InexactBounds{ cap: cs1_reg });
                        }
                        conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                    }
                    (_, 0x2) => {
                        // CSetBoundsImm
                        let cs1_reg = CapOrRegister::Reg(rs1);
                        let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        let imm_val = imm.no_extend_u64();

                        let new_base = cs1_val.address();
                        let new_top = new_base as u128 + imm_val as u128;
                       
                        if !cs1_val.tag() {
                            bail!(CapabilityException::TagViolation{ cap: cs1_reg });
                        } else if cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: cs1_reg });
                        } else if !Cc128::inCapBounds(&cs1_val, new_base, imm_val as u128) {
                            bail!(CapabilityException::LengthViolation{ cap: cs1_reg, base: new_base, top: new_top });
                        }

                        let (_, new_cap) = Cc128::setCapBounds(&cs1_val, new_base, new_top);
                        conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                    }
                    (0xb, 0x0) => {
                        // CSeal
                        bail!("Haven't implemented CSeal")
                    }
                    (0xc, 0x0) => {
                        // CUnseal
                        bail!("Haven't implemented CUnseal")
                    }
                    (0xd, 0x0) => {
                        // CAndPerm
                        bail!("Haven't implemented CAndPerm")
                    }
                    (0xe, 0x0) => {
                        // CSetFlags
                        bail!("Haven't implemented CSetFlags")
                    }
                    (0xf, 0x0) => {
                        // CSetOffset
                        bail!("Haven't implemented CSetOffset")
                    }
                    (0x10, 0x0) => {
                        // CSetAddr
                        bail!("Haven't implemented CSetAddr")
                    }
                    (0x11, 0x0) => {
                        // CIncOffset
                        let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        let rs2_val = conn.sreg.read_u64(rs2)?;
                        if cs1_val.tag() && cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: CapOrRegister::Reg(rs1) })
                        } else {
                            let (success, mut new_cap) = Cc128::incCapOffset(&cs1_val, rs2_val);
                            if !success {
                                new_cap = Cc128::invalidateCap(&new_cap);
                            }
                            conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                        }
                    }
                    (_, 0x1) => {
                        // CIncOffsetImm
                        let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                        if cs1_val.tag() && cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: CapOrRegister::Reg(rs1) })
                        } else {
                            let (success, mut new_cap) = Cc128::incCapOffset(&cs1_val, imm.sign_extend_u64());
                            if !success {
                                new_cap = Cc128::invalidateCap(&new_cap);
                            }
                            conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                        }
                    }
                    (0x12, 0x0) => {
                        // CToPtr
                        bail!("Haven't implemented CToPtr")
                    }
                    (0x13, 0x0) => {
                        // CFromPtr
                        
                        let cs1_val = if rs1 == 0 {
                            conn.ddc
                        } else {
                            conn.sreg.read_maybe_cap(rs1)?.to_cap()
                        };
                        let rs2_val = conn.sreg.read_u64(rs2)?;

                        if rs2_val == 0 {
                            conn.sreg.write_u64(rd, 0)?;
                        } else if !cs1_val.tag() {
                            bail!(CapabilityException::TagViolation{ cap: CapOrRegister::Reg(rs1) })
                        } else if cs1_val.is_sealed() {
                            bail!(CapabilityException::SealViolation{ cap: CapOrRegister::Reg(rs1) })
                        } else {
                            let (success, mut new_cap) = Cc128::setCapOffset(&cs1_val, rs2_val);
                            if !success {
                                new_cap = Cc128::invalidateCap(&new_cap);
                            }
                            conn.sreg.write_maybe_cap(rd, SafeTaggedCap::from_cap(new_cap))?;
                        }
                    }
                    (0x14, 0x0) => {
                        // CSub
                        bail!("Haven't implemented CSub")
                    }
                    (0x1d, 0x0) => {
                        // CBuildCap
                        bail!("Haven't implemented CBuildCap")
                    }
                    (0x1e, 0x0) => {
                        // CCopyType
                        bail!("Haven't implemented CCopyType")
                    }
                    (0x1f, 0x0) => {
                        // CCSeal
                        bail!("Haven't implemented CCSeal")
                    }
                    (0x20, 0x0) => {
                        // CestSubset
                        bail!("Haven't implemented CestSubset")
                    }
                    (0x21, 0x0) => {
                        // CSetEqualExact
                        bail!("Haven't implemented CSetEqualExact")
                    }
                    (0x7e, 0x0) => match rd {
                        0x1 => {
                            // CInvoke
                            bail!("Haven't implemented CInvoke")
                        }
                        0x1f => {
                            // CClearTags
                            bail!("Haven't implemented CClearTags")
                        }
                        _ => bail!("Invalid funct3/funct7/rd combination {:x}/{:x}/{:x}", funct3, funct7, rd)
                    }
                    (0x7f, 0x0) => match rs2 {
                        0x0 => {
                            // CGetPerm
                            bail!("Haven't implemented CGetPerm")
                        }
                        0x1 => {
                            // CGetType
                            bail!("Haven't implemented CGetType")
                        }
                        0x2 => {
                            // CGetBase
                            bail!("Haven't implemented CGetBase")
                        }
                        0x3 => {
                            // CGetLen
                            bail!("Haven't implemented CGetLen")
                        }
                        0x4 => {
                            // CGetTag
                            let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                            conn.sreg.write_u64(
                                rd,
                                if cs1_val.tag() {
                                    1
                                } else {
                                    0
                                }
                            )?;
                        }
                        0x5 => {
                            // CGetSealed
                            bail!("Haven't implemented CGetSealed")
                        }
                        0x6 => {
                            // CGetOffset
                            bail!("Haven't implemented CGetOffset")
                        }
                        0x7 => {
                            // CGetFlags
                            bail!("Haven't implemented CGetFlags")
                        }
                        0x8 => {
                            // CRoundRepresentableLength
                            bail!("Haven't implemented CRoundRepresentableLength")
                        }
                        0x9 => {
                            // CRepresentableAlignmentMask
                            bail!("Haven't implemented CRepresentableAlignmentMask")
                        }
                        0xa => {
                            // CMove
                            let val = conn.sreg.read_maybe_cap(rs1)?;
                            conn.sreg.write_maybe_cap(rd, val)?;
                        }
                        0xb => {
                            // CClearTag
                            bail!("Haven't implemented CClearTag")
                        }
                        0xc => {
                            // CJALR
                            // CJALR doesn't support immediate-offset, so say offset=0
                            return Ok(Some(self.handle_cjalr(rd, rs1, 0, conn)?))
                        }
                        0xd => {
                            // Clear
                            bail!("Haven't implemented Clear")
                        }
                        0xe => {
                            // CClear
                            bail!("Haven't implemented CClear")
                        }
                        0xf => {
                            // CGetAddr
                            bail!("Haven't implemented CGetAddr")
                        }
                        0x10 => {
                            // FPClear
                            bail!("FPClear called when floating-point not supported")
                        }
                        0x11 => {
                            // CSealEntry
                            bail!("Haven't implemented CSealEntry")
                        }
                        0x12 => {
                            // CLoadTags
                            bail!("Haven't implemented CLoadTags")
                        }
                        0x14 => {
                            // CJALR - relative to PCC?
                            // See llvm/lib/Target/RISCV/RISCVInstrInfoXCheri.td:389
                            bail!("Haven't implemented CJALR relative to PCC")
                        }
                        _ => bail!("Invalid rs2 value {:x} for CHERI funct3=0x0,funct7=0x7f", rs2)
                    }
                    (0x7d, 0) => match rs2 {
                        0xb => {
                            // LC.CAP (RV32??)
                            // Load Capability with Cap
                            let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                            // The memory will clear the tag if cs1_val doesn't have a Load_Capability permission
                            let loaded_cap = conn.memory.load_maybe_cap(cs1_val)?;
                            conn.sreg.write_maybe_cap(rd, loaded_cap)?;
                        }
                        _ => bail!("Unhandled rs2 value {:x} for CHERI load (funct7=0x7d, funct3=0)", rs2)
                    }
                    (0x7c, 0) => match rd {
                        0xb => {
                            // SD.CAP
                            // i.e. Store Doubleword (64-bits) with Cap
                            let cs1_val = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                            let stored_val = conn.sreg.read_u64(rs2)?;
                            conn.memory.store_u64(cs1_val, stored_val)?;
                        }
                        _ => bail!("Unhandled rd value {:x} for CHERI store (funct7=0x7c, funct3=0)", rd)
                    }
                    _ => bail!("Invalid funct3/funct7 combination {:x} {:x}", funct3, funct7)
                }
            }

            _ => bail!("Invalid opcode/instruction pair passed to XCheri64")   
        }
        Ok(None)
    }
}

/// Override for base RV64I instructions when in "capability mode".
/// See TR-951$5.3.6, page 152 for list of affected instructions.
pub struct Rv64imCapabilityMode {}
impl IsaMod<XCheri64Conn<'_>> for Rv64imCapabilityMode {
    type Pc = u64;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match opcode {
            Load | Store | AddUpperImmPC => true,

            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: XCheri64Conn) -> ProcessorResult<Option<Self::Pc>> {
        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = conn.pcc.address().wrapping_add(imm.sign_extend_u64());
                let (representable, mut new_cap) = Cc128::setCapAddr(&conn.pcc, addr);
                if !representable {
                    new_cap = Cc128::invalidateCap(&new_cap);
                }
                conn.sreg.write(rd, SafeTaggedCap::from_cap(new_cap))?;
            }
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let offset = imm.sign_extend_u64();
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address().wrapping_add(offset));

                let new_val = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => (conn.memory.load_u8(cap)? as i8) as i64 as u64, // LB
                    0b001 => (conn.memory.load_u16(cap)? as i16) as i64 as u64, // LH
                    0b010 => (conn.memory.load_u32(cap)? as i32) as i64 as u64, // LW
                    0b011 => conn.memory.load_u64(cap)? as u64, // LD
                    // LBU, LHU, LWU don't sign-extend
                    0b100 => conn.memory.load_u8(cap)? as u64, // LBU
                    0b101 => conn.memory.load_u16(cap)? as u64, // LHU
                    0b110 => conn.memory.load_u32(cap)? as u64, // LWU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3)))
                };
                conn.sreg.write(rd, new_val)?;
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let offset = imm.sign_extend_u64();
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address().wrapping_add(offset));
                
                match funct3 {
                    0b000 => conn.memory.store_u8(cap, (conn.sreg.read_u64(rs2)? & 0xFF) as u8)?,
                    0b001 => conn.memory.store_u16(cap, (conn.sreg.read_u64(rs2)? & 0xFFFF) as u16)?,
                    0b010 => conn.memory.store_u32(cap, (conn.sreg.read_u64(rs2)? & 0xFFFF_FFFF) as u32)?,
                    0b011 => conn.memory.store_u64(cap, (conn.sreg.read_u64(rs2)? & 0xFFFF_FFFF_FFFF_FFFF) as u64)?,
                    
                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3)))
                };
            }

            _ => bail!("Invalid opcode/instruction pair passed to RV32I")
        }

        Ok(None)
    }
}