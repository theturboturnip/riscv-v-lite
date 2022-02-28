use anyhow::Result;

pub trait Processor<TModules> {
    fn reset(&mut self, mods: &mut TModules);
    fn exec_step(&mut self, mods: &mut TModules) -> Result<()>;
    fn dump(&self, mods: &TModules);
    fn running(&self) -> bool;
}

mod rv32iv;
pub use rv32iv::{Processor32,ProcessorModules32};
mod rv64i;
pub use rv64i::{Rv64iProcessor,Rv64iProcessorModules};