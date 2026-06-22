#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopOperatorClassification {
    PreparedSpatialOnly,
    TopologyDeclarationFamily,
    TopologyGroupedDeclarationFamily,
    TopologyContributionWorkflow,
    QueryGraphCompositionProgram,
    SupportGatedFutureTopologyMutation,
}

impl PlanarBooleanLoopOperatorClassification {
    pub fn may_commit_topology_in_7_4(self) -> bool {
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
pub enum PlanarBooleanLoopOperatorTruthAuthority {
    WorthSpatialPrepared,
    WorthTopoQueryDeclaration,
    ForgeQueryGraphComposition,
    FutureSupportGated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopRequiredQuerySurface {
    None,
    TopologyDeclarationEntry,
    TopologyGroupedDeclaration,
    TopologyContributionWorkflow,
    QueryGraphComposition,
    QueryInvariantRegistration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopValidatorRuntimeLane {
    SpatialPreparedProductValidation,
    TopologyDeclarationReview,
    QueryGraphInvariantPack,
    SupportGatedFutureRuntime,
}

impl PlanarBooleanLoopValidatorRuntimeLane {
    pub fn is_runtime_facing(self) -> bool {
        matches!(
            self,
            Self::TopologyDeclarationReview | Self::QueryGraphInvariantPack
        )
    }
}
