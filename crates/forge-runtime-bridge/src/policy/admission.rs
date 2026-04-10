use super::{
    AdmittedBridgePolicyContract, AdmittedBridgePolicyContractParts, BridgePolicyAuthorityInputs,
    BridgePolicyDeclaration, BridgePolicyFieldKind, BridgePolicyRejection,
    BridgePolicyRejectionKind, BridgePolicyRejectionStage, BridgePolicyResolution,
    BridgePolicyResolutionEntry, BridgePolicySourceClass, ValidatedBridgePolicyDeclaration,
};

pub(crate) fn admit_policy_declaration(
    validated: ValidatedBridgePolicyDeclaration,
    authority_inputs: BridgePolicyAuthorityInputs,
) -> Result<AdmittedBridgePolicyContract, BridgePolicyRejection> {
    let declaration = validated.declaration().clone();

    reject_illegal_replay_bundle(&declaration, authority_inputs)?;
    reject_illegal_route_artifact_bundle(&declaration, authority_inputs)?;

    let resolved_diagnostics_tier = declaration
        .diagnostics_tier()
        .min(authority_inputs.baseline_diagnostics_tier());
    let diagnostics_resolution = if resolved_diagnostics_tier == declaration.diagnostics_tier() {
        BridgePolicyResolution::AcceptedAsDeclared
    } else {
        BridgePolicyResolution::Narrowed
    };

    Ok(AdmittedBridgePolicyContract::new(
        AdmittedBridgePolicyContractParts {
            validated_declaration: validated,
            authority_inputs,
            resolved_execution_class: declaration.execution_class(),
            resolved_diagnostics_tier,
            resolved_route_artifacts: declaration.request_route_artifacts(),
            resolved_replay_artifacts: declaration.require_replay_artifacts(),
            resolution_entries: vec![
                BridgePolicyResolutionEntry::new(
                    BridgePolicyFieldKind::ExecutionMode,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicyResolution::AcceptedAsDeclared,
                ),
                BridgePolicyResolutionEntry::new(
                    BridgePolicyFieldKind::DiagnosticsTier,
                    BridgePolicySourceClass::RequestDeclared,
                    if diagnostics_resolution == BridgePolicyResolution::AcceptedAsDeclared {
                        BridgePolicySourceClass::RequestDeclared
                    } else {
                        BridgePolicySourceClass::RuntimeBaseline
                    },
                    diagnostics_resolution,
                ),
                BridgePolicyResolutionEntry::new(
                    BridgePolicyFieldKind::ReplayArtifacts,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicyResolution::AcceptedAsDeclared,
                ),
                BridgePolicyResolutionEntry::new(
                    BridgePolicyFieldKind::ArtifactRetention,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicySourceClass::RequestDeclared,
                    BridgePolicyResolution::AcceptedAsDeclared,
                ),
            ],
        },
    ))
}

fn reject_illegal_replay_bundle(
    declaration: &BridgePolicyDeclaration,
    authority_inputs: BridgePolicyAuthorityInputs,
) -> Result<(), BridgePolicyRejection> {
    if declaration.require_replay_artifacts() && !authority_inputs.replay_artifacts_permitted() {
        return Err(BridgePolicyRejection::new(
            declaration,
            BridgePolicyRejectionKind::ReplayPolicyConflict,
            BridgePolicyRejectionStage::Admission,
            BridgePolicyFieldKind::ReplayArtifacts,
            BridgePolicySourceClass::RequestDeclared,
            BridgePolicySourceClass::RuntimeBaseline,
            "request required replay artifacts but the runtime baseline forbids replay retention",
        ));
    }

    Ok(())
}

fn reject_illegal_route_artifact_bundle(
    declaration: &BridgePolicyDeclaration,
    authority_inputs: BridgePolicyAuthorityInputs,
) -> Result<(), BridgePolicyRejection> {
    if declaration.request_route_artifacts() && !authority_inputs.route_artifacts_permitted() {
        return Err(BridgePolicyRejection::new(
            declaration,
            BridgePolicyRejectionKind::ArtifactRetentionConflict,
            BridgePolicyRejectionStage::Admission,
            BridgePolicyFieldKind::ArtifactRetention,
            BridgePolicySourceClass::RequestDeclared,
            BridgePolicySourceClass::RuntimeBaseline,
            "request required route artifacts but the runtime baseline forbids route artifact retention",
        ));
    }

    Ok(())
}
