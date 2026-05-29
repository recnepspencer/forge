mod checkpointing;
mod compatibility;
mod diagnostics;
mod recovery;
mod runtime_rebuild;

use crate::logic::runtime::RelationalRuntime;

pub struct DurabilityAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> DurabilityAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

impl RelationalRuntime {
    pub fn durability_authority(&mut self) -> DurabilityAuthority<'_> {
        DurabilityAuthority::new(self)
    }
}
