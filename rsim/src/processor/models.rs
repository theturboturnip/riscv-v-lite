use anyhow::Result;

pub trait Processor<TModules> {
    fn reset(&mut self, mods: &mut TModules);
    fn exec_step(&mut self, mods: &mut TModules) -> Result<()>;
    fn dump(&self, mods: &TModules);
    fn running(&self) -> bool;
}

mod rv32imv;
pub use rv32imv::{Processor32,ProcessorModules32};
mod rv64imv;
pub use rv64imv::{Rv64imvProcessor,Rv64imvProcessorModules};
mod rv64imvxcheri;
pub use rv64imvxcheri::{Rv64imvXCheriProcessor,Rv64imvXCheriProcessorModules};