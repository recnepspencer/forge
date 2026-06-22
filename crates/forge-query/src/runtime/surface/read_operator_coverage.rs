#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadOperatorFamily {
    Projection,
    Traversal,
    Predicate,
    Ordering,
}

impl ForgeQueryReadOperatorFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Projection => "projection",
            Self::Traversal => "traversal",
            Self::Predicate => "predicate",
            Self::Ordering => "ordering",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadBuiltInOperator {
    DirectEdge,
    SuccessorWalk,
    BoundedAncestor,
    BoundedDescendant,
    AnchoredFrontier,
    SharedEndpoint,
    SharedAttachment,
    FrontierSearch,
}

impl ForgeQueryReadBuiltInOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectEdge => "direct-edge",
            Self::SuccessorWalk => "successor-walk",
            Self::BoundedAncestor => "bounded-ancestor",
            Self::BoundedDescendant => "bounded-descendant",
            Self::AnchoredFrontier => "anchored-frontier",
            Self::SharedEndpoint => "shared-endpoint",
            Self::SharedAttachment => "shared-attachment",
            Self::FrontierSearch => "frontier-search",
        }
    }
}
