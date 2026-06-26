use forge_query::facade::{
    ForgeQueryDerivedView, ForgeQueryMutationMetadata,
    ForgeQueryRuntimeDeclarationInitializationAdapter, ForgeQueryWorkspaceError,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::projection::runtime_boundary::declared_query_surfaces::query_diagnostics::TopologyQueryMutationEvidence;

#[derive(Debug, Clone)]
pub(crate) enum TopologyRuntimeDeclarationInitialization {
    Default,
    HistoricalReadBasis(DerivedTopologyReadBasis),
}

impl TopologyRuntimeDeclarationInitialization {
    pub(crate) fn default_runtime() -> Self {
        Self::Default
    }

    pub(crate) fn historical_read_basis(read_basis: DerivedTopologyReadBasis) -> Self {
        Self::HistoricalReadBasis(read_basis)
    }
}

pub(crate) struct TopologyRuntimeDeclarationInitializationAdapter {
    initialization: TopologyRuntimeDeclarationInitialization,
}

impl TopologyRuntimeDeclarationInitializationAdapter {
    pub(crate) fn new(initialization: TopologyRuntimeDeclarationInitialization) -> Self {
        Self { initialization }
    }
}

impl ForgeQueryRuntimeDeclarationInitializationAdapter
    for TopologyRuntimeDeclarationInitializationAdapter
{
    fn declaration_initialization_metadata(
        &self,
        _view: &ForgeQueryDerivedView,
    ) -> Result<ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        match &self.initialization {
            TopologyRuntimeDeclarationInitialization::Default => {
                Ok(ForgeQueryMutationMetadata::default())
            }
            TopologyRuntimeDeclarationInitialization::HistoricalReadBasis(read_basis) => {
                let mut metadata = ForgeQueryMutationMetadata::default();
                metadata.insert(
                    TopologyQueryMutationEvidence::metadata_key().to_string(),
                    serde_json::to_string(&TopologyQueryMutationEvidence::from_read_basis(read_basis))
                        .map_err(|error| {
                            ForgeQueryWorkspaceError::new(format!(
                                "topology declaration initialization failed to encode historical read-basis evidence: {error}"
                            ))
                        })?,
                )?;
                Ok(metadata)
            }
        }
    }
}
