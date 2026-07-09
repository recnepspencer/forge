use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessShapeDigest(String);

impl WorthQueryGraphReadAccessShapeDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadRootPosture {
    Local,
    Anchored,
    ExplicitBroadSearch,
}

impl WorthQueryGraphReadRootPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Anchored => "anchored",
            Self::ExplicitBroadSearch => "explicit_broad_search",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadTraversalOperator {
    DirectEdge,
    SuccessorWalk,
    BoundedAncestor,
    BoundedDescendant,
    AnchoredFrontier,
    SharedEndpoint,
    SharedAttachment,
    FrontierSearch,
    DeclarationTraversal,
}

impl WorthQueryGraphReadTraversalOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectEdge => "direct_edge",
            Self::SuccessorWalk => "successor_walk",
            Self::BoundedAncestor => "bounded_ancestor",
            Self::BoundedDescendant => "bounded_descendant",
            Self::AnchoredFrontier => "anchored_frontier",
            Self::SharedEndpoint => "shared_endpoint",
            Self::SharedAttachment => "shared_attachment",
            Self::FrontierSearch => "frontier_search",
            Self::DeclarationTraversal => "declaration_traversal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadFanoutPosture {
    None,
    SingleRelation,
    MultiRelation,
    Frontier,
}

impl WorthQueryGraphReadFanoutPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SingleRelation => "single_relation",
            Self::MultiRelation => "multi_relation",
            Self::Frontier => "frontier",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadPredicateFamily {
    None,
    Equality,
    Range,
    Text,
    Membership,
    Presence,
    Mixed,
}

impl WorthQueryGraphReadPredicateFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Equality => "equality",
            Self::Range => "range",
            Self::Text => "text",
            Self::Membership => "membership",
            Self::Presence => "presence",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadOrderingPosture {
    Unordered,
    Ordered,
}

impl WorthQueryGraphReadOrderingPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unordered => "unordered",
            Self::Ordered => "ordered",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadResultPressure {
    Detail,
    CollectionNarrow,
    CollectionWide,
}

impl WorthQueryGraphReadResultPressure {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::CollectionNarrow => "collection_narrow",
            Self::CollectionWide => "collection_wide",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadLifecycleClass {
    ReusableReadFamily,
}

impl WorthQueryGraphReadLifecycleClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReusableReadFamily => "reusable_read_family",
        }
    }
}
