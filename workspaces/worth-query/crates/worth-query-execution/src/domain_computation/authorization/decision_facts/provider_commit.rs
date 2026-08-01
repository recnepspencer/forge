use super::{
    BridgeAuthorizationRuntime, WorthQueryAuthorizationDecisionFact,
    WorthQueryCapabilityCommitBasis, WorthQueryOperationAdmissionIdentity,
    WorthQueryPrincipalCurrentnessDependency,
};

pub(in crate::domain_computation) struct WorthQueryProviderAuthorizationDecisionFacts {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decisions: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryProviderAuthorizationDecisionFacts {
    pub(super) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self {
            principal,
            decisions,
        }
    }

    pub(in crate::domain_computation) fn into_parts(
        self,
    ) -> (
        WorthQueryPrincipalCurrentnessDependency,
        Vec<WorthQueryAuthorizationDecisionFact>,
    ) {
        (self.principal, self.decisions)
    }
}

pub(in crate::domain_computation) enum WorthQueryCommitAuthorizationBasis {
    Observed {
        admission_identity: WorthQueryOperationAdmissionIdentity,
        authorization: WorthQueryObservedCommitBasis,
    },
    Capability {
        admission_identity: WorthQueryOperationAdmissionIdentity,
        authorization: WorthQueryCapabilityCommitBasis,
    },
}

impl WorthQueryCommitAuthorizationBasis {
    pub(in crate::domain_computation::authorization) const fn admission_identity(
        &self,
    ) -> WorthQueryOperationAdmissionIdentity {
        match self {
            Self::Observed {
                admission_identity, ..
            }
            | Self::Capability {
                admission_identity, ..
            } => *admission_identity,
        }
    }

    pub(in crate::domain_computation::authorization) fn belongs_to_session(
        &self,
        session_identity: &worth_foundational::facade::CanonicalDigestId,
    ) -> bool {
        match self {
            Self::Observed { authorization, .. } => {
                authorization.belongs_to_session(session_identity)
            }
            Self::Capability { authorization, .. } => {
                authorization.belongs_to_session(session_identity)
            }
        }
    }

    pub(in crate::domain_computation::authorization) fn belongs_to_branch(
        &self,
        branch_id: &worth_relational::facade::history::BranchId,
    ) -> bool {
        match self {
            Self::Observed { authorization, .. } => authorization.belongs_to_branch(branch_id),
            Self::Capability { authorization, .. } => authorization.belongs_to_branch(branch_id),
        }
    }
}

pub(in crate::domain_computation) struct WorthQueryObservedCommitBasis {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decisions: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryObservedCommitBasis {
    pub(super) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self {
            principal,
            decisions,
        }
    }

    pub(in crate::domain_computation::authorization) fn principal_remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        self.principal.remains_current_in(runtime, snapshot)
    }

    pub(in crate::domain_computation::authorization) fn decisions_remain_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        self.decisions
            .iter()
            .all(|fact| fact.remains_current_in(runtime, snapshot, bridge))
    }

    pub(in crate::domain_computation::authorization) fn belongs_to_session(
        &self,
        session_identity: &worth_foundational::facade::CanonicalDigestId,
    ) -> bool {
        self.principal.session_identity() == session_identity
            && self
                .decisions
                .iter()
                .all(|fact| fact.session_identity() == session_identity)
    }

    pub(in crate::domain_computation::authorization) fn belongs_to_branch(
        &self,
        branch_id: &worth_relational::facade::history::BranchId,
    ) -> bool {
        self.principal.branch_id() == branch_id
            && self
                .decisions
                .iter()
                .all(|fact| fact.branch_id() == branch_id)
    }
}
