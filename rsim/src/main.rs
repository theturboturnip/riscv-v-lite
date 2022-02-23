extern crate clap;
use clap::{Arg, App};

use anyhow::Result;

use rsim::{Processor};
use rsim::memory::{AggregateMemory,MemoryBacking,IOMemory};

fn main() -> Result<()> {
    let matches = App::new("risc-v-v-lite")
        .version("1.0")
        .author("Samuel S. <popgoestoast@gmail.com>")
        .about("Simplistic RISC-V emulator for Vector extension")
        .subcommand(App::new("direct")
            .about("Run a RISC-V program binary directly")
            .arg(
                Arg::with_name("memory_bin")
                .required(true)
                .index(1)
            ))
        
        .get_matches();

    match matches.subcommand() {
        ("direct", Some(sub_matches)) => {
            // Get the filepath for program memory
            let memory_bin = sub_matches.value_of("memory_bin").unwrap();
            // Create the memory map
            let mem = AggregateMemory::from_mappings(vec![
                // Allocate 4KB for the program
                MemoryBacking::from_file(memory_bin, 0x0..0x1000),
                // Allocate ~400KB for RAM
                MemoryBacking::zeros(0x1000..0x99cf0),
                // Add one I/O memory address
                IOMemory::return_address(0xF000_0000)
            ]);

            // Create the processor and vector unit
            let (mut processor, mut v_unit) = Processor::new(mem);

            loop {
                let res = processor.exec_step(&mut v_unit);

                match res {
                    Err(e) => {
                        processor.dump(&mut v_unit);
                        return Err(e)
                    },
                    Ok(()) => {}
                }
                if !processor.running {
                    break
                }
            }

            Ok(())
        }
        _ => unreachable!("invalid subcommand name")
    }

    
}
