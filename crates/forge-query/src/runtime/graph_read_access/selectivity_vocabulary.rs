use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBooleanSelectivityShapeDigest(String);

impl ForgeQueryBooleanSelectivityShapeDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryBooleanPredicateTopology {
    None,
    ConjunctiveFlat,
    DisjunctiveBranching,
    BranchingUnsupportedByCurrentAuthoring,
}

impl ForgeQueryBooleanPredicateTopology {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ConjunctiveFlat => "conjunctive_flat",
            Self::DisjunctiveBranching => "disjunctive_branching",
            Self::BranchingUnsupportedByCurrentAuthoring => {
                "branching_unsupported_by_current_authoring"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryBooleanSelectivityBranchKind {
    EmptyRoot,
    ConjunctiveRoot,
    DisjunctiveBranch,
}

impl ForgeQueryBooleanSelectivityBranchKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRoot => "empty_root",
            Self::ConjunctiveRoot => "conjunctive_root",
            Self::DisjunctiveBranch => "disjunctive_branch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryPredicateOperandOperator {
    Equal,
    GreaterThan,
    LessThan,
    Contains,
    In,
    Presence,
}

impl ForgeQueryPredicateOperandOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equal => "eq",
            Self::GreaterThan => "gt",
            Self::LessThan => "lt",
            Self::Contains => "contains",
            Self::In => "in",
            Self::Presence => "presence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryBooleanSelectivityAdmissionPosture {
    InlineEligible,
    RequiresAccessCapabilityRegistration,
}

impl ForgeQueryBooleanSelectivityAdmissionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineEligible => "inline_eligible",
            Self::RequiresAccessCapabilityRegistration => "requires_access_capability_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryPredicateSelectivityClass {
    ExactAnchor,
    TenantAnchor,
    PolicyAnchor,
    SelectivePredicate,
    RangePredicate,
    BroadPredicate,
    UnknownPredicate,
    TraversalPredicate,
    DisjunctionBarrier,
    IntersectionEligible,
    PostTraversalOnly,
}

impl ForgeQueryPredicateSelectivityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactAnchor => "exact_anchor",
            Self::TenantAnchor => "tenant_anchor",
            Self::PolicyAnchor => "policy_anchor",
            Self::SelectivePredicate => "selective_predicate",
            Self::RangePredicate => "range_predicate",
            Self::BroadPredicate => "broad_predicate",
            Self::UnknownPredicate => "unknown_predicate",
            Self::TraversalPredicate => "traversal_predicate",
            Self::DisjunctionBarrier => "disjunction_barrier",
            Self::IntersectionEligible => "intersection_eligible",
            Self::PostTraversalOnly => "post_traversal_only",
        }
    }

    pub(crate) fn is_pre_traversal_eligible(&self) -> bool {
        matches!(
            self,
            Self::ExactAnchor
                | Self::TenantAnchor
                | Self::PolicyAnchor
                | Self::SelectivePredicate
                | Self::RangePredicate
                | Self::IntersectionEligible
        )
    }

    pub(crate) fn is_broad_or_risky(&self) -> bool {
        matches!(
            self,
            Self::BroadPredicate
                | Self::UnknownPredicate
                | Self::TraversalPredicate
                | Self::DisjunctionBarrier
                | Self::PostTraversalOnly
        )
    }

    pub(crate) fn is_exact_anchor(&self) -> bool {
        matches!(
            self,
            Self::ExactAnchor | Self::TenantAnchor | Self::PolicyAnchor
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryPredicateAnchorPosture {
    NoPredicateAnchor,
    AnchoredByExactPredicate,
    AnchoredByMembershipPredicate,
    BroadOnly,
    MixedAnchorAndBroad,
}

impl ForgeQueryPredicateAnchorPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoPredicateAnchor => "no_predicate_anchor",
            Self::AnchoredByExactPredicate => "anchored_by_exact_predicate",
            Self::AnchoredByMembershipPredicate => "anchored_by_membership_predicate",
            Self::BroadOnly => "broad_only",
            Self::MixedAnchorAndBroad => "mixed_anchor_and_broad",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryTraversalPredicateOrderingPosture {
    NoPredicate,
    PreTraversalEligible,
    PostTraversalFilterRequired,
    Mixed,
}

impl ForgeQueryTraversalPredicateOrderingPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoPredicate => "no_predicate",
            Self::PreTraversalEligible => "pre_traversal_eligible",
            Self::PostTraversalFilterRequired => "post_traversal_filter_required",
            Self::Mixed => "mixed",
        }
    }
}
