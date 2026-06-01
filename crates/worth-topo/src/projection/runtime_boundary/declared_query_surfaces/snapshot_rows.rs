use forge_query::facade::ForgeQueryWorkspace;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use super::{historical_rows, TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceError};
use crate::facade::NamingAttachmentReport;

#[derive(Debug, Clone)]
pub(super) struct TopologyQuerySnapshotRows {
    pub naming_attachments: NamingAttachmentReport,
    pub materialized_rows: Vec<Value>,
    pub interpreted_rows: Vec<Value>,
    pub validation_rows: Vec<Value>,
    pub diagnostics_rows: Vec<Value>,
    pub equivalence_rows: Vec<Value>,
}

impl TopologyQuerySnapshotRows {
    pub(super) fn historical(
        surfaces: &TopologyDeclaredQuerySurfaces,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<Self, TopologyQuerySurfaceError> {
        historical_rows::historical_snapshot_rows(surfaces, workspace, read_basis)
    }
}
