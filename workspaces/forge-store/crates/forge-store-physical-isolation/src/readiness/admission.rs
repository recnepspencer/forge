use forge_store_recovery_physics::{PageLsn, S5RecoveryReadinessAdmission};

use super::{
    PhysicalIsolationEntryDenial, PhysicalIsolationEntryEvidence, PhysicalIsolationEntryIdentity,
    PhysicalIsolationEntryRebindRequired, PhysicalIsolationEntryRequest,
    PhysicalIsolationRootEpochBasis,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationEntryAdmission {
    recovery_admission: S5RecoveryReadinessAdmission,
    identity: PhysicalIsolationEntryIdentity,
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
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
        let recovery_admission = verify_s5_startup_admission(request)?;
        let identity = derive_entry_identity_from_admission(request, &recovery_admission);
        let root_epoch_basis = identity.root_epoch_basis();
        let evidence = seal_physical_isolation_entry_evidence(&identity);
        Ok(Self {
            recovery_admission,
            identity,
            root_epoch_basis,
            evidence,
        })
    }

    pub const fn recovery_admission(&self) -> &S5RecoveryReadinessAdmission {
        &self.recovery_admission
    }

    pub const fn identity(&self) -> &PhysicalIsolationEntryIdentity {
        &self.identity
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }

    pub fn recovered_root(&self) -> &str {
        self.recovery_admission.recovered_root()
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.recovery_admission.admitted_page_lsn_frontier()
    }

    pub const fn replayed_frames(&self) -> usize {
        self.recovery_admission.replayed_frames()
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.recovery_admission.source_candidate_count()
    }

    pub const fn evidence(&self) -> &PhysicalIsolationEntryEvidence {
        &self.evidence
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}

fn verify_s5_startup_admission(
    request: PhysicalIsolationEntryRequest<'_>,
) -> Result<S5RecoveryReadinessAdmission, PhysicalIsolationEntryDenial> {
    request
        .recovery_readiness()
        .admit_for_s5_startup()
        .map_err(PhysicalIsolationEntryDenial::from)
}

fn derive_entry_identity_from_admission(
    request: PhysicalIsolationEntryRequest<'_>,
    admission: &S5RecoveryReadinessAdmission,
) -> PhysicalIsolationEntryIdentity {
    let source_decision_digest = request
        .recovery_readiness()
        .source_precedence_trace()
        .canonical_replay_digest();
    PhysicalIsolationEntryIdentity::new(
        admission.recovered_root(),
        admission.admitted_page_lsn_frontier(),
        &source_decision_digest,
        admission.replayed_frames(),
        admission.source_candidate_count(),
    )
}

fn seal_physical_isolation_entry_evidence(
    identity: &PhysicalIsolationEntryIdentity,
) -> PhysicalIsolationEntryEvidence {
    PhysicalIsolationEntryEvidence::from_entry_identity(identity)
}