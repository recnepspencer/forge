use super::family::{
    ForgeQueryOrchestrationCheckedTopologyKind, ForgeQueryOrchestrationSupportSurface,
    ForgeQueryOrchestrationTranscriptFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationProofContract {
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    support_surface: ForgeQueryOrchestrationSupportSurface,
}

impl ForgeQueryOrchestrationProofContract {
    pub const fn new(
        checked_type_name: &'static str,
        proof_type_name: &'static str,
        transcript_family: ForgeQueryOrchestrationTranscriptFamily,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
        support_surface: ForgeQueryOrchestrationSupportSurface,
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

    pub fn transcript_family(&self) -> ForgeQueryOrchestrationTranscriptFamily {
        self.transcript_family
    }

    pub fn checked_topology_kind(&self) -> ForgeQueryOrchestrationCheckedTopologyKind {
        self.checked_topology_kind
    }

    pub fn support_surface(&self) -> ForgeQueryOrchestrationSupportSurface {
        self.support_surface
    }
}
