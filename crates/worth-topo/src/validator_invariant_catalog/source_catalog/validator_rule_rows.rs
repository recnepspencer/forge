use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture,
};

use super::{
    WorthTopologyLegalityFamilySourceAuthorityKind, WorthTopologyLegalityFamilySourceProof,
    WorthTopologyLegalityFamilySourceProofInput, WorthTopologyValidatorFamilySourceRow,
};
use crate::topology_operators::{
    TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedScope,
};
use crate::validation::derived_topology_rule_specs;
use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::{
    WorthTopologyDiagnosticProjectionPosture, WorthTopologyEnforcementPhase,
    WorthTopologyLegalityCatalogError, WorthTopologyRequiredAccessPosture,
    WorthTopologyTouchedApplicability, WorthTopologyValidatorFamilyIdentity,
    WorthTopologyWitnessPosture,
};

pub(in crate::validator_invariant_catalog) fn current_validator_family_inputs(
    milestone_eight_posture_digest: &str,
) -> Result<Vec<WorthTopologyValidatorFamilySourceRow>, WorthTopologyLegalityCatalogError> {
    derived_topology_rule_specs()
        .iter()
        .map(|spec| {
            let identity =
                WorthTopologyValidatorFamilyIdentity::from_registered_rule((spec.identity)());
            let touched_applicability = applicability_for_validator(spec.name)?;
            let enforcement_phase = WorthTopologyEnforcementPhase::SelectedObligationExecution;
            let witness_posture = witness_for_validator(spec.name)?;
            let input = WorthTopologyLegalityFamilyRecordInput {
                identity: identity.clone(),
                query_obligation_kind: ForgeQueryGraphObligationKind::SchemaContractValidator,
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
                        WorthTopologyLegalityFamilySourceAuthorityKind::ValidatorRuleSpec,
                    source_identity_digest: identity.identity_digest().to_string(),
                    rule_name: spec.name.to_string(),
                    semantic_version: identity.semantic_version().to_string(),
                    execution_point: None,
                    applicability_digest: touched_applicability.digest_part(),
                    enforcement_phase,
                    witness_posture,
                },
            );
            Ok(WorthTopologyValidatorFamilySourceRow {
                input,
                source_proof,
            })
        })
        .collect()
}

fn applicability_for_validator(
    name: &str,
) -> Result<WorthTopologyTouchedApplicability, WorthTopologyLegalityCatalogError> {
    Ok(match name {
        "ownership" => WorthTopologyTouchedApplicability::from_parts(
            [TopologyTouchedAspect::TopologyOwnership],
            [
                TopologyTouchedScope::Entity,
                TopologyTouchedScope::LocalNeighborhood,
            ],
            TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        ),
        "loop_wiring" => WorthTopologyTouchedApplicability::from_parts(
            [TopologyTouchedAspect::TopologyBoundary],
            [TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
            TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        ),
        "radial_rings" => WorthTopologyTouchedApplicability::from_parts(
            [TopologyTouchedAspect::TopologyRadial],
            [
                TopologyTouchedScope::RadialNeighborhood,
                TopologyTouchedScope::Relation,
            ],
            TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        ),
        "shell_closure" => WorthTopologyTouchedApplicability::from_parts(
            [TopologyTouchedAspect::TopologyBoundary],
            [
                TopologyTouchedScope::Shell,
                TopologyTouchedScope::LocalNeighborhood,
            ],
            TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        ),
        "vertex_disks" => WorthTopologyTouchedApplicability::from_parts(
            [TopologyTouchedAspect::TopologyRadial],
            [
                TopologyTouchedScope::RadialNeighborhood,
                TopologyTouchedScope::LocalNeighborhood,
            ],
            TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        ),
        _ => {
            return Err(
                WorthTopologyLegalityCatalogError::UnknownValidatorApplicability(name.to_string()),
            );
        }
    })
}

fn witness_for_validator(
    name: &str,
) -> Result<WorthTopologyWitnessPosture, WorthTopologyLegalityCatalogError> {
    Ok(match name {
        "ownership" => WorthTopologyWitnessPosture::TouchedNeighborhood,
        "loop_wiring" | "radial_rings" => WorthTopologyWitnessPosture::TouchedRelations,
        "shell_closure" | "vertex_disks" => WorthTopologyWitnessPosture::TouchedNeighborhood,
        _ => {
            return Err(
                WorthTopologyLegalityCatalogError::UnknownValidatorWitnessPosture(name.to_string()),
            );
        }
    })
}
