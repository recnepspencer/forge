use forge_store_recovery_physics::PageLsn;

use super::{
    PhysicalIsolationEntryDenial, PhysicalIsolationEntryEvidence, PhysicalIsolationEntryIdentity,
    PhysicalIsolationEntryRebindRequired, PhysicalIsolationEntryRequest,
    PhysicalIsolationRootEpochBasis,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationEntryAdmission {
    identity: PhysicalIsolationEntryIdentity,
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    replayed_frames: usize,
    source_candidate_count: usize,
    evidence: PhysicalIsolationEntryEvidence,
}

pub fn admit_physical_isolation_entry(
    request: PhysicalIsolationEntryRequest<'_>,
) -> Result<PhysicalIsolationEntryAdmission, PhysicalIsolationEntryDenial> {
    PhysicalIsolationEntryAdmission::admit(request)
}

pub fn admit_physical_isolation_entry_checked(
    request: PhysicalIsolationEntryRequest<'_>,
) -> PhysicalIsolationEntryCheckedOutcome {
    match admit_physical_isolation_entry(request) {
        Ok(admission) => PhysicalIsolationEntryCheckedOutcome::Admitted(admission),
        Err(denial) => PhysicalIsolationEntryCheckedOutcome::Denied(denial),
    }
}

#[derive(Debug, Clone)]
pub enum PhysicalIsolationEntryCheckedOutcome {
    Admitted(PhysicalIsolationEntryAdmission),
    Denied(PhysicalIsolationEntryDenial),
    Stale(PhysicalIsolationEntryDenial),
    RebindRequired(PhysicalIsolationEntryRebindRequired),
}

impl PartialEq for PhysicalIsolationEntryCheckedOutcome {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Admitted(left), Self::Admitted(right)) => left.identity() == right.identity(),
            (Self::Denied(left), Self::Denied(right)) | (Self::Stale(left), Self::Stale(right)) => {
                left == right
            }
            (Self::RebindRequired(left), Self::RebindRequired(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for PhysicalIsolationEntryCheckedOutcome {}

impl PhysicalIsolationEntryAdmission {
    fn admit(
        request: PhysicalIsolationEntryRequest<'_>,
    ) -> Result<Self, PhysicalIsolationEntryDenial> {
        let admission = request.recovery_readiness().admit_for_s5_startup()?;
        let source_decision_digest = request
            .recovery_readiness()
            .source_precedence_trace()
            .canonical_replay_digest();
        let identity = PhysicalIsolationEntryIdentity::new(
            admission.recovered_root(),
            admission.admitted_page_lsn_frontier(),
            &source_decision_digest,
            admission.replayed_frames(),
            admission.source_candidate_count(),
        );
        let root_epoch_basis = identity.root_epoch_basis();
        let evidence = PhysicalIsolationEntryEvidence::from_entry_identity(&identity);
        Ok(Self {
            identity,
            root_epoch_basis,
            recovered_root: admission.recovered_root().to_string(),
            admitted_page_lsn_frontier: admission.admitted_page_lsn_frontier(),
            replayed_frames: admission.replayed_frames(),
            source_candidate_count: admission.source_candidate_count(),
            evidence,
        })
    }

    pub const fn identity(&self) -> &PhysicalIsolationEntryIdentity {
        &self.identity
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub const fn replayed_frames(&self) -> usize {
        self.replayed_frames
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.source_candidate_count
    }

    pub const fn evidence(&self) -> &PhysicalIsolationEntryEvidence {
        &self.evidence
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}
