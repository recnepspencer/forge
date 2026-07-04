use std::sync::OnceLock;

use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use crate::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use crate::derived_invalidation_family_catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
};
use crate::facade::{
    topology_runtime, DerivedInvalidationDensityPolicy, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure, TopologyDeclaredTouchedGraphBasisProof,
    TopologyRewireLoopSuccessorProgramDeclaration, TopologyRuntimeAdapters,
    TopologyTouchedOperatingWorld,
};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    TopologyCurrentHeadReadHandleExt, TopologyReadAnchorIdentity,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validator_invariant_catalog::current_topology_validator_invariant_selection_closeout_for_declared_touch;

use super::current_declaration_support::{
    first_source_identity_for_relation_kind, successor_candidate_with_retained_predecessor,
    successor_relocation_declaration,
};

#[derive(Clone, Debug)]
pub(crate) struct CurrentTopologyInvalidationProof {
    declaration: TopologyRewireLoopSuccessorProgramDeclaration,
    touched_closure: DerivedInvalidationTouchedClosure,
    selected_plan: DerivedInvalidationSelectedPlan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CurrentTopologyInvalidationProofError {
    detail: String,
}

pub(crate) fn current_topology_invalidation_proof(
) -> Result<CurrentTopologyInvalidationProof, CurrentTopologyInvalidationProofError> {
    static CACHE: OnceLock<CurrentTopologyInvalidationProof> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let declaration = current_topology_invalidation_declaration()?;
    let proof = current_topology_invalidation_declared_touch_proof()?;
    let touched_closure = DerivedInvalidationTouchedClosure::from_declared_touch(&proof);
    let legality_support = legality_support_for_touch(&proof)?;
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .map_err(|error| CurrentTopologyInvalidationProofError {
            detail: format!(
                "current derived invalidation authority inventory closeout failed: {error:?}"
            ),
        })?;
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .map_err(|error| CurrentTopologyInvalidationProofError {
                detail: format!("current derived invalidation family catalog failed: {error:?}"),
            })?;
    let catalog_closeout =
        DerivedInvalidationFamilyCatalogCloseout::close(catalog).map_err(|error| {
            CurrentTopologyInvalidationProofError {
                detail: format!(
                    "current derived invalidation family catalog closeout failed: {error:?}"
                ),
            }
        })?;
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout,
        &touched_closure,
        &DerivedInvalidationQuerySupportEvidence::missing(),
        &legality_support,
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .map_err(|error| CurrentTopologyInvalidationProofError {
        detail: format!("current topology invalidation selected plan failed: {error:?}"),
    })?;

    let current_proof = CurrentTopologyInvalidationProof {
        declaration,
        touched_closure,
        selected_plan,
    };
    let _ = CACHE.set(current_proof.clone());
    Ok(current_proof)
}

impl CurrentTopologyInvalidationProof {
    pub(crate) fn declaration(&self) -> &TopologyRewireLoopSuccessorProgramDeclaration {
        &self.declaration
    }

    pub(crate) fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        &self.touched_closure
    }

    pub(crate) fn selected_plan(&self) -> &DerivedInvalidationSelectedPlan {
        &self.selected_plan
    }
}

impl CurrentTopologyInvalidationProofError {
    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

fn current_topology_invalidation_declaration(
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, CurrentTopologyInvalidationProofError> {
    static CACHE: OnceLock<TopologyRewireLoopSuccessorProgramDeclaration> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let mut runtime = crate::validation::reference_integrity::build_milestone_one_runtime()
        .map_err(|error| CurrentTopologyInvalidationProofError {
            detail: format!("current topology invalidation runtime did not build: {error:?}"),
        })?;
    let _verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "phase13.current-topology-invalidation-proof",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .map_err(current_runtime_error)?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "phase13.current-topology-invalidation-proof")
        .map_err(current_runtime_error)?;
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).map_err(current_runtime_error)?;
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &workspace.read::<serde_json::Value>(surfaces.relations()),
        TopologyRelationKind::HalfEdgeNext,
    )
    .map_err(CurrentTopologyInvalidationProofError::new)?;
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .map_err(current_runtime_error)?
        .admit()
        .map_err(current_runtime_error)?;
    let local_rewire = handle
        .topology_reads(&mut workspace)
        .local_rewire_neighborhood(
            &TopologyReadAnchorIdentity::from_runtime_row_label(moved_half_edge_identity),
            6,
        )
        .map_err(current_runtime_error)?;
    let chosen_successor_identity = successor_candidate_with_retained_predecessor(
        &local_rewire,
        3,
        "phase 13 current topology invalidation proof",
    )
    .map_err(CurrentTopologyInvalidationProofError::new)?;
    let declaration = successor_relocation_declaration(&local_rewire, &chosen_successor_identity)
        .map_err(CurrentTopologyInvalidationProofError::new)?;
    let _ = CACHE.set(declaration.clone());
    Ok(declaration)
}

pub(crate) fn current_topology_invalidation_declared_touch_proof(
) -> Result<TopologyDeclaredTouchedGraphBasisProof, CurrentTopologyInvalidationProofError> {
    static CACHE: OnceLock<TopologyDeclaredTouchedGraphBasisProof> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let declared_touch_proof = current_topology_invalidation_declaration()?
        .declared_touched_basis_proof(
            "topology.rewire_loop_successor_program",
            TopologyTouchedOperatingWorld::mainline(),
        )
        .map_err(current_runtime_error)?;
    let _ = CACHE.set(declared_touch_proof.clone());
    Ok(declared_touch_proof)
}

fn legality_support_for_touch(
    proof: &TopologyDeclaredTouchedGraphBasisProof,
) -> Result<DerivedInvalidationLegalitySupportEvidence, CurrentTopologyInvalidationProofError> {
    let selection_closeout =
        current_topology_validator_invariant_selection_closeout_for_declared_touch(proof).map_err(
            |error| CurrentTopologyInvalidationProofError {
                detail: format!(
                    "current topology invalidation validator selection closeout failed: {error:?}"
                ),
            },
        )?;
    Ok(
        DerivedInvalidationLegalitySupportEvidence::from_selected_legality_plan(
            selection_closeout.selected_plan(),
        ),
    )
}

fn current_runtime_error(error: impl std::fmt::Debug) -> CurrentTopologyInvalidationProofError {
    CurrentTopologyInvalidationProofError {
        detail: format!("current topology invalidation proof did not assemble: {error:?}"),
    }
}

impl CurrentTopologyInvalidationProofError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}
