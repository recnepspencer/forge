#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantProjectionDenialKind {
    BasisUnavailable,
    ForeignBasis,
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    SnapshotIdentityExhausted,
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    WorkBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantProjectionDenial {
    kind: WorthQueryInvariantProjectionDenialKind,
}

impl WorthQueryInvariantProjectionDenial {
    pub const fn kind(&self) -> WorthQueryInvariantProjectionDenialKind {
        self.kind
    }

    pub(super) const fn basis_unavailable() -> Self {
        Self {
            kind: WorthQueryInvariantProjectionDenialKind::BasisUnavailable,
        }
    }

    pub(super) const fn from_kind(kind: WorthQueryInvariantProjectionDenialKind) -> Self {
        Self { kind }
    }

    pub(super) const fn work_budget_exceeded() -> Self {
        Self {
            kind: WorthQueryInvariantProjectionDenialKind::WorkBudgetExceeded,
        }
    }
}

impl std::fmt::Display for WorthQueryInvariantProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invariant projection denied: {:?}", self.kind)
    }
}

impl std::error::Error for WorthQueryInvariantProjectionDenial {}

pub(super) fn from_branch_basis_denial(
    denial: worth_relational::facade::branch::RelationalBranchBasisDenial,
) -> WorthQueryInvariantProjectionDenial {
    match denial {
        worth_relational::facade::branch::RelationalBranchBasisDenial::ForeignRuntime { .. } => {
            WorthQueryInvariantProjectionDenial::from_kind(
                WorthQueryInvariantProjectionDenialKind::ForeignBasis,
            )
        }
        worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
            WorthQueryInvariantProjectionDenial::from_kind(
                WorthQueryInvariantProjectionDenialKind::RetentionCapacityExhausted,
            )
        }
        worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
            WorthQueryInvariantProjectionDenial::from_kind(
                WorthQueryInvariantProjectionDenialKind::RetentionIdentityExhausted,
            )
        }
        worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
            WorthQueryInvariantProjectionDenial::from_kind(
                WorthQueryInvariantProjectionDenialKind::SnapshotIdentityExhausted,
            )
        }
        _ => WorthQueryInvariantProjectionDenial::basis_unavailable(),
    }
}

pub(super) fn from_snapshot_admission_denial(
    denial: worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial,
) -> WorthQueryInvariantProjectionDenial {
    let kind = match denial {
        worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => WorthQueryInvariantProjectionDenialKind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        },
        worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::ForeignRuntime { .. } => {
            WorthQueryInvariantProjectionDenialKind::ForeignBasis
        }
        worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::SnapshotIdentityExhausted => {
            WorthQueryInvariantProjectionDenialKind::SnapshotIdentityExhausted
        }
    };
    WorthQueryInvariantProjectionDenial::from_kind(kind)
}
