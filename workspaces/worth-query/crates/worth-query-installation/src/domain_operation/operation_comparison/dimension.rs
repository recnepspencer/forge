use super::super::WorthQueryOperationConditionalDimension;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableOperationDimension {
    IdentityName,
    IdentityVersion,
    NativeContract,
    NativeProjectionMask,
    NativeExport,
    Parameters,
    CanonicalQuery,
    Collection,
    RequiredCapabilities,
    RequiredDomains,
    Conditional(WorthQueryOperationConditionalDimension),
    Workflow,
    GraphReads,
    Touches,
    Effects,
    Invariants,
    Replay,
    Reversal,
    Lineage,
    Promotion,
    Publication,
    ProjectionConsumption,
    TerminalResultStates,
    TerminalFailureClasses,
    Cost(WorthQueryPortableOperationCostDimension),
    Support(WorthQueryPortableOperationSupportDimension),
    LoweringFamily,
    LoweringDeterminism,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableOperationCostDimension {
    Lookup,
    Execution,
    ResultWidth,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPortableOperationSupportDimension {
    Live,
    Continuation,
    AsyncResultState,
    Recovery,
    Inspection,
    ProjectionConsumption,
    DependencyImpact,
    Sharing,
    Invalidation,
    CollectionDelivery,
    ConditionalEvaluation,
    ConditionalComparator,
    ConditionalTrigger,
    ConditionalTemporalOrOnDemand,
}
