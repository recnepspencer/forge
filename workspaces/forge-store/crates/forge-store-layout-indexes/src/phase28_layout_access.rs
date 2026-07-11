use crate::Phase19LayoutRuleDenial;
use crate::{
    access_shapes,
    artifact_family::ArtifactFamilyDenial,
    materialization::{
        S8LayoutCoverageWitness, S8LayoutMaterializationState, S8PhysicalCoverageBasis,
    },
    AdmittedCapsuleManifestLayoutRule, AdmittedExportBundleLayoutRule,
    AdmittedImportReadmissionLayoutRule, AdmittedOfflineVerifierLayoutRule,
    AdmittedRestoreEvidenceLayoutRule, PhysicalArtifactFamilyDeclaration,
    S8AccessLaneClassification, S8AccessShape, S8FullDeclaredScanBasis,
};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;

pub fn phase28_export_bundle_rule(
) -> Result<AdmittedExportBundleLayoutRule, Phase19LayoutRuleDenial> {
    validate_terminal_manifest_family(DurableArtifactFamilyId::ExportBundle)?;
    Ok(AdmittedExportBundleLayoutRule::internal_phase28())
}

pub fn phase28_capsule_manifest_rule(
) -> Result<AdmittedCapsuleManifestLayoutRule, Phase19LayoutRuleDenial> {
    validate_terminal_manifest_family(DurableArtifactFamilyId::CapsuleArtifact)?;
    Ok(AdmittedCapsuleManifestLayoutRule::internal_phase28())
}

pub fn phase28_offline_verifier_rule(
) -> Result<AdmittedOfflineVerifierLayoutRule, Phase19LayoutRuleDenial> {
    let declaration = declaration(DurableArtifactFamilyId::OfflineVerificationRecord)?;
    let access = access_shapes()
        .full_declared_scan(
            exact_coverage(declaration),
            S8AccessLaneClassification::Verifier,
            S8FullDeclaredScanBasis::DeclaredFullTraversal,
        )
        .into_result()
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if access.shape() != S8AccessShape::FullDeclaredScan {
        return Err(Phase19LayoutRuleDenial::WrongShape(access.shape()));
    }
    Ok(AdmittedOfflineVerifierLayoutRule::internal_phase28())
}

pub fn phase28_restore_evidence_rule(
) -> Result<AdmittedRestoreEvidenceLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family(DurableArtifactFamilyId::ImportBundle)?;
    Ok(AdmittedRestoreEvidenceLayoutRule::internal_phase28())
}

pub fn phase28_import_readmission_rule(
) -> Result<AdmittedImportReadmissionLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family(DurableArtifactFamilyId::ImportBundle)?;
    Ok(AdmittedImportReadmissionLayoutRule::internal_phase28())
}

fn validate_terminal_manifest_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration(family_id)?;
    let access = access_shapes()
        .manifest_graph_walk(
            exact_coverage(declaration),
            S8AccessLaneClassification::Terminal,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if access.shape() != S8AccessShape::ManifestGraphWalk {
        return Err(Phase19LayoutRuleDenial::WrongShape(access.shape()));
    }
    Ok(())
}

fn validate_exact_point_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration(family_id)?;
    let access = access_shapes()
        .point_lookup(exact_coverage(declaration))
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if access.shape() != S8AccessShape::PointLookup {
        return Err(Phase19LayoutRuleDenial::WrongShape(access.shape()));
    }
    Ok(())
}

fn declaration(
    family_id: DurableArtifactFamilyId,
) -> Result<&'static PhysicalArtifactFamilyDeclaration, Phase19LayoutRuleDenial> {
    crate::layout_declarations()
        .declaration(family_id)
        .map_err(|denial: ArtifactFamilyDenial| Phase19LayoutRuleDenial::Family(denial))
}

fn exact_coverage(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
) -> S8LayoutCoverageWitness {
    let watermark = S8PhysicalCoverageBasis::root_epoch(
        PhysicalEpoch::from_raw(1).expect("phase-28 coverage watermark must be non-zero"),
    )
    .watermark();
    S8LayoutCoverageWitness::exact_through(
        S8LayoutMaterializationState::exact_through_physical_basis(declaration.family()),
        watermark,
    )
    .expect("phase-28 exact physical basis coverage must stay well-formed")
}
