use worth_spatial::facade::blocker_provenance::{
    PlanarBooleanBlockerProvenanceInput, WorkloadBlockerBoundaryKind, WorkloadBlockerSourceKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBlockerContext {
    provenance: PlanarBooleanBlockerProvenanceInput,
}

impl PlanarBooleanBlockerContext {
    pub fn new(
        source_kind: WorkloadBlockerSourceKind,
        boundary_kind: WorkloadBlockerBoundaryKind,
        source_identity: impl Into<String>,
        boundary_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            provenance: PlanarBooleanBlockerProvenanceInput::new(
                source_kind,
                boundary_kind,
                source_identity,
                boundary_identity,
                human_reason,
            ),
        }
    }

    pub fn provenance(&self) -> &PlanarBooleanBlockerProvenanceInput {
        &self.provenance
    }
}
