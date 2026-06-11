use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract,
};

use crate::anchor_selection::SpatialAnchorSelectionDeclarationFamily;
use crate::bindings::query_native::{
    PrimitiveAnchorBindingDeclarationFamily, PrimitiveBindingDeclarationFamily,
};
use crate::bindings::query_native_geometry_inventory::GeometryPublicSurface;
use crate::bindings::query_native_planar_boolean_readiness_workload::PlanarBooleanReadinessWorkloadDeclarationFamily;
use crate::bindings::query_native_planar_clean_fail_boundary::PlanarCleanFailBoundaryDeclarationFamily;
use crate::bindings::query_native_planar_contract_bundle::PlanarContractBundleValidationDeclarationFamily;
use crate::bindings::query_native_planar_diagnostics::PlanarDiagnosticBundleDeclarationFamily;
use crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateDeclarationFamily;
use crate::bindings::query_native_planar_local_rebuild_parity::PlanarLocalRebuildParityDeclarationFamily;
use crate::bindings::query_native_planar_motion_posture::PlanarMotionPostureDeclarationFamily;
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractDeclarationFamily;
use crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationDeclarationFamily;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityDeclarationFamily;
use crate::bindings::query_native_planar_predicate_consumption::PredicateCertificateConsumptionDeclarationFamily;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DDeclarationFamily;
use crate::bindings::query_native_planar_projection_consumption::ProjectionConsumedPlanarFactsDeclarationFamily;
use crate::bindings::query_native_planar_recovery::PlanarRecoveryPostureDeclarationFamily;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DDeclarationFamily;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DDeclarationFamily;
use crate::bindings::query_native_planar_structural_identity::PlanarStructuralIdentityDeclarationFamily;
use crate::bindings::query_native_planar_topology_contract::PlanarTopologyContractCompletenessDeclarationFamily;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DDeclarationFamily;
use crate::bindings::query_native_rebinding::PrimitiveRebindingDeclarationFamily;
use crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementDeclarationFamily;
use crate::bindings::query_native_rebinding_projection_consumption::GeometryProjectionConsumptionDeclarationFamily;
use crate::bindings::query_native_retained_geometry::{
    BranchLocalGeometryInspectionDeclarationFamily, GeometryRecoveryActionDeclarationFamily,
    GeometryReplayParityDeclarationFamily, HistoricalGeometryInspectionDeclarationFamily,
};
use crate::bindings::query_native_retained_planar_facts::RetainedPlanarFactsDeclarationFamily;
use crate::bindings::query_native_tolerance_precision::ToleranceAndPrecisionCertificationDeclarationFamily;

