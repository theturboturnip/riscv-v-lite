extern crate clap;
use clap::{Arg, App};

use anyhow::{Result,bail};

use rsim::models::{Processor,Processor32,Rv64imProcessor,Rv64imvXCheriProcessor};
use rsim::memory::{CheriAggregateMemory,AggregateMemory64,AggregateMemory32,MemoryBacking,IOMemory};
use rsim::{Cc128,CompressedCapability,CheriRVFuncs};

use object::read::{Object,ObjectSection,ObjectSegment};
use object::SegmentFlags;

/// __cap_relocs section structure.
/// 
/// See TR-949, [crt_init_globals.c](https://github.com/CTSRD-CHERI/device-model/blob/master/src/crt_init_globals.c)
/// 
/// This is used to create initial capabilities at startup.
/// On full systems the C runtime(?) does this, but we do it manually here.
#[repr(C)]
#[derive(Default,Debug,Clone,Copy)]
struct CapReloc {
    capability_location: u64,
    object: u64,
    offset: u64,
    size: u64,
    permissions: u64,
}

const FUNCTION_RELOC_FLAG: u64 = 1 << 63;
const FUNCTION_POINTER_PERMISSIONS: u32 =
	!0 &
	!Cc128::PERM_STORE_CAP &
	!Cc128::PERM_STORE;
const GLOBAL_POINTER_PERMISSIONS: u32 = !0 & !Cc128::PERM_EXECUTE;

fn load_cheri_elf(elf_path: &str) -> Result<(u64, CheriAggregateMemory)> {
    let bin_data = std::fs::read(elf_path)?;

    let obj_file = object::File::parse(&*bin_data)?;

    let mut code_data = Vec::<u8>::default();

    for segment in obj_file.segments() {
        let flags = segment.flags();
        let (r,w,x) = match flags {
            SegmentFlags::None => (false, false, false),
            SegmentFlags::Elf{p_flags} => (p_flags & 0x4 != 0, p_flags & 0x2 != 0, p_flags & 0x1 != 0),
            _ => bail!("non-elf section") 
        };
        println!("Segment: file offset/size {:x?}, vaddr: 0x{:x}, flags: r {} w {} x {}", segment.file_range(), segment.address(), r,w,x);
        code_data.extend_from_slice(segment.data()?);
    }

    let agg_mem = AggregateMemory64::from_mappings(vec![
        // Allocate 4KB for the program
        // Load in the code+data sections
        Box::new(MemoryBacking::from_vec(code_data, 0x0..0x2000)),
        // Allocate ~96KB for RAM
        Box::new(MemoryBacking::zeros(0x2000..0x25_000)),
        // Add one I/O memory address, which expects 0x3FFF as a return value
        Box::new(IOMemory::return_address(0xF000_0000, 0x3FFF))
    ]);
    let mut cheri_mem = CheriAggregateMemory::from_base(agg_mem);

    // Check .captable section
    let num_global_caps = if let Some(section) = obj_file.section_by_name(".captable") {
        assert!(section.size() % 16 == 0, ".captable entry must be a multiple of 16 bytes");

        println!(".captable: virtual address {:x}, num: {}", section.address(), section.size() / 16);

        section.size() / 16
    } else {
        bail!("The ELF file {} does not contain a .captable entry", elf_path);
    };

    // Perform cap relocations specified in __cap_relocs
    if let Some(section) = obj_file.section_by_name("__cap_relocs") {
        assert!(section.size() % (std::mem::size_of::<CapReloc>() as u64) == 0, "__cap_relocs should be an array of cap_reloc structures");
        assert!(section.size() / (std::mem::size_of::<CapReloc>() as u64) == num_global_caps, "__cap_relocs should have one element per global capability");

        // Create a vector of CapReloc structures, which will be correctly aligned etc.
        let mut relocs = vec![CapReloc::default(); num_global_caps as usize];
        // Copy the bytes from the section data over the relocs
        unsafe {
            std::intrinsics::copy_nonoverlapping(
                section.data()?.as_ptr(),
                relocs.as_mut_ptr() as *mut u8,
                (num_global_caps as usize) * std::mem::size_of::<CapReloc>()
            );
        }
        // Now relocs contains the actual relocations we want

        // Generate the base data and program capabilities
        // TODO - should really make these only correspond to valid text/data segments
        // rather than the whole address space
        let full_cap = cheri_mem.get_full_range_cap();
        let mut global_data_cap = full_cap.clone();
        global_data_cap.set_permissions(global_data_cap.permissions() & GLOBAL_POINTER_PERMISSIONS);
        let mut function_cap = full_cap.clone();
        function_cap.set_permissions(function_cap.permissions() & FUNCTION_POINTER_PERMISSIONS);

        for reloc in relocs {
            println!("{:x?}", reloc);

            // Get a capability pointing to the capability we're relocating
            let dst = Cc128::setCapOffset(&global_data_cap, reloc.capability_location).1;

            // Find the base cap to reduce
            let is_function = (reloc.permissions & FUNCTION_RELOC_FLAG) != 0;
            let base_cap = if is_function { &function_cap } else { &global_data_cap };
            // setOffset(cap, reloc.object) is not a typo - "reloc.object" is the address of the "object" this relocated cap should point at
            let (_, mut relocated_cap) = Cc128::setCapOffset(base_cap, reloc.object);
            if is_function {
                // TODO - why doesn't crt_init_globals.c do this?
                relocated_cap.set_otype(Cc128::OTYPE_SENTRY);
                // Set the function to execute in pure-capability mode
                // TODO - why doesn't crt_init_globals.c do this?
                relocated_cap.set_flags(1);
            } else if reloc.size != 0 {
                // setCapBounds returns (exact, cap), we don't care if it's not exact so just take the second element
                relocated_cap = Cc128::setCapBounds(
                    &relocated_cap,
                    relocated_cap.address(),
                    (relocated_cap.address() + reloc.size).into()
                ).1;
            }
            relocated_cap = Cc128::incCapOffset(&relocated_cap, reloc.offset).1;
            cheri_mem.store_cap(dst, relocated_cap)?;
        }
    } else {
        bail!("The ELF file {} does not contain a __cap_relocs entry", elf_path);
    }

    // obj_file.entry() returns the address of .text, *not* .text.init.
    let entry_pc = if let Some(section) = obj_file.section_by_name(".text.init") {
        section.address()
    } else {
        bail!("The ELF file {} does not contain a .text.init entry", elf_path);
    };

    Ok((entry_pc, cheri_mem))
}

