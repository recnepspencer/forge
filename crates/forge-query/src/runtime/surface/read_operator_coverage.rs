#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadOperatorFamily {
    Projection,
    Traversal,
    Predicate,
    Ordering,
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
