use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQuerySharedReadPinningBoundaryPosture {
    Open,
    Partial,
    Closed,
}

impl ForgeQuerySharedReadPinningBoundaryPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Partial => "partial",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningBoundaryEvidence {
    inventory_failure_count: usize,
    counter_residue_count: usize,
    hostile_matrix_certified: bool,
    send_sync_proven: bool,
    stale_basis_denial_proven: bool,
}

impl ForgeQuerySharedReadPinningBoundaryEvidence {
    pub(crate) fn new(
        inventory_failure_count: usize,
        counter_residue_count: usize,
        hostile_matrix_certified: bool,
        send_sync_proven: bool,
        stale_basis_denial_proven: bool,
    ) -> Self {
        Self {
            inventory_failure_count,
            counter_residue_count,
            hostile_matrix_certified,
            send_sync_proven,
            stale_basis_denial_proven,
        }
    }

    pub fn inventory_failure_count(&self) -> usize {
        self.inventory_failure_count
    }

    pub fn counter_residue_count(&self) -> usize {
        self.counter_residue_count
    }

    pub fn hostile_matrix_certified(&self) -> bool {
        self.hostile_matrix_certified
    }

    pub fn send_sync_proven(&self) -> bool {
        self.send_sync_proven
    }

    pub fn stale_basis_denial_proven(&self) -> bool {
        self.stale_basis_denial_proven
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningBoundaryClosure {
    posture: ForgeQuerySharedReadPinningBoundaryPosture,
    inventory_failure_count: usize,
    counter_residue_count: usize,
    hostile_matrix_green: bool,
    send_sync_proven: bool,
    stale_basis_denial_proven: bool,
    closure_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQuerySharedReadPinningBoundaryClosure {
    pub(crate) fn derive_from_evidence(
        evidence: &ForgeQuerySharedReadPinningBoundaryEvidence,
    ) -> Self {
        let posture = classify_shared_read_pinning_boundary_posture(
            evidence.inventory_failure_count(),
            evidence.counter_residue_count(),
            evidence.hostile_matrix_certified(),
            evidence.send_sync_proven(),
            evidence.stale_basis_denial_proven(),
        );
        let closure_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_shape(
                    ForgeQueryEvidenceTag::new("shared_read_pinning_boundary_posture"),
                    posture.as_str(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("inventory_failure_count"),
                    evidence.inventory_failure_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("counter_residue_count"),
                    evidence.counter_residue_count(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("hostile_matrix_green"),
                    evidence.hostile_matrix_certified(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("send_sync_proven"),
                    evidence.send_sync_proven(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("stale_basis_denial_proven"),
                    evidence.stale_basis_denial_proven(),
                )
                .seal();
        Self {
            posture,
            inventory_failure_count: evidence.inventory_failure_count(),
            counter_residue_count: evidence.counter_residue_count(),
            hostile_matrix_green: evidence.hostile_matrix_certified(),
            send_sync_proven: evidence.send_sync_proven(),
            stale_basis_denial_proven: evidence.stale_basis_denial_proven(),
            closure_identity,
        }
    }

    pub fn posture(&self) -> ForgeQuerySharedReadPinningBoundaryPosture {
        self.posture
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    #[allow(dead_code)]
    pub fn inventory_failure_count(&self) -> usize {
        self.inventory_failure_count
    }

    #[allow(dead_code)]
    pub fn counter_residue_count(&self) -> usize {
        self.counter_residue_count
    }

    #[allow(dead_code)]
    pub fn hostile_matrix_green(&self) -> bool {
        self.hostile_matrix_green
    }

    #[allow(dead_code)]
    pub fn send_sync_proven(&self) -> bool {
        self.send_sync_proven
    }

    #[allow(dead_code)]
    pub fn stale_basis_denial_proven(&self) -> bool {
        self.stale_basis_denial_proven
    }
}

fn classify_shared_read_pinning_boundary_posture(
    inventory_failure_count: usize,
    counter_residue_count: usize,
    hostile_matrix_green: bool,
    send_sync_proven: bool,
    stale_basis_denial_proven: bool,
) -> ForgeQuerySharedReadPinningBoundaryPosture {
    if inventory_failure_count == 0
        && counter_residue_count == 0
        && hostile_matrix_green
        && send_sync_proven
        && stale_basis_denial_proven
    {
        return ForgeQuerySharedReadPinningBoundaryPosture::Closed;
    }
    if inventory_failure_count > 0 && counter_residue_count > 0 {
        return ForgeQuerySharedReadPinningBoundaryPosture::Open;
    }
    ForgeQuerySharedReadPinningBoundaryPosture::Partial
}
