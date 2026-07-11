use crate::{
    access_shapes,
    artifact_family::ArtifactFamilyDenial,
    materialization::{
        S8LayoutCoverageWitness, S8LayoutMaterializationState, S8PhysicalCoverageBasis,
    },
    phase26_rules::{
        AdmittedBackgroundPacingLayoutRule, AdmittedColdRecallLayoutRule,
        AdmittedForegroundInterferenceLayoutRule, AdmittedMaintenanceQueueLayoutRule,
        AdmittedRecallAmplificationLayoutRule, AdmittedSchedulerReservationLayoutRule,
        AdmittedTierPlacementLayoutRule,
    },
    PhysicalArtifactFamilyDeclaration, S8AccessShape,
};
use crate::{Phase19LayoutRuleDenial, S8AccessLaneClassification};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;

pub fn phase26_maintenance_queue_rule(
) -> Result<AdmittedMaintenanceQueueLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::MaintenanceQueueDeclaration)?;
    Ok(AdmittedMaintenanceQueueLayoutRule::phase26())
}

pub fn phase26_scheduler_reservation_rule(
) -> Result<AdmittedSchedulerReservationLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family(DurableArtifactFamilyId::SchedulerReservationIndex)?;
    Ok(AdmittedSchedulerReservationLayoutRule::phase26())
}

pub fn phase26_tier_placement_rule(
) -> Result<AdmittedTierPlacementLayoutRule, Phase19LayoutRuleDenial> {
    validate_bounded_scan_family(DurableArtifactFamilyId::TierPlacementManifest)?;
    Ok(AdmittedTierPlacementLayoutRule::phase26())
}

pub fn phase26_cold_recall_rule() -> Result<AdmittedColdRecallLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::ColdRecallQueue)?;
    Ok(AdmittedColdRecallLayoutRule::phase26())
}

pub fn phase26_recall_amplification_rule(
) -> Result<AdmittedRecallAmplificationLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::RecallAmplificationIndex)?;
    Ok(AdmittedRecallAmplificationLayoutRule::phase26())
}

pub fn phase26_background_pacing_rule(
) -> Result<AdmittedBackgroundPacingLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::BackgroundPacingRecord)?;
    Ok(AdmittedBackgroundPacingLayoutRule::phase26())
}

pub fn phase26_foreground_interference_rule(
) -> Result<AdmittedForegroundInterferenceLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family(DurableArtifactFamilyId::ForegroundInterferenceRecord)?;
    Ok(AdmittedForegroundInterferenceLayoutRule::phase26())
}

fn validate_exact_point_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration(family_id)?;
    let point_lookup = access_shapes()
        .point_lookup(exact_coverage(declaration))
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if point_lookup.shape() != S8AccessShape::PointLookup {
        return Err(Phase19LayoutRuleDenial::WrongShape(point_lookup.shape()));
    }
    Ok(())
}

fn validate_bounded_scan_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration(family_id)?;
    let bounded = access_shapes()
        .bounded_scan(
            exact_coverage(declaration),
            S8AccessLaneClassification::Foreground,
            crate::S8BoundedScanBasis::LocalityBoundedTraversal,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if bounded.shape() != S8AccessShape::BoundedScan {
        return Err(Phase19LayoutRuleDenial::WrongShape(bounded.shape()));
    }
    Ok(())
}

fn validate_maintenance_bounded_scan_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = declaration(family_id)?;
    let bounded = access_shapes()
        .bounded_scan(
            exact_coverage(declaration),
            S8AccessLaneClassification::Maintenance,
            crate::S8BoundedScanBasis::LocalityBoundedTraversal,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if bounded.shape() != S8AccessShape::BoundedScan {
        return Err(Phase19LayoutRuleDenial::WrongShape(bounded.shape()));
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
        PhysicalEpoch::from_raw(1).expect("phase-26 coverage watermark must be non-zero"),
    )
    .watermark();
    S8LayoutCoverageWitness::exact_through(
        S8LayoutMaterializationState::exact_through_physical_basis(declaration.family()),
        watermark,
    )
    .expect("phase-26 exact physical basis coverage must stay well-formed")
}
