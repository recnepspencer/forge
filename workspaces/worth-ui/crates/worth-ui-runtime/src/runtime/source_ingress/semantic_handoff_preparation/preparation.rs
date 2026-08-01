use crate::capability::CapabilitySnapshot;
use crate::declaration::prepare_authored_intent_material;
use crate::source::{
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiStructuralLegalityLowerer,
};
use worth_ui_dsl::WorthUiSealedSemanticPackage;

use super::{
    prepare_declaration_material, WorthUiPreparedSemanticHandoffMaterial,
    WorthUiSemanticHandoffEvidence, WorthUiSemanticHandoffPreparationDenial,
    WorthUiSemanticHandoffPreparationStop,
};

pub(in crate::runtime::source_ingress) fn prepare_semantic_handoff(
    package: WorthUiSealedSemanticPackage,
    snapshot: &CapabilitySnapshot,
) -> Result<WorthUiPreparedSemanticHandoffMaterial, WorthUiSemanticHandoffPreparationDenial> {
    let mut evidence = WorthUiSemanticHandoffEvidence::from_package(&package);
    if !package.protocol().is_current() {
        return Err(denial(
            evidence,
            WorthUiSemanticHandoffPreparationStop::UnsupportedProtocol,
        ));
    }
    let intent_material = prepare_authored_intent_material(&package).map_err(|_| {
        denial(
            evidence.clone(),
            WorthUiSemanticHandoffPreparationStop::IntentDeclaration,
        )
    })?;
    evidence.admit_intent_material(intent_material);
    let resolved = WorthUiArtifactInputResolver::resolve(&package, snapshot).map_err(|_| {
        denial(
            evidence.clone(),
            WorthUiSemanticHandoffPreparationStop::CapabilityResolution,
        )
    })?;
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).map_err(|_| {
            denial(
                evidence.clone(),
                WorthUiSemanticHandoffPreparationStop::RuntimeStructuralAdmission,
            )
        })?;
    let declaration_material =
        prepare_declaration_material(&package, &structured).map_err(|_| {
            denial(
                evidence.clone(),
                WorthUiSemanticHandoffPreparationStop::DeclarationProjection,
            )
        })?;
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).map_err(|_| {
        denial(
            evidence.clone(),
            WorthUiSemanticHandoffPreparationStop::BindingAdmission,
        )
    })?;
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .map_err(|_| {
            denial(
                evidence.clone(),
                WorthUiSemanticHandoffPreparationStop::IdentitySeeding,
            )
        })?
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded).map_err(|_| {
        denial(
            evidence.clone(),
            WorthUiSemanticHandoffPreparationStop::CanonicalAssembly,
        )
    })?;
    Ok(WorthUiPreparedSemanticHandoffMaterial::new(
        artifact,
        declaration_material,
        evidence,
    ))
}

fn denial(
    evidence: WorthUiSemanticHandoffEvidence,
    stop: WorthUiSemanticHandoffPreparationStop,
) -> WorthUiSemanticHandoffPreparationDenial {
    WorthUiSemanticHandoffPreparationDenial::new(evidence, stop)
}
