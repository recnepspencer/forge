use forge_query::facade::{
    ForgeQueryDerivedView, ForgeQueryMutationMetadata,
    ForgeQueryRuntimeDeclarationInitializationAdapter, ForgeQueryWorkspaceError,
};
use forge_relational::facade::bridge::bridge_snapshot_identity_for_handle;
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
        snapshot_token: &str,
    ) -> Result<ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        match &self.initialization {
            TopologyRuntimeDeclarationInitialization::Default => {
                Ok(ForgeQueryMutationMetadata::default())
            }
            TopologyRuntimeDeclarationInitialization::HistoricalReadBasis(read_basis) => {
                let expected = bridge_snapshot_identity_for_handle(read_basis.snapshot());
                if expected.as_str() != snapshot_token {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "topology declaration initialization received snapshot token `{snapshot_token}` but historical read basis requires `{}`",
                        expected.as_str()
                    )));
                }
                let mut metadata = ForgeQueryMutationMetadata::default();
                metadata.insert(
                    TopologyQueryMutationEvidence::metadata_key().to_string(),
                    serde_json::to_value(TopologyQueryMutationEvidence::from_read_basis(read_basis))
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
