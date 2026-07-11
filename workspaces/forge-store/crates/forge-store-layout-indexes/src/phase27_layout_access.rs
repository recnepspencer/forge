use crate::Phase19LayoutRuleDenial;
use crate::{
    access_shapes,
    artifact_family::ArtifactFamilyDenial,
    materialization::{
        S8LayoutCoverageWitness, S8LayoutMaterializationState, S8PhysicalCoverageBasis,
    },
    PhysicalArtifactFamilyDeclaration, S8AccessShape,
};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_security::{
    AdmittedAuthenticityLayoutRule, AdmittedCustodyLayoutRule, AdmittedKeyScopeLayoutRule,
    AdmittedRepairBlastRadiusLayoutRule, AdmittedTenantScopeLayoutRule,
};

pub fn phase27_tenant_scope_rule() -> Result<AdmittedTenantScopeLayoutRule, Phase19LayoutRuleDenial>
{
    validate_exact_point_family()?;
    Ok(AdmittedTenantScopeLayoutRule::phase27())
}

pub fn phase27_key_scope_rule() -> Result<AdmittedKeyScopeLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family()?;
    Ok(AdmittedKeyScopeLayoutRule::phase27())
}

pub fn phase27_authenticity_rule() -> Result<AdmittedAuthenticityLayoutRule, Phase19LayoutRuleDenial>
{
    validate_exact_point_family()?;
    Ok(AdmittedAuthenticityLayoutRule::phase27())
}

pub fn phase27_custody_rule() -> Result<AdmittedCustodyLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family()?;
    Ok(AdmittedCustodyLayoutRule::phase27())
}

pub fn phase27_repair_blast_radius_rule(
) -> Result<AdmittedRepairBlastRadiusLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family()?;
    Ok(AdmittedRepairBlastRadiusLayoutRule::phase27())
}

fn validate_exact_point_family() -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration()?;
    let point_lookup = access_shapes()
        .point_lookup(exact_coverage(declaration))
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if point_lookup.shape() != S8AccessShape::PointLookup {
        return Err(Phase19LayoutRuleDenial::WrongShape(point_lookup.shape()));
    }
    Ok(())
}

fn declaration() -> Result<&'static PhysicalArtifactFamilyDeclaration, Phase19LayoutRuleDenial> {
    crate::layout_declarations()
        .declaration(DurableArtifactFamilyId::SecurityCustodyLookup)
        .map_err(|denial: ArtifactFamilyDenial| Phase19LayoutRuleDenial::Family(denial))
}

fn exact_coverage(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
) -> S8LayoutCoverageWitness {
    let watermark = S8PhysicalCoverageBasis::root_epoch(
        PhysicalEpoch::from_raw(1).expect("phase-27 coverage watermark must be non-zero"),
    )
    .watermark();
    S8LayoutCoverageWitness::exact_through(
        S8LayoutMaterializationState::exact_through_physical_basis(declaration.family()),
        watermark,
    )
    .expect("phase-27 exact physical basis coverage must stay well-formed")
}
