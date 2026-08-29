mod append_authority;
mod authority_continuity;
mod checkpoint_capture;
mod checkpoint_image;
mod checkpointing;
mod diagnostics;
mod recovery;
mod runtime_rebuild;

use crate::runtime::RelationalRuntime;

pub(crate) use append_authority::DurableAppendAuthority;

pub struct DurabilityAuthority<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> DurabilityAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }
}

/// The exclusive recovery lane.
///
/// Recovery rebuilds a whole runtime from durable evidence and then installs it
/// in place of the live one, so it is the single durability operation that
/// cannot run against a shared borrow. It is a separate wrapper rather than a
/// method on the shared authority so the exclusive requirement is visible at
/// every call site instead of infecting ordinary durability work.
pub struct DurabilityRecoveryAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> DurabilityRecoveryAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

impl RelationalRuntime {
    pub fn durability_authority(&self) -> DurabilityAuthority<'_> {
        DurabilityAuthority::new(self)
    }

    pub fn durability_recovery(&mut self) -> DurabilityRecoveryAuthority<'_> {
        DurabilityRecoveryAuthority::new(self)
    }
}
