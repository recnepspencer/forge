mod append_authority;
mod authority_continuity;
mod checkpointing;
mod diagnostics;
mod recovery;
mod runtime_rebuild;

use crate::runtime::RelationalRuntime;

pub(crate) use append_authority::DurableAppendAuthority;

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
