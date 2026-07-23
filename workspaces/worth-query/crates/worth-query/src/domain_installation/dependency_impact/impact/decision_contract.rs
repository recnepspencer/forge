use super::super::compiled::WorthQuerySemanticDependencyRole;
use super::authority::WorthQueryCheckedImpactBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryImpactClass {
    UnaffectedOrSuppressed,
    ValuePatch,
    MembershipSplice,
    ReorderOrRegroup,
    WindowShift,
    Reexecute,
    ExplicitRebind,
    Replacement,
    Retirement,
    UnsupportedEscalation,
}

impl WorthQueryImpactClass {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::UnaffectedOrSuppressed => "unaffected-or-suppressed",
            Self::ValuePatch => "value-patch",
            Self::MembershipSplice => "membership-splice",
            Self::ReorderOrRegroup => "reorder-or-regroup",
            Self::WindowShift => "window-shift",
            Self::Reexecute => "reexecute",
            Self::ExplicitRebind => "explicit-rebind",
            Self::Replacement => "replacement",
            Self::Retirement => "retirement",
            Self::UnsupportedEscalation => "unsupported-escalation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryImpactAdmissionDenialKind {
    DependencyClosureUnavailable,
    ForeignRuntime,
    StaleInstallation,
    ForeignOperation,
    ForeignConditionalOutcome,
    ConditionalAuthorityMismatch,
    ConditionalDeliveryMismatch,
    CausalDeliveryMismatch,
    OwnerDeliveryOutOfOrder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryImpactAdmissionDenial {
    kind: WorthQueryImpactAdmissionDenialKind,
    counters: WorthQueryImpactCounters,
}

#[derive(Debug)]
pub struct WorthQueryImpactDecision {
    pub(super) class: WorthQueryImpactClass,
    pub(super) affected_roles: Vec<WorthQuerySemanticDependencyRole>,
    pub(super) owner_change_count: usize,
    pub(super) affected_dependency_count: usize,
    pub(super) counters: WorthQueryImpactCounters,
    pub(super) checked_basis: WorthQueryCheckedImpactBasis,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryImpactCounters {
    pub runtime_authority_checks: usize,
    pub installation_generation_checks: usize,
    pub operation_affinity_checks: usize,
    pub conditional_location_checks: usize,
    pub conditional_authority_checks: usize,
    pub delivery_identity_checks: usize,
    pub dependency_membership_lookups: usize,
    pub staged_changes_inspected: usize,
    pub causal_keys_materialized: usize,
    pub causal_key_lookups: usize,
    pub owner_order_checks: usize,
    pub owner_changes_inspected: usize,
    pub index_lookups: usize,
    pub affected_edges: usize,
    pub conditional_outcomes_inspected: usize,
    pub unrelated_dependency_scans: usize,
    pub consumer_registry_scans: usize,
}

impl WorthQueryImpactAdmissionDenial {
    pub(crate) const fn new(
        kind: WorthQueryImpactAdmissionDenialKind,
        counters: WorthQueryImpactCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> WorthQueryImpactAdmissionDenialKind {
        self.kind
    }

    pub const fn counters(self) -> WorthQueryImpactCounters {
        self.counters
    }
}

impl WorthQueryImpactDecision {
    pub const fn class(&self) -> WorthQueryImpactClass {
        self.class
    }

    pub fn affected_roles(&self) -> &[WorthQuerySemanticDependencyRole] {
        &self.affected_roles
    }

    pub const fn owner_change_count(&self) -> usize {
        self.owner_change_count
    }

    pub const fn affected_dependency_count(&self) -> usize {
        self.affected_dependency_count
    }

    pub const fn counters(&self) -> WorthQueryImpactCounters {
        self.counters
    }
}
