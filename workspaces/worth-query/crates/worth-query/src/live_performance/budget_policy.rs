#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PatchWidthUnit {
    ProjectedFieldDelta,
    CollectionRowChange,
    MaterializedNodeChange,
}

impl PatchWidthUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedFieldDelta => "projected_field_delta",
            Self::CollectionRowChange => "collection_row_change",
            Self::MaterializedNodeChange => "materialized_node_change",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PatchWidthBudget {
    unit: PatchWidthUnit,
    limit: usize,
}

impl PatchWidthBudget {
    pub fn unit(&self) -> &PatchWidthUnit {
        &self.unit
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn new(unit: PatchWidthUnit, limit: usize) -> Self {
        Self { unit, limit }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PatchWidthPolicy {
    DeliverWithinBudget,
    CoalesceWithinAdmittedClass,
    RefreshWithinAdmissionMatrix,
    RejectOverflow,
}

impl PatchWidthPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeliverWithinBudget => "deliver_within_budget",
            Self::CoalesceWithinAdmittedClass => "coalesce_within_admitted_class",
            Self::RefreshWithinAdmissionMatrix => "refresh_within_admission_matrix",
            Self::RejectOverflow => "reject_overflow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoalescingAdmissionClass {
    Forbidden,
    BasisStableEquivalent,
}

impl CoalescingAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forbidden => "forbidden",
            Self::BasisStableEquivalent => "basis_stable_equivalent",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshCostClass {
    NarrowRefresh,
    BroadRefresh,
}

impl RefreshCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NarrowRefresh => "narrow_refresh",
            Self::BroadRefresh => "broad_refresh",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshAdmissionStatus {
    Verified,
    Debt,
    Forbidden,
}

impl RefreshAdmissionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
            Self::Forbidden => "forbidden",
        }
    }
}
