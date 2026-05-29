use forge_query::facade::ForgeQueryWorkspace;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use super::{historical_rows, TopologyQueryAssembly, TopologyQuerySurfaceError};
use crate::facade::NamingAttachmentReport;
use crate::projection::{naming_attachment_report_from_query_input, TopologyNamingAttachmentInput};

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
    pub(super) fn current_head(
        assembly: &TopologyQueryAssembly,
        workspace: &mut ForgeQueryWorkspace,
    ) -> Result<Self, TopologyQuerySurfaceError> {
        let entity_rows = workspace.read(assembly.entities());
        let persistent_name_rows = workspace.read(assembly.persistent_names());
        let naming_attachments = naming_attachment_report_from_query_input(
            TopologyNamingAttachmentInput::new(&entity_rows, &persistent_name_rows),
        )?;
        Ok(Self {
            naming_attachments,
            materialized_rows: workspace.materialize(assembly.materialized()),
            interpreted_rows: workspace.materialize(assembly.interpreted()),
            validation_rows: workspace.materialize(assembly.validation()),
            diagnostics_rows: workspace.materialize(assembly.diagnostics()),
            equivalence_rows: workspace.materialize(assembly.equivalence_contract()),
        })
    }

    pub(super) fn historical(
        assembly: &TopologyQueryAssembly,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<Self, TopologyQuerySurfaceError> {
        historical_rows::historical_snapshot_rows(assembly, workspace, read_basis)
    }
}
