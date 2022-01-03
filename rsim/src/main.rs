extern crate clap;
use clap::{Arg, App};

use rsim::{Memory,Processor};

fn main() {
    let matches = App::new("risc-v-v-lite")
        .version("1.0")
        .author("Samuel S. <popgoestoast@gmail.com>")
        .about("Simplistic RISC-V emulator for Vector extension")
        .arg(
            Arg::with_name("memory_bin")
            .required(true)
            .index(1)
        )
        .get_matches();

    let memory_bin = matches.value_of("memory_bin").unwrap();
    let mem = Memory::new_from_file(memory_bin, 640_000);
    let (mut processor, mut v_unit) = Processor::new(mem);

    loop {
        let res = processor.exec_step(&mut v_unit);

        match res {
            Err(e) => {
                processor.dump(&mut v_unit);
                println!("Encountered error: {:#}", e);
                break
            },
            Ok(()) => {}
        }
        if !processor.running {
            break
        }
    }
}