pub(super) fn declared_family_key_for(surface: GeometryPublicSurface) -> Option<&'static str> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarPredicateAuthority => {
            Some(PlanarPredicateAuthorityDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarPrecisionCertification => {
            Some(PlanarPrecisionCertificationDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarLocalFrameCertificate => {
            Some(PlanarLocalFrameCertificateDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::ProjectPointToCertifiedPlane2D => {
            Some(ProjectPointToCertifiedPlane2DDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::CertifiedSegmentSegment2D => {
            Some(CertifiedSegmentSegment2DDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::CertifiedPolygonWinding2D => {
            Some(CertifiedPolygonWinding2DDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::CertifiedSignedArea2D => {
            Some(CertifiedSignedArea2DDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::CoplanarOverlapContractExtractor => {
            Some(CoplanarOverlapContractDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarContractBundleValidator => {
            Some(PlanarContractBundleValidationDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PredicateCertificateConsumptionValidator => {
            Some(PredicateCertificateConsumptionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarStructuralIdentity => {
            Some(PlanarStructuralIdentityDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarMotionPosture => {
            Some(PlanarMotionPostureDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarTopologyContractCompleteness => {
            Some(PlanarTopologyContractCompletenessDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::RetainedPlanarFacts => {
            Some(RetainedPlanarFactsDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::ProjectionConsumedPlanarFacts => {
            Some(ProjectionConsumedPlanarFactsDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarRecoveryPosture => {
            Some(PlanarRecoveryPostureDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarDiagnosticBundle => {
            Some(PlanarDiagnosticBundleDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarLocalRebuildParity => {
            Some(PlanarLocalRebuildParityDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarCleanFailBoundary => {
            Some(PlanarCleanFailBoundaryDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PlanarBooleanReadinessWorkload => {
            Some(PlanarBooleanReadinessWorkloadDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::semantic_family_key())
        }
    }
}

pub(super) fn legality_contract_for(
    surface: GeometryPublicSurface,
) -> Option<ForgeQueryDeclarationLegalityContract> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarPredicateAuthority => {
            Some(PlanarPredicateAuthorityDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarPrecisionCertification => {
            Some(PlanarPrecisionCertificationDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarLocalFrameCertificate => {
            Some(PlanarLocalFrameCertificateDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::ProjectPointToCertifiedPlane2D => {
            Some(ProjectPointToCertifiedPlane2DDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::CertifiedSegmentSegment2D => {
            Some(CertifiedSegmentSegment2DDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::CertifiedPolygonWinding2D => {
            Some(CertifiedPolygonWinding2DDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::CertifiedSignedArea2D => {
            Some(CertifiedSignedArea2DDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::CoplanarOverlapContractExtractor => {
            Some(CoplanarOverlapContractDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarContractBundleValidator => {
            Some(PlanarContractBundleValidationDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PredicateCertificateConsumptionValidator => {
            Some(PredicateCertificateConsumptionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarStructuralIdentity => {
            Some(PlanarStructuralIdentityDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarMotionPosture => {
            Some(PlanarMotionPostureDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarTopologyContractCompleteness => {
            Some(PlanarTopologyContractCompletenessDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::RetainedPlanarFacts => {
            Some(RetainedPlanarFactsDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::ProjectionConsumedPlanarFacts => {
            Some(ProjectionConsumedPlanarFactsDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarRecoveryPosture => {
            Some(PlanarRecoveryPostureDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarDiagnosticBundle => {
            Some(PlanarDiagnosticBundleDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarLocalRebuildParity => {
            Some(PlanarLocalRebuildParityDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarCleanFailBoundary => {
            Some(PlanarCleanFailBoundaryDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PlanarBooleanReadinessWorkload => {
            Some(PlanarBooleanReadinessWorkloadDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::legality_contract())
        }
    }
}

pub(super) fn route_contract_for(
    surface: GeometryPublicSurface,
) -> Option<ForgeQueryDeclarationRouteContract> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarPredicateAuthority => {
            Some(PlanarPredicateAuthorityDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarPrecisionCertification => {
            Some(PlanarPrecisionCertificationDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarLocalFrameCertificate => {
            Some(PlanarLocalFrameCertificateDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::ProjectPointToCertifiedPlane2D => {
            Some(ProjectPointToCertifiedPlane2DDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::CertifiedSegmentSegment2D => {
            Some(CertifiedSegmentSegment2DDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::CertifiedPolygonWinding2D => {
            Some(CertifiedPolygonWinding2DDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::CertifiedSignedArea2D => {
            Some(CertifiedSignedArea2DDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::CoplanarOverlapContractExtractor => {
            Some(CoplanarOverlapContractDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarContractBundleValidator => {
            Some(PlanarContractBundleValidationDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PredicateCertificateConsumptionValidator => {
            Some(PredicateCertificateConsumptionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarStructuralIdentity => {
            Some(PlanarStructuralIdentityDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarMotionPosture => {
            Some(PlanarMotionPostureDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarTopologyContractCompleteness => {
            Some(PlanarTopologyContractCompletenessDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::RetainedPlanarFacts => {
            Some(RetainedPlanarFactsDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::ProjectionConsumedPlanarFacts => {
            Some(ProjectionConsumedPlanarFactsDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarRecoveryPosture => {
            Some(PlanarRecoveryPostureDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarDiagnosticBundle => {
            Some(PlanarDiagnosticBundleDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarLocalRebuildParity => {
            Some(PlanarLocalRebuildParityDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarCleanFailBoundary => {
            Some(PlanarCleanFailBoundaryDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PlanarBooleanReadinessWorkload => {
            Some(PlanarBooleanReadinessWorkloadDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::route_contract())
        }
    }
}
