use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::seed::WorthGraphReadAccessInventorySeed;
use super::scope_binding::WorthGraphReadAccessScopeBinding;
use super::scope_family::WorthGraphReadAccessScopeFamily;

pub(in crate::graph_read_access_inventory::inventory_lane) fn graph_read_scope_binding_for_covered_source(
    source_path: &str,
    seed: &WorthGraphReadAccessInventorySeed,
) -> Result<WorthGraphReadAccessScopeBinding, WorthGraphReadAccessInventoryError> {
    let scope_plan = scope_plan_for_covered_source(source_path)?;
    scope_plan.bind(source_path, seed)
}

enum WorthGraphReadAccessCoveredSourceScopePlan {
    TopologyReadProof {
        selected_obligation_index: usize,
    },
    TouchedAuthority {
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
    },
    TouchDescriptor {
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
    },
    SelectedObligation {
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
    },
    SpatialContinuation {
        selected_obligation_index: usize,
    },
    DeletedGraphReadSource,
    CertificationBoundary,
}

impl WorthGraphReadAccessCoveredSourceScopePlan {
    fn bind(
        self,
        source_path: &str,
        seed: &WorthGraphReadAccessInventorySeed,
    ) -> Result<WorthGraphReadAccessScopeBinding, WorthGraphReadAccessInventoryError> {
        match self {
            Self::TopologyReadProof {
                selected_obligation_index,
            } => WorthGraphReadAccessScopeBinding::topology_read_proof(
                source_path,
                selected_obligation_index,
                digest_at(seed.authority_digests(), selected_obligation_index)?,
                digest_at(seed.touch_descriptor_digests(), selected_obligation_index)?,
                digest_at(seed.execution_proof_digests(), selected_obligation_index)?,
            ),
            Self::TouchedAuthority {
                selected_obligation_index,
                scope_family,
            } => WorthGraphReadAccessScopeBinding::touched_authority_digest(
                source_path,
                selected_obligation_index,
                scope_family,
                digest_at(seed.authority_digests(), selected_obligation_index)?,
                digest_at(seed.touch_descriptor_digests(), selected_obligation_index)?,
                digest_at(seed.execution_proof_digests(), selected_obligation_index)?,
            ),
            Self::TouchDescriptor {
                selected_obligation_index,
                scope_family,
            } => WorthGraphReadAccessScopeBinding::touch_descriptor_digest(
                source_path,
                selected_obligation_index,
                scope_family,
                digest_at(seed.authority_digests(), selected_obligation_index)?,
                digest_at(seed.touch_descriptor_digests(), selected_obligation_index)?,
                digest_at(seed.execution_proof_digests(), selected_obligation_index)?,
            ),
            Self::SelectedObligation {
                selected_obligation_index,
                scope_family,
            } => WorthGraphReadAccessScopeBinding::selected_obligation(
                source_path,
                selected_obligation_index,
                scope_family,
                digest_at(seed.authority_digests(), selected_obligation_index)?,
                digest_at(seed.touch_descriptor_digests(), selected_obligation_index)?,
                digest_at(seed.execution_proof_digests(), selected_obligation_index)?,
                digest_at(
                    seed.selected_registration_digests(),
                    selected_obligation_index,
                )?,
            ),
            Self::SpatialContinuation {
                selected_obligation_index,
            } => WorthGraphReadAccessScopeBinding::spatial_continuation_proof(
                source_path,
                selected_obligation_index,
                digest_at(seed.authority_digests(), selected_obligation_index)?,
                digest_at(seed.touch_descriptor_digests(), selected_obligation_index)?,
                digest_at(seed.execution_proof_digests(), selected_obligation_index)?,
            ),
            Self::DeletedGraphReadSource => {
                WorthGraphReadAccessScopeBinding::deleted_graph_read_source(
                    source_path,
                    first_adoption_manifest_digest(seed)?,
                )
            }
            Self::CertificationBoundary => {
                WorthGraphReadAccessScopeBinding::certification_boundary(
                    source_path,
                    format!("certification-boundary:{source_path}"),
                )
            }
        }
    }
}

