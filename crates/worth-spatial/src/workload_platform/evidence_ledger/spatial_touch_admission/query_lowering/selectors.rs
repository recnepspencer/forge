use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchReadVerb,
};

use super::super::{
    SpatialEvidenceLookupProduct, SpatialGeometryEvidenceTouchAuthority,
    SpatialGeometryEvidenceTouchOperatingWorld,
};
use crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind;

pub(super) const SPATIAL_QUERY_COLLECTION: &str = "worth.spatial.evidence_touch";
pub(super) const SPATIAL_QUERY_RELATION_KIND: &str = SPATIAL_QUERY_COLLECTION;

pub(super) fn spatial_query_aspect_paths(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    lookup: &SpatialEvidenceLookupProduct,
) -> Vec<String> {
    let mut paths = vec![
        format!(
            "boolean_stage.{}",
            query_field_segment(boolean_stage_selector_value(authority.boolean_stage()))
        ),
        format!(
            "evidence_stage.{}",
            query_field_segment(authority.evidence_stage().human_name())
        ),
        format!(
            "evidence_identity.{}",
            query_field_segment(lookup.evidence_identity())
        ),
        format!(
            "support.{}",
            query_field_segment(&format!("{:?}", lookup.support()))
        ),
        "spatial_touch.digest".to_string(),
        "spatial_lookup.digest".to_string(),
    ];
    paths.sort();
    paths.dedup();
    paths
}

fn query_field_segment(value: &str) -> String {
    let mut segment = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    if segment.trim().is_empty() {
        segment = "unknown".to_string();
    }
    segment
}

pub(super) fn spatial_query_read_verbs(
    authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Vec<ForgeQueryGraphTouchReadVerb> {
    let mut verbs = vec![
        ForgeQueryGraphTouchReadVerb::ObservesCollection,
        ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
        ForgeQueryGraphTouchReadVerb::ObservesAspect,
        ForgeQueryGraphTouchReadVerb::MaterializesDiagnostic,
    ];
    if authority.operating_world() == SpatialGeometryEvidenceTouchOperatingWorld::CurrentHead {
        verbs.push(ForgeQueryGraphTouchReadVerb::CrossesOperatingWorld);
    }
    verbs
}

pub(super) fn query_operating_world_descriptor(
    operating_world: SpatialGeometryEvidenceTouchOperatingWorld,
) -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    match operating_world {
        SpatialGeometryEvidenceTouchOperatingWorld::CurrentHead => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
        }
    }
}

fn boolean_stage_selector_value(stage: BooleanEvidenceStageKind) -> &'static str {
    match stage {
        BooleanEvidenceStageKind::DeclarationEntry => "declaration-entry",
        BooleanEvidenceStageKind::RoutePlan => "route-plan",
        BooleanEvidenceStageKind::OperandPairConstruction => "operand-pair-construction",
        BooleanEvidenceStageKind::BlockerProvenance => "blocker-provenance",
        BooleanEvidenceStageKind::PrecisionAgreement => "precision-agreement",
        BooleanEvidenceStageKind::SharedPlaneIdentity => "shared-plane-identity",
        BooleanEvidenceStageKind::LocalFrameSelection => "local-frame-selection",
        BooleanEvidenceStageKind::OperandAProjectionConsumption => {
            "operand-a-projection-consumption"
        }
        BooleanEvidenceStageKind::OperandBProjectionConsumption => {
            "operand-b-projection-consumption"
        }
        BooleanEvidenceStageKind::ReducedOperandPair => "reduced-operand-pair",
        BooleanEvidenceStageKind::EventExtractionRequest => "event-extraction-request",
        BooleanEvidenceStageKind::SegmentPairEnumeration => "segment-pair-enumeration",
        BooleanEvidenceStageKind::EventLedger => "event-ledger",
        BooleanEvidenceStageKind::Split => "split",
        BooleanEvidenceStageKind::LoopReconstruction => "loop-reconstruction",
        BooleanEvidenceStageKind::Classify => "classify",
        BooleanEvidenceStageKind::Assemble => "assemble",
        BooleanEvidenceStageKind::Cleanup => "cleanup",
    }
}
