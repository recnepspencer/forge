//! Canonical Forge Query-backed  topology assembly.
//!
//! This module is the topology-owned workspace seam for:
//! - declaring the full topology live/computed surface family once
//! - applying topology intent through canonical Query writes/assertions
//! - carrying persistent-name truth alongside topology truth
//! - decoding current-head and snapshot-bound truth into  domain outputs

use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::DerivedTopologyReadBasis;
use serde_json::Value;

use super::{
    declare_persistent_name_live_view, declare_topology_diagnostics_surface,
    declare_topology_entity_live_view, declare_topology_equivalence_contract_surface,
    declare_topology_interpreted_surface, declare_topology_materialized_surface,
    declare_topology_relation_live_view, declare_topology_validation_surface,
    naming_attachment_report_from_query_rows, TopologyQuerySurfaceError,
};
use crate::edit::{
    TopologyEditApplicationMode, TopologyEditBatch, TopologyQueryEditExecution,
    TopologyQueryEditExecutionError,
};
use crate::facade::{
    DerivedEquivalenceContractReport, DerivedReadDiagnostics, DerivedTopologyValidationReport,
    InterpretedTopologyView, MaterializedTopologyView, NamingAttachmentReport,
};

mod authority;
mod authority_support;
mod historical_rows;
mod snapshot_decode;
pub use self::authority::{TopologyQueryAppliedIntent, TopologyQueryApplyError};

const ENTITY_SURFACE: &str = ".topology.entities";
const RELATION_SURFACE: &str = ".topology.relations";
const PERSISTENT_NAME_SURFACE: &str = ".naming.persistent_names";
const MATERIALIZED_SURFACE: &str = ".topology.materialized";
const INTERPRETED_SURFACE: &str = ".topology.interpreted";
const VALIDATION_SURFACE: &str = ".topology.validation";
const DIAGNOSTICS_SURFACE: &str = ".topology.diagnostics";
const EQUIVALENCE_SURFACE: &str = ".topology.equivalence_contract";

#[derive(Debug, Clone)]
pub struct TopologyQueryAssembly {
    entities: ForgeQueryLiveView<Value>,
    relations: ForgeQueryLiveView<Value>,
    persistent_names: ForgeQueryLiveView<Value>,
    materialized: ForgeQueryDerivedViewHandle<Value>,
    interpreted: ForgeQueryDerivedViewHandle<Value>,
    validation: ForgeQueryDerivedViewHandle<Value>,
    diagnostics: ForgeQueryDerivedViewHandle<Value>,
    equivalence_contract: ForgeQueryDerivedViewHandle<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopologyQuerySnapshot {
    pub naming_attachments: NamingAttachmentReport,
    pub materialized: MaterializedTopologyView,
    pub interpreted: InterpretedTopologyView,
    pub validation: DerivedTopologyValidationReport,
    pub diagnostics: DerivedReadDiagnostics,
    pub equivalence_contract: DerivedEquivalenceContractReport,
}

impl TopologyQueryAssembly {
    pub fn declare(workspace: &mut ForgeQueryWorkspace) -> Result<Self, ForgeQueryRuntimeError> {
        let entities = declare_topology_entity_live_view(workspace, ENTITY_SURFACE)?;
        let relations = declare_topology_relation_live_view(workspace, RELATION_SURFACE)?;
        let persistent_names =
            declare_persistent_name_live_view(workspace, PERSISTENT_NAME_SURFACE)?;
        let materialized = declare_topology_materialized_surface(
            workspace,
            MATERIALIZED_SURFACE,
            &entities,
            &relations,
        )?;
        let interpreted =
            declare_topology_interpreted_surface(workspace, INTERPRETED_SURFACE, &materialized)?;
        let validation = declare_topology_validation_surface(
            workspace,
            VALIDATION_SURFACE,
            &materialized,
            &interpreted,
        )?;
        let diagnostics = declare_topology_diagnostics_surface(
            workspace,
            DIAGNOSTICS_SURFACE,
            &materialized,
            &interpreted,
            &validation,
        )?;
        let equivalence_contract = declare_topology_equivalence_contract_surface(
            workspace,
            EQUIVALENCE_SURFACE,
            &diagnostics,
        )?;
        Ok(Self {
            entities,
            relations,
            persistent_names,
            materialized,
            interpreted,
            validation,
            diagnostics,
            equivalence_contract,
        })
    }

