use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture,
};
use forge_relational::facade::runtime::{CustomInvariantRegistration, InvariantExecutionPoint};

use super::{
    WorthTopologyInvariantFamilySourceRow, WorthTopologyLegalityFamilySourceAuthorityKind,
    WorthTopologyLegalityFamilySourceProof, WorthTopologyLegalityFamilySourceProofInput,
};
use crate::runtime_support::milestone_one_invariant_registrations;
use crate::topology_operators::{
    TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedScope,
};
use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::{
    WorthTopologyDiagnosticProjectionPosture, WorthTopologyEnforcementPhase,
    WorthTopologyInvariantFamilyIdentity, WorthTopologyLegalityCatalogError,
    WorthTopologyRequiredAccessPosture, WorthTopologyTouchedApplicability,
    WorthTopologyWitnessPosture,
};

pub(in crate::validator_invariant_catalog) fn current_invariant_family_inputs(
    milestone_eight_posture_digest: &str,
) -> Result<Vec<WorthTopologyInvariantFamilySourceRow>, WorthTopologyLegalityCatalogError> {
    let registrations = milestone_one_invariant_registrations().map_err(|error| {
        WorthTopologyLegalityCatalogError::InvariantRegistration(format!("{error:?}"))
    })?;
    registrations
        .iter()
        .map(|registration| {
            let identity = invariant_family_identity(registration);
            let touched_applicability = applicability_for_invariant(registration)?;
            let enforcement_phase = enforcement_for_invariant(registration.execution_point())?;
            let witness_posture = witness_for_invariant(registration.rule_id().as_str())?;
            let input = WorthTopologyLegalityFamilyRecordInput {
                identity: identity.clone(),
                query_obligation_kind: ForgeQueryGraphObligationKind::BlockingInvariant,
                touched_applicability: Some(touched_applicability.clone()),
                required_access_posture: Some(
                    WorthTopologyRequiredAccessPosture::milestone_eight_receipt_backed(
                        milestone_eight_posture_digest,
                    ),
                ),
                enforcement_phase: Some(enforcement_phase),
                witness_posture: Some(witness_posture),
                diagnostic_projection: Some(
                    WorthTopologyDiagnosticProjectionPosture::ViolationWitness,
                ),
                query_support_posture: ForgeQueryGraphObligationSupportPosture::supported(
                    ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
                ),
            };
            let source_proof = WorthTopologyLegalityFamilySourceProof::new(
                WorthTopologyLegalityFamilySourceProofInput {
                    authority_kind:
                        WorthTopologyLegalityFamilySourceAuthorityKind::RuntimeInvariantRegistration,
                    source_identity_digest: identity.identity_digest().to_string(),
                    rule_name: registration.rule_id().as_str().to_string(),
                    semantic_version: identity.semantic_version().to_string(),
                    execution_point: Some(registration.execution_point().diagnostic_label().to_string()),
                    applicability_digest: touched_applicability.digest_part(),
                    enforcement_phase,
                    witness_posture,
                },
            );
            Ok(WorthTopologyInvariantFamilySourceRow {
                input,
                source_proof,
            })
        })
        .collect::<Result<Vec<_>, WorthTopologyLegalityCatalogError>>()
}

fn invariant_family_identity(
    registration: &CustomInvariantRegistration,
) -> WorthTopologyInvariantFamilyIdentity {
    let descriptor = registration.descriptor();
    WorthTopologyInvariantFamilyIdentity::registered(
        invariant_family_name(registration),
        format!(
            "v{}.{}",
            descriptor.identity.semantic_version.major, descriptor.identity.semantic_version.minor
        ),
    )
}

fn invariant_family_name(registration: &CustomInvariantRegistration) -> String {
    format!(
        "{}.{}",
        registration.rule_id().as_str(),
        registration.execution_point().diagnostic_label()
    )
}

fn applicability_for_invariant(
    registration: &CustomInvariantRegistration,
) -> Result<WorthTopologyTouchedApplicability, WorthTopologyLegalityCatalogError> {
    let rule_id = registration.rule_id().as_str();
    let (aspect, scope) = applicability_axis_for_rule(rule_id).ok_or_else(|| {
        WorthTopologyLegalityCatalogError::UnknownInvariantApplicability(rule_id.to_string())
    })?;
    Ok(WorthTopologyTouchedApplicability::from_parts(
        [aspect],
        [scope],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    ))
}

fn applicability_axis_for_rule(
    rule_id: &str,
) -> Option<(TopologyTouchedAspect, TopologyTouchedScope)> {
    Some(match rule_id {
        "naming" | ".m1.naming.coverage" => (
            TopologyTouchedAspect::NamingPersistentName,
            TopologyTouchedScope::Naming,
        ),
        "radial_rings" | ".m1.topology.radial_surface" => (
            TopologyTouchedAspect::TopologyRadial,
            TopologyTouchedScope::RadialNeighborhood,
        ),
        "vertex_disks" | ".m1.topology.vertex_disks" => (
            TopologyTouchedAspect::TopologyRadial,
            TopologyTouchedScope::RadialNeighborhood,
        ),
        "loop_wiring" | ".m1.topology.loop_wiring" => (
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedScope::Loop,
        ),
        "shell_closure" | ".m1.topology.shell_closure" => (
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedScope::Shell,
        ),
        "wire_connectivity" | ".m1.topology.wire_connectivity" => (
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedScope::Wire,
        ),
        "ownership" | ".m1.topology.ownership_surface" => (
            TopologyTouchedAspect::TopologyOwnership,
            TopologyTouchedScope::LocalNeighborhood,
        ),
        _ => return None,
    })
}

fn enforcement_for_invariant(
    execution_point: InvariantExecutionPoint,
) -> Result<WorthTopologyEnforcementPhase, WorthTopologyLegalityCatalogError> {
    match execution_point {
        InvariantExecutionPoint::CommitBoundary => {
            Ok(WorthTopologyEnforcementPhase::CommitBackstop)
        }
        InvariantExecutionPoint::GraphComposition => {
            Ok(WorthTopologyEnforcementPhase::SelectedObligationExecution)
        }
        _ => Err(
            WorthTopologyLegalityCatalogError::UnknownInvariantExecutionPoint(
                execution_point.diagnostic_label().to_string(),
            ),
        ),
    }
}

fn witness_for_invariant(
    rule_id: &str,
) -> Result<WorthTopologyWitnessPosture, WorthTopologyLegalityCatalogError> {
    Ok(match rule_id {
        "ownership" | ".m1.topology.ownership_surface" | "naming" | ".m1.naming.coverage" => {
            WorthTopologyWitnessPosture::TouchedFacts
        }
        "radial_rings"
        | ".m1.topology.radial_surface"
        | "vertex_disks"
        | ".m1.topology.vertex_disks"
        | "loop_wiring"
        | ".m1.topology.loop_wiring"
        | "shell_closure"
        | ".m1.topology.shell_closure"
        | "wire_connectivity"
        | ".m1.topology.wire_connectivity" => WorthTopologyWitnessPosture::TouchedNeighborhood,
        _ => {
            return Err(
                WorthTopologyLegalityCatalogError::UnknownInvariantWitnessPosture(
                    rule_id.to_string(),
                ),
            );
        }
    })
}
