use worth_store_recovery_physics::{PageLsn, RecoveryCompletion};

use super::{
    PhysicalIsolationEntryDenial, PhysicalIsolationEntryEvidence, PhysicalIsolationEntryIdentity,
    PhysicalIsolationEntryRebindRequired, PhysicalIsolationEntryRequest,
    PhysicalIsolationRootEpochBasis,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationEntryAdmission {
    recovery_completion: RecoveryCompletion,
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
        let recovery_completion = request.recovery_completion().clone();
        let identity = derive_entry_identity_from_completion(
            &recovery_completion,
            request.store_authority_identity(),
        );
        let root_epoch_basis = identity.root_epoch_basis();
        let evidence = seal_physical_isolation_entry_evidence(&identity);
        Ok(Self {
            recovery_completion,
            identity,
            root_epoch_basis,
            evidence,
        })
    }

    pub const fn recovery_completion(&self) -> &RecoveryCompletion {
        &self.recovery_completion
    }

    pub const fn identity(&self) -> &PhysicalIsolationEntryIdentity {
        &self.identity
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }

    pub fn recovered_root(&self) -> &str {
        self.recovery_completion.recovered_root()
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.recovery_completion.admitted_page_lsn_frontier()
    }

    pub const fn replayed_frames(&self) -> usize {
        self.recovery_completion.replayed_frames()
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.recovery_completion.source_candidate_count()
    }

    pub const fn evidence(&self) -> &PhysicalIsolationEntryEvidence {
        &self.evidence
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}

fn derive_entry_identity_from_completion(
    completion: &RecoveryCompletion,
    store_authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
) -> PhysicalIsolationEntryIdentity {
    PhysicalIsolationEntryIdentity::new(
        completion.recovered_root(),
        completion.admitted_page_lsn_frontier(),
        completion.source_decision_digest(),
        completion.replayed_frames(),
        completion.source_candidate_count(),
        store_authority_identity,
    )
}

fn seal_physical_isolation_entry_evidence(
    identity: &PhysicalIsolationEntryIdentity,
) -> PhysicalIsolationEntryEvidence {
    PhysicalIsolationEntryEvidence::from_entry_identity(identity)
}
