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
                let addr = conn.pcc.address().wrapping_add(imm.sign_extend_u64());
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
                let offset = imm.sign_extend_u64();
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address().wrapping_add(offset));

                // Load a capability from cap_with_addr
                let loaded_cap = conn.memory.load_maybe_cap(cap)?;
                // Store it into the register file
                conn.sreg.write_maybe_cap(rd, loaded_cap)?;
            },
            (Store, InstructionBits::SType{funct3: 0x4, rs1, rs2, imm}) => {
                // SC = Store Capability

                let offset = imm.sign_extend_u64();
                let mut cap = conn.sreg.read_maybe_cap(rs1)?.to_cap();
                cap.set_address_unchecked(cap.address().wrapping_add(offset));

                let cap_to_store = conn.sreg.read_maybe_cap(rs2)?;
                conn.memory.store_maybe_cap(cap, cap_to_store)?;
            },
            (JumpAndLinkRegister, InstructionBits::IType{funct3, imm, rd, rs1}) => {
                // Vanilla JALR jumps with an immediate offset, unlike CJALR.
                // It appears CHERI-Clang emits vanilla JALRs rather than CJALRs, even in capability mode.
                // To account for this, we handle vanilla JALRs as CJALRs.
                // JALR supports an immediate offset (although technically CJALR doesn't?)
                if funct3 == 0 {
                    return Ok(Some(self.handle_cjalr(rd, rs1, imm.sign_extend_u64(), conn)?))
                } else {
                    bail!("Vanilla nonzero-offset JumpAndLinkRegister in Capability Mode")
                }
            }
            (Custom2CHERI, InstructionBits::ROrIType{rd, funct3, rs1, rs2, funct7, imm}) => {
                match (funct7, funct3) {
                    (0x1, 0x0) => {
                        // CSpecialRW
                        bail!("Haven't implemented CSpecialRW")
                    }
                    (0x8, 0x0) => {
                        // CSetBounds
                        bail!("Haven't implemented CSetBounds")
                    }
                    (0x9, 0x0) => {
                        // CSetBoundsExact
                        bail!("Haven't implemented CSetBoundsExact")
                    }
                    (_, 0x2) => {
                        // CSetBoundsImm
                        bail!("Haven't implemented CSetBoundsImm")
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
                        bail!("Haven't implemented CIncOffset")
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
                        bail!("Haven't implemented CFromPtr")
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
                            bail!("Haven't implemented CGetTag")
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
                            bail!("Haven't implemented CMove")
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
                        _ => bail!("Invalid rs2 value {:x} for CHERI funct3=0x0,funct7=0x7f", rs2)
                    }
                    _ => bail!("Invalid funct3/funct7 combination {:x} {:x}", funct3, funct7)
                }
            }

            _ => bail!("Invalid opcode/instruction pair passed to XCheri64")   
        }
        Ok(None)
    }
}