fn run_binary_in_processor<T>(mut processor: Box<dyn Processor<T>>, mut mods: T) -> Result<()> where T: Sized {
    loop {
        let res = processor.exec_step(&mut mods);

        match res {
            Err(e) => {
                processor.dump(&mods);
                return Err(e)
            },
            Ok(()) => {}
        }
        if !processor.running() {
            break
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("risc-v-v-lite")
        .version("1.0")
        .author("Samuel S. <popgoestoast@gmail.com>")
        .about("Simplistic RISC-V emulator for Vector extension")
        .subcommand(App::new("direct")
            .about("Run a RISC-V program binary directly")
            .arg(Arg::with_name("riscv_profile")
                .required(true)
                .index(1)
                .possible_values(&["rv32imv", "rv64im", "rv64imvxcheri"])
            )
            .arg(
                Arg::with_name("memory_bin")
                .required(true)
                .index(2)
            ))
        
        .get_matches();

    match matches.subcommand() {
        ("direct", Some(sub_matches)) => {
            // Get the filepath for program memory
            let memory_bin = sub_matches.value_of("memory_bin").unwrap();

            match sub_matches.value_of("riscv_profile") {
                Some("rv32imv") => {
                    // Create the memory map
                    let mem = AggregateMemory32::from_mappings(vec![
                        // Allocate 4KB for the program
                        Box::new(MemoryBacking::from_file(memory_bin, 0x0..0x2000)),
                        // Allocate ~96KB for RAM
                        Box::new(MemoryBacking::zeros(0x2000..0x25_000)),
                        // Add one I/O memory address, which expects 0x3FFF as a return value
                        Box::new(IOMemory::return_address(0xF000_0000, 0x3FFF))
                    ]);

                    let (processor, mods) = Processor32::new(mem);
                    run_binary_in_processor(Box::new(processor), mods)
                },
                Some("rv64im") => {
                    // Create the memory map
                    let mem = AggregateMemory64::from_mappings(vec![
                        // Allocate 4KB for the program
                        Box::new(MemoryBacking::from_file(memory_bin, 0x0..0x2000)),
                        // Allocate ~96KB for RAM
                        Box::new(MemoryBacking::zeros(0x2000..0x25_000)),
                        // Add one I/O memory address, which expects 0x3FFF as a return value
                        Box::new(IOMemory::return_address(0xF000_0000, 0x3FFF))
                    ]);

                    let (processor, mods) = Rv64imProcessor::new(mem);
                    run_binary_in_processor(Box::new(processor), mods)
                },
                Some("rv64imvxcheri") => {
                    // Create the memory map
                    let (start_pc, cheri_mem) = load_cheri_elf(memory_bin)?;

                    let (processor, mods) = Rv64imvXCheriProcessor::new(start_pc, cheri_mem);
                    run_binary_in_processor(Box::new(processor), mods)
                },
                _ => unreachable!("invalid riscv profile")
            }

        }
        _ => unreachable!("invalid subcommand name")
    }

    
}
