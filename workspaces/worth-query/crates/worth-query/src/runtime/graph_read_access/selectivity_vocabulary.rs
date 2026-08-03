use worth_foundational::facade::CanonicalDigestId;

use crate::identity::canonical_hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBooleanSelectivityShapeDigest {
    canonical: CanonicalDigestId,
    rendered: String,
}

impl WorthQueryBooleanSelectivityShapeDigest {
    pub fn as_str(&self) -> &str {
        &self.rendered
    }

    pub(crate) fn canonical_digest(&self) -> &CanonicalDigestId {
        &self.canonical
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        let canonical = canonical_hash_parts(parts);
        let rendered = canonical.render_hex();
        Self {
            canonical,
            rendered,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryBooleanPredicateTopology {
    None,
    ConjunctiveFlat,
    DisjunctiveBranching,
    BranchingUnsupportedByCurrentAuthoring,
}

impl WorthQueryBooleanPredicateTopology {
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
pub enum WorthQueryBooleanSelectivityBranchKind {
    EmptyRoot,
    ConjunctiveRoot,
    DisjunctiveBranch,
}

impl WorthQueryBooleanSelectivityBranchKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRoot => "empty_root",
            Self::ConjunctiveRoot => "conjunctive_root",
            Self::DisjunctiveBranch => "disjunctive_branch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPredicateOperandOperator {
    Equal,
    GreaterThan,
    LessThan,
    Contains,
    In,
    Presence,
}

impl WorthQueryPredicateOperandOperator {
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
pub enum WorthQueryBooleanSelectivityAdmissionPosture {
    InlineEligible,
    RequiresAccessCapabilityRegistration,
}

impl WorthQueryBooleanSelectivityAdmissionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineEligible => "inline_eligible",
            Self::RequiresAccessCapabilityRegistration => "requires_access_capability_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPredicateSelectivityClass {
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

impl WorthQueryPredicateSelectivityClass {
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
pub enum WorthQueryPredicateAnchorPosture {
    NoPredicateAnchor,
    AnchoredByExactPredicate,
    AnchoredByMembershipPredicate,
    BroadOnly,
    MixedAnchorAndBroad,
}

impl WorthQueryPredicateAnchorPosture {
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
pub enum WorthQueryTraversalPredicateOrderingPosture {
    NoPredicate,
    PreTraversalEligible,
    PostTraversalFilterRequired,
    Mixed,
}

impl WorthQueryTraversalPredicateOrderingPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoPredicate => "no_predicate",
            Self::PreTraversalEligible => "pre_traversal_eligible",
            Self::PostTraversalFilterRequired => "post_traversal_filter_required",
            Self::Mixed => "mixed",
        }
    }
}
