extern crate clap;
use clap::{Arg, App};

use anyhow::Result;

use rsim::models::{Processor,Processor32};
use rsim::memory::{AggregateMemory32,MemoryBacking,IOMemory};

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
                .possible_values(&["rv32iv", "rv64i"])
            )
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
            let mem = AggregateMemory32::from_mappings(vec![
                // Allocate 4KB for the program
                Box::new(MemoryBacking::from_file(memory_bin, 0x0..0x1000)),
                // Allocate ~96KB for RAM
                Box::new(MemoryBacking::zeros(0x1000..0x25_000)),
                // Add one I/O memory address, which expects 0x3FFF as a return value
                Box::new(IOMemory::return_address(0xF000_0000, 0x3FFF))
            ]);

            // Create the processor and vector unit
            let (processor, mods) = Processor32::new(mem);

            run_binary_in_processor(Box::new(processor), mods)
        }
        _ => unreachable!("invalid subcommand name")
    }

    
}
