//! Internal declared Query surfaces for topology historical snapshot decoding.

use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use crate::facade::{
    DerivedEquivalenceContractReport, DerivedReadDiagnostics, DerivedTopologyValidationReport,
    InterpretedTopologyView, MaterializedTopologyView, NamingAttachmentReport,
};
use crate::projection::{
    declare_persistent_name_live_view, declare_topology_diagnostics_surface,
    declare_topology_entity_live_view, declare_topology_equivalence_contract_surface,
    declare_topology_interpreted_surface, declare_topology_materialized_surface,
    declare_topology_relation_live_view, declare_topology_validation_surface,
    TopologyQuerySurfaceError,
};
mod historical_rows;
mod snapshot_decode;
mod snapshot_rows;

const ENTITY_SURFACE: &str = ".topology.entities";
const RELATION_SURFACE: &str = ".topology.relations";
const PERSISTENT_NAME_SURFACE: &str = ".naming.persistent_names";
const MATERIALIZED_SURFACE: &str = ".topology.materialized";
const INTERPRETED_SURFACE: &str = ".topology.interpreted";
const VALIDATION_SURFACE: &str = ".topology.validation";
const DIAGNOSTICS_SURFACE: &str = ".topology.diagnostics";
const EQUIVALENCE_SURFACE: &str = ".topology.equivalence_contract";

#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredQuerySurfaces {
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
pub(crate) struct TopologyDerivedQuerySnapshot {
    pub naming_attachments: NamingAttachmentReport,
    pub materialized: MaterializedTopologyView,
    pub interpreted: InterpretedTopologyView,
    pub validation: DerivedTopologyValidationReport,
    pub diagnostics: DerivedReadDiagnostics,
    pub equivalence_contract: DerivedEquivalenceContractReport,
}

pub(crate) fn declare_topology_query_surfaces(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<TopologyDeclaredQuerySurfaces, ForgeQueryRuntimeError> {
    let entities = declare_topology_entity_live_view(workspace, ENTITY_SURFACE)?;
    let relations = declare_topology_relation_live_view(workspace, RELATION_SURFACE)?;
    let persistent_names = declare_persistent_name_live_view(workspace, PERSISTENT_NAME_SURFACE)?;
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
    Ok(TopologyDeclaredQuerySurfaces {
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

impl TopologyDeclaredQuerySurfaces {
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

    pub fn snapshot_for_read_basis(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<TopologyDerivedQuerySnapshot, TopologyQuerySurfaceError> {
        let rows =
            snapshot_rows::TopologyQuerySnapshotRows::historical(self, workspace, read_basis)?;
        let snapshot = snapshot_decode::snapshot_from_query_rows(rows)?;
        ensure_snapshot_matches_read_basis(&snapshot, read_basis)?;
        Ok(snapshot)
    }
}

fn ensure_snapshot_matches_read_basis(
    snapshot: &TopologyDerivedQuerySnapshot,
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
            .mutation_digest_hex
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
mod boundary_tests;
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tests;