    pub fn entities(&self) -> &ForgeQueryLiveView<Value> {
        &self.entities
    }

    pub fn relations(&self) -> &ForgeQueryLiveView<Value> {
        &self.relations
    }

    pub fn persistent_names(&self) -> &ForgeQueryLiveView<Value> {
        &self.persistent_names
    }

    pub fn materialized(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.materialized
    }

    pub fn interpreted(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.interpreted
    }

    pub fn validation(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.validation
    }

    pub fn diagnostics(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.diagnostics
    }

    pub fn equivalence_contract(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.equivalence_contract
    }

    pub fn apply_edit(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        batch: TopologyEditBatch,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyQueryEditExecution, TopologyQueryEditExecutionError> {
        crate::edit::TopologyQueryEditRunner::new(workspace, self).apply(batch, mode)
    }

    pub fn snapshot(
        &self,
        workspace: &mut ForgeQueryWorkspace,
    ) -> Result<TopologyQuerySnapshot, TopologyQuerySurfaceError> {
        workspace
            .state(&self.materialized)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.interpreted)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.validation)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.diagnostics)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.equivalence_contract)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let entity_rows = workspace.read(&self.entities);
        let persistent_name_rows = workspace.read(&self.persistent_names);
        let naming_attachments =
            naming_attachment_report_from_query_rows(&entity_rows, &persistent_name_rows)?;
        let materialized_rows = workspace.materialize(&self.materialized);
        let interpreted_rows = workspace.materialize(&self.interpreted);
        let validation_rows = workspace.materialize(&self.validation);
        let diagnostics_rows = workspace.materialize(&self.diagnostics);
        let equivalence_rows = workspace.materialize(&self.equivalence_contract);
        snapshot_decode::snapshot_from_query_rows(
            naming_attachments,
            &materialized_rows,
            &interpreted_rows,
            &validation_rows,
            &diagnostics_rows,
            &equivalence_rows,
        )
    }

    pub fn snapshot_for_read_basis(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<TopologyQuerySnapshot, TopologyQuerySurfaceError> {
        let rows = historical_rows::historical_derived_rows(self, workspace, read_basis)?;
        let snapshot = snapshot_decode::snapshot_from_query_rows(
            rows.naming_attachments,
            &rows.materialized_rows,
            &rows.interpreted_rows,
            &rows.validation_rows,
            &rows.diagnostics_rows,
            &rows.equivalence_rows,
        )?;
        ensure_snapshot_matches_read_basis(&snapshot, read_basis)?;
        Ok(snapshot)
    }
}

fn ensure_snapshot_matches_read_basis(
    snapshot: &TopologyQuerySnapshot,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(), TopologyQuerySurfaceError> {
    let equivalence = &snapshot.equivalence_contract;
    if equivalence.authority_snapshot_id != read_basis.snapshot().snapshot_id.0 {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority snapshot id `{}` did not match requested read basis snapshot `{}`",
            equivalence.authority_snapshot_id,
            read_basis.snapshot().snapshot_id.0
        )));
    }
    if equivalence.authority_branch_id != read_basis.branch_id().0 {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority branch id `{}` did not match requested read basis branch `{}`",
            equivalence.authority_branch_id,
            read_basis.branch_id().0
        )));
    }
    if equivalence.authoritative_mutation_origin != read_basis.authoritative_mutation_origin() {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot authoritative mutation origin diverged from requested read basis",
        ));
    }
    if equivalence.derivation_origin != read_basis.derivation_origin() {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot derivation origin diverged from requested read basis",
        ));
    }
    if equivalence.truth_basis_digest_hex
        != read_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
    {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot truth basis digest diverged from requested read basis",
        ));
    }
    if equivalence.touched_aspect_count != read_basis.touched_aspects().len() {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot touched-aspect count `{}` did not match requested read basis count `{}`",
            equivalence.touched_aspect_count,
            read_basis.touched_aspects().len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod upsert_assertion_tests;
