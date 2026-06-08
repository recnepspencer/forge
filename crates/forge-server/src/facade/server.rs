use std::io;

use crate::{
    diagnostics::ForgeServerCounterSnapshot, registration::ForgeServerSurfaceInventory,
    runtime::ForgeServerRuntime, transport::serve_runtime,
};

use super::builder::ForgeServerBuilder;

#[derive(Debug)]
pub struct ForgeServer {
    runtime: ForgeServerRuntime,
}

impl ForgeServer {
    pub fn builder() -> ForgeServerBuilder {
        ForgeServerBuilder::default()
    }

    pub(crate) fn new(runtime: ForgeServerRuntime) -> Self {
        Self { runtime }
    }

    pub fn surface_inventory(&self) -> ForgeServerSurfaceInventory {
        self.runtime.assembly().surface_registry().inventory()
    }

    pub fn counters(&self) -> ForgeServerCounterSnapshot {
        self.runtime.assembly().counters().snapshot()
    }

    pub async fn serve(self) -> io::Result<()> {
        serve_runtime(self.runtime).await
    }
}
