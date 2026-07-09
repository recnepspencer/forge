use super::family::{
    WorthQueryOrchestrationCheckedTopologyKind, WorthQueryOrchestrationSupportSurface,
    WorthQueryOrchestrationTranscriptFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationProofContract {
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    transcript_family: WorthQueryOrchestrationTranscriptFamily,
    checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind,
    support_surface: WorthQueryOrchestrationSupportSurface,
}

impl WorthQueryOrchestrationProofContract {
    pub const fn new(
        checked_type_name: &'static str,
        proof_type_name: &'static str,
        transcript_family: WorthQueryOrchestrationTranscriptFamily,
        checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind,
        support_surface: WorthQueryOrchestrationSupportSurface,
    ) -> Self {
        Self {
            checked_type_name,
            proof_type_name,
            transcript_family,
            checked_topology_kind,
            support_surface,
        }
    }

    pub fn checked_type_name(&self) -> &'static str {
        self.checked_type_name
    }

    pub fn proof_type_name(&self) -> &'static str {
        self.proof_type_name
    }

    pub fn transcript_family(&self) -> WorthQueryOrchestrationTranscriptFamily {
        self.transcript_family
    }

    pub fn checked_topology_kind(&self) -> WorthQueryOrchestrationCheckedTopologyKind {
        self.checked_topology_kind
    }

    pub fn support_surface(&self) -> WorthQueryOrchestrationSupportSurface {
        self.support_surface
    }
}
