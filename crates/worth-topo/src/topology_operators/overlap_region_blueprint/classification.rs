#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapOperatorClassification {
    PreparedSpatialOnly,
    TopologyDeclarationFamily,
    TopologyGroupedDeclarationFamily,
    TopologyContributionWorkflow,
    QueryGraphCompositionProgram,
}

impl PlanarBooleanOverlapOperatorClassification {
    pub fn requires_query_surface(self) -> bool {
        !matches!(self, Self::PreparedSpatialOnly)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapOperatorTruthAuthority {
    WorthSpatialPrepared,
    WorthTopoQueryDeclaration,
    ForgeQueryGraphComposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRequiredQuerySurface {
    None,
    TopologyDeclarationEntry,
    TopologyGroupedDeclaration,
    TopologyContributionWorkflow,
    QueryGraphComposition,
    QueryInvariantRegistration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapValidatorRuntimeLane {
    SpatialPreparedProductValidation,
    TopologyDeclarationReview,
    QueryGraphInvariantPack,
}

impl PlanarBooleanOverlapValidatorRuntimeLane {
    pub fn is_runtime_facing(self) -> bool {
        matches!(
            self,
            Self::TopologyDeclarationReview | Self::QueryGraphInvariantPack
        )
    }
}
