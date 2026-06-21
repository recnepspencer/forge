use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryWorkspace};

use super::super::admitted_handoff::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use super::super::birth_synopsis::TopologyPrimitiveConstructionBirthFamily;
use super::super::surface_vocab::TopologyConstructionQueryMutationSurface;
use super::coverage::TopologyPrimitiveConstructionBirthMaterializationCoverage;
use super::error::TopologyPrimitiveConstructionBirthComposeExecutionError;
use super::obligation_registration::{
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_LAYOUT_VIOLATION_COLLECTION,
};
use super::touched_basis::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis;
use crate::construction::query_native_boundary::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionBirthComposeProgram {
    family: TopologyPrimitiveConstructionBirthFamily,
    source_admitted_handoff_digest: String,
    birth_marker_collection: String,
    birth_marker_symbol: String,
    birth_entities: Vec<TopologyPrimitiveConstructionBirthEntity>,
    materialization_coverage: TopologyPrimitiveConstructionBirthMaterializationCoverage,
    program_digest: String,
}

impl TopologyPrimitiveConstructionBirthComposeProgram {
    pub(crate) fn new(
        family: TopologyPrimitiveConstructionBirthFamily,
        source_admitted_handoff_digest: impl Into<String>,
        birth_entities: Vec<TopologyPrimitiveConstructionBirthEntity>,
        materialization_coverage: TopologyPrimitiveConstructionBirthMaterializationCoverage,
        routes_to_layout_violation_probe: bool,
    ) -> Self {
        let source_admitted_handoff_digest = source_admitted_handoff_digest.into();
        let birth_marker_collection = if routes_to_layout_violation_probe {
            TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_LAYOUT_VIOLATION_COLLECTION
        } else {
            TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION
        }
        .to_string();
        let birth_marker_symbol = format!("primitive-birth-{}", family.as_str());
        let program_digest = digest_parts(&[
            "primitive-construction-birth-compose-program".to_string(),
            family.as_str().to_string(),
            source_admitted_handoff_digest.clone(),
            birth_marker_collection.clone(),
            birth_marker_symbol.clone(),
            birth_entities
                .iter()
                .map(TopologyPrimitiveConstructionBirthEntity::entity_digest)
                .collect::<Vec<_>>()
                .join("|"),
            materialization_coverage.coverage_digest().to_string(),
        ]);
        Self {
            family,
            source_admitted_handoff_digest,
            birth_marker_collection,
            birth_marker_symbol,
            birth_entities,
            materialization_coverage,
            program_digest,
        }
    }

    pub fn family(&self) -> TopologyPrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        TopologyConstructionQueryMutationSurface::ComposeGraph
    }

    pub fn source_admitted_handoff_digest(&self) -> &str {
        &self.source_admitted_handoff_digest
    }

    pub fn birth_marker_collection(&self) -> &str {
        &self.birth_marker_collection
    }

    pub fn birth_entity_count(&self) -> usize {
        self.birth_entities.len()
    }

    pub fn materialization_coverage(
        &self,
    ) -> &TopologyPrimitiveConstructionBirthMaterializationCoverage {
        &self.materialization_coverage
    }

    pub fn program_digest(&self) -> &str {
        &self.program_digest
    }

    pub(super) fn execute_declared_touched_basis_checked(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        admitted_handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
        declared_touched_basis: &TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyPrimitiveConstructionBirthComposeExecutionError>
    {
        declared_touched_basis.require_matches_handoff(admitted_handoff)?;
        Ok(self.execute_checked_graph_write(workspace)?)
    }

    fn execute_checked_graph_write(
        &self,
        workspace: &mut ForgeQueryWorkspace,
    ) -> Result<ForgeQueryBatchWriteReceipt, forge_query::facade::ForgeQueryRuntimeError> {
        workspace.compose_graph(|graph| {
            for birth_entity in &self.birth_entities {
                let symbol = format!(
                    "{}.{}",
                    self.birth_marker_symbol,
                    birth_entity.structure_suffix()
                );
                let structure = format!(
                    "primitive-construction-birth.{}.{}.{}",
                    self.family.as_str(),
                    self.source_admitted_handoff_digest,
                    birth_entity.structure_suffix(),
                );
                let topology_kind = birth_entity.topology_kind();
                graph.insert_entity(symbol, &self.birth_marker_collection, |entity| {
                    entity
                        .aspect("topology.kind", topology_kind)
                        .aspect("topology.structure", structure.clone())
                        .aspect("naming.persistent_name", structure)
                })?;
            }
            Ok(())
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TopologyPrimitiveConstructionBirthEntity {
    topology_kind: &'static str,
    structure_suffix: String,
    entity_digest: String,
}

impl TopologyPrimitiveConstructionBirthEntity {
    pub(crate) fn new(topology_kind: &'static str, structure_suffix: impl Into<String>) -> Self {
        let structure_suffix = structure_suffix.into();
        let entity_digest = digest_parts(&[
            "primitive-construction-birth-entity".to_string(),
            topology_kind.to_string(),
            structure_suffix.clone(),
        ]);
        Self {
            topology_kind,
            structure_suffix,
            entity_digest,
        }
    }

    fn topology_kind(&self) -> &'static str {
        self.topology_kind
    }

    fn structure_suffix(&self) -> &str {
        &self.structure_suffix
    }

    fn entity_digest(&self) -> &str {
        &self.entity_digest
    }
}
