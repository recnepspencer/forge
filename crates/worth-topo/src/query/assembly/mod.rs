//! Canonical Forge Query-backed Worth topology assembly.
//!
//! This module is the Worth-owned workspace seam for:
//! - declaring the full topology live/computed surface family once
//! - applying topology intent through canonical Query writes/assertions
//! - carrying persistent-name truth alongside topology truth
//! - decoding current-head and snapshot-bound truth into Worth domain outputs

use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use serde_json::Value;
use worth_schema::facade::DerivedTopologyReadBasis;

use super::{
    declare_worth_persistent_name_live_view, declare_worth_topology_diagnostics_surface,
    declare_worth_topology_entity_live_view, declare_worth_topology_equivalence_contract_surface,
    declare_worth_topology_interpreted_surface, declare_worth_topology_materialized_surface,
    declare_worth_topology_relation_live_view, declare_worth_topology_validation_surface,
    equivalence_contract_from_diagnostics_rows, interpreted_topology_from_materialized_rows,
    naming_attachment_report_from_query_rows, validation_report_from_query_rows,
    WorthTopologyQuerySurfaceError,
};
use crate::diagnostics::build_derived_read_diagnostics;
use crate::facade::{
    DerivedTopologyValidationReport, InterpretedTopologyView, MaterializedTopologyView,
    WorthDerivedEquivalenceContractReport, WorthDerivedReadDiagnostics,
    WorthNamingAttachmentReport,
};
use crate::interpretation::interpret_topology_view;
use crate::validators::validate_interpreted_topology;

mod authority;
mod authority_support;
pub use self::authority::{WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError};

const ENTITY_SURFACE: &str = "worth.topology.entities";
const RELATION_SURFACE: &str = "worth.topology.relations";
const PERSISTENT_NAME_SURFACE: &str = "worth.naming.persistent_names";
const MATERIALIZED_SURFACE: &str = "worth.topology.materialized";
const INTERPRETED_SURFACE: &str = "worth.topology.interpreted";
const VALIDATION_SURFACE: &str = "worth.topology.validation";
const DIAGNOSTICS_SURFACE: &str = "worth.topology.diagnostics";
const EQUIVALENCE_SURFACE: &str = "worth.topology.equivalence_contract";

#[derive(Debug, Clone)]
pub struct WorthTopologyQueryAssembly {
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
pub struct WorthTopologyQuerySnapshot {
    pub naming_attachments: WorthNamingAttachmentReport,
    pub materialized: MaterializedTopologyView,
    pub interpreted: InterpretedTopologyView,
    pub validation: DerivedTopologyValidationReport,
    pub diagnostics: WorthDerivedReadDiagnostics,
    pub equivalence_contract: WorthDerivedEquivalenceContractReport,
}

impl WorthTopologyQueryAssembly {
    pub fn declare(workspace: &mut ForgeQueryWorkspace) -> Result<Self, ForgeQueryRuntimeError> {
        let entities = declare_worth_topology_entity_live_view(workspace, ENTITY_SURFACE)?;
        let relations = declare_worth_topology_relation_live_view(workspace, RELATION_SURFACE)?;
        let persistent_names =
            declare_worth_persistent_name_live_view(workspace, PERSISTENT_NAME_SURFACE)?;
        let materialized = declare_worth_topology_materialized_surface(
            workspace,
            MATERIALIZED_SURFACE,
            &entities,
            &relations,
        )?;
        let interpreted = declare_worth_topology_interpreted_surface(
            workspace,
            INTERPRETED_SURFACE,
            &materialized,
        )?;
        let validation = declare_worth_topology_validation_surface(
            workspace,
            VALIDATION_SURFACE,
            &materialized,
            &interpreted,
        )?;
        let diagnostics = declare_worth_topology_diagnostics_surface(
            workspace,
            DIAGNOSTICS_SURFACE,
            &materialized,
            &interpreted,
            &validation,
        )?;
        let equivalence_contract = declare_worth_topology_equivalence_contract_surface(
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

    pub fn snapshot(
        &self,
        workspace: &mut ForgeQueryWorkspace,
    ) -> Result<WorthTopologyQuerySnapshot, WorthTopologyQuerySurfaceError> {
        workspace
            .state(&self.materialized)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.interpreted)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.validation)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.diagnostics)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        workspace
            .state(&self.equivalence_contract)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        let entity_rows = workspace.read(&self.entities);
        let persistent_name_rows = workspace.read(&self.persistent_names);
        let naming_attachments =
            naming_attachment_report_from_query_rows(&entity_rows, &persistent_name_rows)?;
        let materialized_rows = workspace.materialize(&self.materialized);
        let materialized: MaterializedTopologyView =
            serde_json::from_value(materialized_rows[0].clone()).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived `materialized topology` row failed to decode: {error}"
                ))
            })?;
        let interpreted_rows = workspace.materialize(&self.interpreted);
        let _validation_rows = workspace.materialize(&self.validation);
        let diagnostics_rows = workspace.materialize(&self.diagnostics);
        let equivalence_rows = workspace.materialize(&self.equivalence_contract);
        let interpreted = interpreted_topology_from_materialized_rows(&materialized_rows)?;
        let validation = validation_report_from_query_rows(&materialized_rows, &interpreted_rows)?;
        let diagnostics: WorthDerivedReadDiagnostics =
            serde_json::from_value(diagnostics_rows[0].clone()).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived `derived read diagnostics` row failed to decode: {error}"
                ))
            })?;
        let equivalence_contract = equivalence_contract_from_diagnostics_rows(&diagnostics_rows)?;
        let decoded_equivalence: WorthDerivedEquivalenceContractReport =
            serde_json::from_value(equivalence_rows[0].clone()).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived `derived equivalence contract` row failed to decode: {error}"
                ))
            })?;
        if equivalence_contract != decoded_equivalence {
            return Err(WorthTopologyQuerySurfaceError::new(
                "query-derived equivalence contract row and diagnostics-carried equivalence contract diverged",
            ));
        }
        Ok(WorthTopologyQuerySnapshot {
            naming_attachments,
            materialized,
            interpreted,
            validation,
            diagnostics,
            equivalence_contract,
        })
    }

    pub fn snapshot_for_read_basis(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTopologyQuerySnapshot, WorthTopologyQuerySurfaceError> {
        let entity_rows = workspace.read(&self.entities);
        let relation_rows = workspace.read(&self.relations);
        let persistent_name_rows = workspace.read(&self.persistent_names);
        let naming_attachments =
            naming_attachment_report_from_query_rows(&entity_rows, &persistent_name_rows)?;
        let materialized = super::materialized::materialized_topology_from_query_rows(
            &entity_rows,
            &relation_rows,
        )
        .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        let interpreted = interpret_topology_view(&materialized);
        let validation =
            validate_interpreted_topology(&materialized, &interpreted).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived validation refresh failed: {error}"
                ))
            })?;
        let diagnostics =
            build_derived_read_diagnostics(read_basis, &materialized, &interpreted, &validation);
        let equivalence_contract = diagnostics.equivalence_contract_report.clone();

        Ok(WorthTopologyQuerySnapshot {
            naming_attachments,
            materialized,
            interpreted,
            validation,
            diagnostics,
            equivalence_contract,
        })
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod upsert_assertion_tests;
