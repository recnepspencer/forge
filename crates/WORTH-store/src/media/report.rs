use crate::media::DurabilityBarrierClass;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DurableBackendFamily {
    InMemory,
    LocalFileAtomicRewrite,
    SqliteTransactional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DurableMediaReport {
    backend_family: DurableBackendFamily,
    content_barrier: DurabilityBarrierClass,
    metadata_barrier: DurabilityBarrierClass,
    ack_required_barrier: DurabilityBarrierClass,
}

impl DurableMediaReport {
    pub(crate) fn new(
        backend_family: DurableBackendFamily,
        content_barrier: DurabilityBarrierClass,
        metadata_barrier: DurabilityBarrierClass,
        ack_required_barrier: DurabilityBarrierClass,
    ) -> Self {
        Self {
            backend_family,
            content_barrier,
            metadata_barrier,
            ack_required_barrier,
        }
    }

    pub fn backend_family(&self) -> DurableBackendFamily {
        self.backend_family
    }

    pub fn content_barrier(&self) -> DurabilityBarrierClass {
        self.content_barrier
    }

    pub fn metadata_barrier(&self) -> DurabilityBarrierClass {
        self.metadata_barrier
    }

    pub fn ack_required_barrier(&self) -> DurabilityBarrierClass {
        self.ack_required_barrier
    }
}