fn scope_plan_for_covered_source(
    source_path: &str,
) -> Result<WorthGraphReadAccessCoveredSourceScopePlan, WorthGraphReadAccessInventoryError> {
    match source_path {
        "crates/worth-topo/src/projection/read_views/domain" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::TopologyReadProof {
                selected_obligation_index: 0,
            })
        }
        "crates/worth-topo/src/projection/runtime_boundary/read_execution" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::TouchedAuthority {
                selected_obligation_index: 0,
                scope_family: WorthGraphReadAccessScopeFamily::TopologyRuntimeReadExecution,
            })
        }
        "crates/worth-spatial/src/workload_platform/evidence_ledger"
        | "crates/worth-kernel/src/workload_composition" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::SelectedObligation {
                selected_obligation_index: selected_obligation_index_for_source(source_path),
                scope_family: selected_obligation_scope_family_for_source(source_path),
            })
        }
        "crates/worth-kernel/src/binding" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::TouchDescriptor {
                selected_obligation_index: 1,
                scope_family: WorthGraphReadAccessScopeFamily::KernelBindingNeighborhood,
            })
        }
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction"
        | "crates/worth-spatial/src/workload_platform/planar_boolean_events" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::SpatialContinuation {
                selected_obligation_index: 1,
            })
        }
        "crates/worth-kernel/src/query_adoption/graph_read_access" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::DeletedGraphReadSource)
        }
        "crates/worth-topo/src/projection/read_views/domain/read_proof"
        | "crates/worth-topo/src/certification/projection_closeout/tests/topology_reads"
        | "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support"
        | "crates/worth-kernel/src/binding/tests" => {
            Ok(WorthGraphReadAccessCoveredSourceScopePlan::CertificationBoundary)
        }
        _ => Err(error(
            WorthGraphReadAccessInventoryErrorKind::MissingScopeBinding,
        )),
    }
}

fn selected_obligation_index_for_source(source_path: &str) -> usize {
    match source_path {
        "crates/worth-spatial/src/workload_platform/evidence_ledger" => 1,
        _ => 0,
    }
}

fn selected_obligation_scope_family_for_source(
    source_path: &str,
) -> WorthGraphReadAccessScopeFamily {
    match source_path {
        "crates/worth-spatial/src/workload_platform/evidence_ledger" => {
            WorthGraphReadAccessScopeFamily::SpatialEvidenceLookup
        }
        _ => WorthGraphReadAccessScopeFamily::KernelWorkloadComposition,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessScopeSubstitutionRole {
    SelectedObligationScope,
    GraphReadDeclaration,
    AdmittedAccessPlan,
    ReadAccessReceipt,
    NoNPlusOneExecutionProof,
}

pub(in crate::graph_read_access_inventory::inventory_lane) fn reject_read_access_plan_scope_substitution(
    claimed_surface: WorthGraphReadAccessScopeSubstitutionRole,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    match claimed_surface {
        WorthGraphReadAccessScopeSubstitutionRole::GraphReadDeclaration
        | WorthGraphReadAccessScopeSubstitutionRole::AdmittedAccessPlan
        | WorthGraphReadAccessScopeSubstitutionRole::ReadAccessReceipt
        | WorthGraphReadAccessScopeSubstitutionRole::NoNPlusOneExecutionProof => Err(error(
            WorthGraphReadAccessInventoryErrorKind::SelectedObligationRelabelledAsReadAccessPlan,
        )),
        WorthGraphReadAccessScopeSubstitutionRole::SelectedObligationScope => Ok(()),
    }
}

fn first_adoption_manifest_digest(
    seed: &WorthGraphReadAccessInventorySeed,
) -> Result<&str, WorthGraphReadAccessInventoryError> {
    first_digest(
        seed.adoption_manifest_digests(),
        WorthGraphReadAccessInventoryErrorKind::MissingAdoptionManifestDigest,
    )
}

fn digest_at(digests: &[String], index: usize) -> Result<&str, WorthGraphReadAccessInventoryError> {
    digests
        .get(index)
        .map(String::as_str)
        .filter(|digest| !digest.is_empty())
        .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingScopeEvidence))
}

fn first_digest(
    digests: &[String],
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<&str, WorthGraphReadAccessInventoryError> {
    digests
        .first()
        .map(String::as_str)
        .filter(|digest| !digest.is_empty())
        .ok_or_else(|| error(error_kind))
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
