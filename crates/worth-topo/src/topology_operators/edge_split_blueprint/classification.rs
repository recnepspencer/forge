#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeSplitOperatorClassification {
    PreparedSpatialOnly,
    TopologyDeclarationFamily,
    TopologyGroupedDeclarationFamily,
    TopologyContributionWorkflow,
    QueryGraphCompositionProgram,
    SupportGatedFutureTopologyMutation,
}

impl EdgeSplitOperatorClassification {
    pub fn may_commit_topology_in_7_3(self) -> bool {
        matches!(
            self,
            Self::TopologyDeclarationFamily
                | Self::TopologyGroupedDeclarationFamily
                | Self::TopologyContributionWorkflow
                | Self::QueryGraphCompositionProgram
        )
    }

    pub fn requires_query_surface(self) -> bool {
        !matches!(self, Self::PreparedSpatialOnly)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeSplitOperatorTruthAuthority {
    WorthSpatialPrepared,
    WorthTopoQueryDeclaration,
    ForgeQueryGraphComposition,
    FutureSupportGated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeSplitRequiredQuerySurface {
    None,
    TopologyDeclarationEntry,
    TopologyGroupedDeclaration,
    TopologyContributionWorkflow,
    QueryGraphComposition,
    QueryInvariantRegistration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeSplitValidatorRuntimeLane {
    SpatialPreparedProductValidation,
    TopologyDeclarationReview,
    QueryGraphInvariantPack,
    SupportGatedFutureRuntime,
}

impl EdgeSplitValidatorRuntimeLane {
    pub fn is_runtime_facing(self) -> bool {
        matches!(
            self,
            Self::TopologyDeclarationReview | Self::QueryGraphInvariantPack
        )
    }
}
