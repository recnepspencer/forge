use crate::bindings::query_native_geometry_applicability::{
    GeometryApplicabilityStatus, GeometryRuntimeConcern,
};
use crate::bindings::query_native_geometry_inventory::GeometryPublicSurface;

pub(crate) fn classify_planar_contract_surface(
    surface: GeometryPublicSurface,
    concern: GeometryRuntimeConcern,
) -> Option<(GeometryApplicabilityStatus, &'static str)> {
    use GeometryApplicabilityStatus::{DeniedForThisRuntime as Denied, RequiredNow as Required};
    use GeometryPublicSurface as Surface;
    use GeometryRuntimeConcern as Concern;

    let classification = match (surface, concern) {
        (Surface::PlanarPredicateAuthority, Concern::LowerRuntimeRouting) => (
            Required,
            "planar predicate authority is an admitted declaration family with an explicit relational route",
        ),
        (Surface::PlanarPredicateAuthority, Concern::MutationEvidence) => (
            Denied,
            "phase 2 predicate facts are receipt-bound authority facts, not mutating authored truth records with a separate mutation-evidence lane",
        ),
        (Surface::PlanarPrecisionCertification, Concern::LowerRuntimeRouting)
        | (Surface::PlanarPrecisionCertification, Concern::HistoricalInspection)
        | (Surface::PlanarPrecisionCertification, Concern::BranchLocalInspection) => (
            Required,
            "planar precision certificates are admitted retained artifacts over receipt-bound predicate precision metadata and spatial local-scale basis",
        ),
        (Surface::PlanarPrecisionCertification, Concern::RecoveryAction) => (
            Denied,
            "precision escalation synthesis recovery remains support-gated; phase 3 may inspect or deny but not synthesize missing basis truth",
        ),
        (Surface::PlanarPrecisionCertification, Concern::MutationEvidence) => (
            Denied,
            "planar precision certificates are receipt-bound certification facts rather than mutating authored truth records",
        ),
        (Surface::PlanarLocalFrameCertificate, Concern::LowerRuntimeRouting)
        | (Surface::PlanarLocalFrameCertificate, Concern::HistoricalInspection)
        | (Surface::PlanarLocalFrameCertificate, Concern::BranchLocalInspection) => (
            Required,
            "planar local-frame certificates are retained artifacts over precision basis, transform-chain identity, and movement/rotation posture",
        ),
        (Surface::PlanarLocalFrameCertificate, Concern::RecoveryAction) => (
            Denied,
            "local-frame recovery suggestions remain support-gated; phase 4 certifies or denies frame basis rather than synthesizing missing frame truth",
        ),
        (Surface::PlanarLocalFrameCertificate, Concern::MutationEvidence) => (
            Denied,
            "planar local-frame certificates are receipt-bound facts rather than mutating authored truth records",
        ),
        (Surface::ProjectPointToCertifiedPlane2D, Concern::LowerRuntimeRouting)
        | (Surface::ProjectPointToCertifiedPlane2D, Concern::HistoricalInspection)
        | (Surface::ProjectPointToCertifiedPlane2D, Concern::BranchLocalInspection) => (
            Required,
            "certified plane-to-2D projection facts are Query-backed retained artifacts over local-frame receipts",
        ),
        (Surface::ProjectPointToCertifiedPlane2D, Concern::MutationEvidence) => (
            Required,
            "phase 5 projection facts are authored truth-bearing projection facts with explicit canonical evidence",
        ),
        (Surface::ProjectPointToCertifiedPlane2D, Concern::ProjectionConsumption) => (
            Denied,
            "projection-consumed planar facts remain support-gated until later retained projection-consumption phases",
        ),
        (Surface::ProjectPointToCertifiedPlane2D, Concern::RecoveryAction) => (
            Denied,
            "certified projection recovery remains support-gated; phase 5 certifies or denies point projection basis rather than repairing points onto planes",
        ),
        (Surface::CertifiedSegmentSegment2D, Concern::LowerRuntimeRouting)
        | (Surface::CertifiedSegmentSegment2D, Concern::HistoricalInspection)
        | (Surface::CertifiedSegmentSegment2D, Concern::BranchLocalInspection) => (
            Required,
            "certified segment classification is a Query-backed retained artifact over projected endpoint receipts and exact planar predicate facts",
        ),
        (Surface::CertifiedSegmentSegment2D, Concern::MutationEvidence) => (
            Required,
            "phase 6 segment classifications are truth-bearing planar contact facts with explicit predicate and projection evidence",
        ),
        (Surface::CertifiedSegmentSegment2D, Concern::ProjectionConsumption) => (
            Denied,
            "coplanar overlap extraction and downstream projection consumption remain support-gated until later planar contract phases",
        ),
        (Surface::CertifiedSegmentSegment2D, Concern::RecoveryAction) => (
            Denied,
            "phase 6 classifies or denies segment contact basis rather than repairing topology or imprinting overlaps",
        ),
        (Surface::CertifiedPolygonWinding2D, Concern::LowerRuntimeRouting)
        | (Surface::CertifiedPolygonWinding2D, Concern::HistoricalInspection)
        | (Surface::CertifiedPolygonWinding2D, Concern::BranchLocalInspection) => (
            Required,
            "certified polygon winding is a Query-backed retained artifact over projected loop receipts, predicate facts, topology loop basis rows, and segment contact facts",
        ),
        (Surface::CertifiedPolygonWinding2D, Concern::MutationEvidence) => (
            Required,
            "phase 7 winding facts are retained truth-bearing planar loop facts with explicit predicate and segment-contact evidence",
        ),
        (Surface::CertifiedPolygonWinding2D, Concern::ProjectionConsumption) => (
            Denied,
            "overlap island extraction and boolean keep-discard projection consumption remain support-gated until later planar phases",
        ),
        (Surface::CertifiedPolygonWinding2D, Concern::RecoveryAction) => (
            Denied,
            "phase 7 certifies or denies winding and containment basis rather than repairing self-intersections or ambiguous touches",
        ),
        (Surface::CertifiedSignedArea2D, Concern::LowerRuntimeRouting)
        | (Surface::CertifiedSignedArea2D, Concern::HistoricalInspection)
        | (Surface::CertifiedSignedArea2D, Concern::BranchLocalInspection) => (
            Required,
            "certified signed area is a Query-backed retained artifact over winding receipts, projected local coordinates, and local precision scale",
        ),
        (Surface::CertifiedSignedArea2D, Concern::MutationEvidence) => (
            Required,
            "phase 8 signed-area classifications are retained truth-bearing planar facts with explicit winding and precision evidence",
        ),
        (Surface::CertifiedSignedArea2D, Concern::ProjectionConsumption) => (
            Denied,
            "boolean keep-discard and projection-consumed overlap facts remain support-gated until later planar phases",
        ),
        (Surface::CertifiedSignedArea2D, Concern::RecoveryAction) => (
            Denied,
            "phase 8 classifies area degeneracy without repairing topology, snapping vertices, or mutating loops",
        ),
        (Surface::CoplanarOverlapContractExtractor, Concern::LowerRuntimeRouting)
        | (Surface::CoplanarOverlapContractExtractor, Concern::HistoricalInspection)
        | (Surface::CoplanarOverlapContractExtractor, Concern::BranchLocalInspection) => (
            Required,
            "coplanar overlap contracts are Query-backed retained artifacts over certified planar face, segment, winding, area, and projection facts",
        ),
        (Surface::CoplanarOverlapContractExtractor, Concern::ProjectionConsumption) => (
            Required,
            "phase 9 emits retained overlap contract rows for downstream projection-consuming boolean readiness without computing a boolean",
        ),
        (Surface::CoplanarOverlapContractExtractor, Concern::MutationEvidence) => (
            Denied,
            "phase 9 overlap extraction is explicitly non-mutating and cannot imprint, split, or classify keep-discard topology",
        ),
        (Surface::CoplanarOverlapContractExtractor, Concern::RecoveryAction) => (
            Denied,
            "phase 9 emits typed policy-required exits rather than repairing ambiguous coplanar intent",
        ),
        (Surface::PlanarContractBundleValidator, Concern::LowerRuntimeRouting)
        | (Surface::PlanarContractBundleValidator, Concern::HistoricalInspection)
        | (Surface::PlanarContractBundleValidator, Concern::BranchLocalInspection)
        | (Surface::PlanarContractBundleValidator, Concern::ProjectionConsumption)
        | (Surface::PlanarContractBundleValidator, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 10 validates complete retained and projection-consumed M6 contract bundles as Query-backed boolean-readiness receipts",
        ),
        (Surface::PlanarContractBundleValidator, Concern::MutationEvidence)
        | (Surface::PlanarContractBundleValidator, Concern::RecoveryAction) => (
            Denied,
            "bundle validation certifies or denies existing receipts and must not mutate, repair, or synthesize missing planar truth",
        ),
        (Surface::PredicateCertificateConsumptionValidator, Concern::LowerRuntimeRouting)
        | (Surface::PredicateCertificateConsumptionValidator, Concern::HistoricalInspection)
        | (Surface::PredicateCertificateConsumptionValidator, Concern::BranchLocalInspection)
        | (Surface::PredicateCertificateConsumptionValidator, Concern::ProjectionConsumption)
        | (Surface::PredicateCertificateConsumptionValidator, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 11 validates retained planar predicate-consumption evidence as Query-backed certification receipts before boolean-readiness consumption",
        ),
        (Surface::PredicateCertificateConsumptionValidator, Concern::MutationEvidence)
        | (Surface::PredicateCertificateConsumptionValidator, Concern::RecoveryAction) => (
            Denied,
            "predicate consumption validation certifies existing worth-math predicate receipt consumption and must not mutate, repair, or synthesize predicate truth",
        ),
        (Surface::PlanarStructuralIdentity, Concern::LowerRuntimeRouting)
        | (Surface::PlanarStructuralIdentity, Concern::HistoricalInspection)
        | (Surface::PlanarStructuralIdentity, Concern::BranchLocalInspection)
        | (Surface::PlanarStructuralIdentity, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 12 certifies planar structural identity as a Query-backed retained fingerprint over boolean-readiness receipts and canonical transform basis",
        ),
        (Surface::PlanarStructuralIdentity, Concern::ProjectionConsumption)
        | (Surface::PlanarStructuralIdentity, Concern::MutationEvidence)
        | (Surface::PlanarStructuralIdentity, Concern::RecoveryAction) => (
            Denied,
            "planar structural identity is an inspectable retained fingerprint and must not consume projections, mutate topology, repair input, or synthesize missing basis truth",
        ),
        (Surface::PlanarMotionPosture, Concern::LowerRuntimeRouting)
        | (Surface::PlanarMotionPosture, Concern::HistoricalInspection)
        | (Surface::PlanarMotionPosture, Concern::BranchLocalInspection)
        | (Surface::PlanarMotionPosture, Concern::SignalContinuation)
        | (Surface::PlanarMotionPosture, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 13 certifies retained planar motion posture as a Query-backed artifact with explicit signal compatibility and continuation from boolean-readiness basis",
        ),
        (Surface::PlanarMotionPosture, Concern::ProjectionConsumption) => (
            Required,
            "phase 13 exposes motion posture as a required explicit basis before downstream projection-consumed planar facts may proceed",
        ),
        (Surface::PlanarMotionPosture, Concern::MutationEvidence)
        | (Surface::PlanarMotionPosture, Concern::RecoveryAction) => (
            Denied,
            "planar motion posture classifies retained transform basis and must not mutate topology, bridge-write lower truth, or reconstruct motion from final coordinates",
        ),
        (Surface::PlanarTopologyContractCompleteness, Concern::LowerRuntimeRouting)
        | (Surface::PlanarTopologyContractCompleteness, Concern::HistoricalInspection)
        | (Surface::PlanarTopologyContractCompleteness, Concern::BranchLocalInspection)
        | (Surface::PlanarTopologyContractCompleteness, Concern::ProjectionConsumption)
        | (Surface::PlanarTopologyContractCompleteness, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 14 certifies topology-to-spatial completeness from Query-owned topology receipts and declared validation surfaces before planar fact consumption",
        ),
        (Surface::PlanarTopologyContractCompleteness, Concern::MutationEvidence)
        | (Surface::PlanarTopologyContractCompleteness, Concern::RecoveryAction) => (
            Denied,
            "topology completeness consumes typed topology facts and must not mutate topology, repair loops, or synthesize missing validation truth",
        ),
        (Surface::RetainedPlanarFacts, Concern::LowerRuntimeRouting)
        | (Surface::RetainedPlanarFacts, Concern::HistoricalInspection)
        | (Surface::RetainedPlanarFacts, Concern::BranchLocalInspection)
        | (Surface::RetainedPlanarFacts, Concern::ProjectionConsumption)
        | (Surface::RetainedPlanarFacts, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 15 freezes retained planar classifications as Query-backed replay artifacts over boolean-readiness, structural identity, movement/rotation posture, topology completeness, and retained family rows",
        ),
        (Surface::RetainedPlanarFacts, Concern::MutationEvidence)
        | (Surface::RetainedPlanarFacts, Concern::RecoveryAction) => (
            Denied,
            "retained planar facts replay frozen receipts and must not mutate topology, repair live state, synthesize missing basis truth, or patch facts from ambient caches",
        ),
        (Surface::ProjectionConsumedPlanarFacts, Concern::LowerRuntimeRouting)
        | (Surface::ProjectionConsumedPlanarFacts, Concern::HistoricalInspection)
        | (Surface::ProjectionConsumedPlanarFacts, Concern::BranchLocalInspection)
        | (Surface::ProjectionConsumedPlanarFacts, Concern::ProjectionConsumption)
        | (Surface::ProjectionConsumedPlanarFacts, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 16 makes retained planar facts downstream-consumable through Query projection-consumption receipts bound to retained source, structural identity, motion posture, topology completeness, and projection materialization",
        ),
        (Surface::ProjectionConsumedPlanarFacts, Concern::MutationEvidence)
        | (Surface::ProjectionConsumedPlanarFacts, Concern::RecoveryAction) => (
            Denied,
            "projection-consumed planar facts deliver existing retained truth and certified projection receipts; they must not mutate, repair, or reclassify planar truth",
        ),
        (Surface::PlanarRecoveryPosture, Concern::LowerRuntimeRouting)
        | (Surface::PlanarRecoveryPosture, Concern::RecoveryAction)
        | (Surface::PlanarRecoveryPosture, Concern::HistoricalInspection)
        | (Surface::PlanarRecoveryPosture, Concern::BranchLocalInspection)
        | (Surface::PlanarRecoveryPosture, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 17 certifies typed planar recovery posture as Query-backed next-step evidence without repairing, bounding, or reclassifying planar truth",
        ),
        (Surface::PlanarRecoveryPosture, Concern::MutationEvidence)
        | (Surface::PlanarRecoveryPosture, Concern::ReplayParity)
        | (Surface::PlanarRecoveryPosture, Concern::ProjectionConsumption) => (
            Denied,
            "planar recovery posture is next-step evidence and must not mutate truth, claim replay equivalence, or deliver projection-consumed facts",
        ),
        (Surface::PlanarDiagnosticBundle, Concern::LowerRuntimeRouting)
        | (Surface::PlanarDiagnosticBundle, Concern::HistoricalInspection)
        | (Surface::PlanarDiagnosticBundle, Concern::BranchLocalInspection)
        | (Surface::PlanarDiagnosticBundle, Concern::ProjectionConsumption)
        | (Surface::PlanarDiagnosticBundle, Concern::RecoveryAction)
        | (Surface::PlanarDiagnosticBundle, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 18 derives machine-checkable planar diagnostics from typed receipts, Query inspection, reference-rich causal inspection, and projection-consumption evidence",
        ),
        (Surface::PlanarDiagnosticBundle, Concern::MutationEvidence)
        | (Surface::PlanarDiagnosticBundle, Concern::ReplayParity) => (
            Denied,
            "planar diagnostics explain existing typed evidence and must not mutate truth, reopen predicate/topology decisions, or claim materialized causal archive replay",
        ),
        (Surface::PlanarLocalRebuildParity, Concern::GroupedNeighborhoodWorkflow)
        | (Surface::PlanarLocalRebuildParity, Concern::ContributionComposition)
        | (Surface::PlanarLocalRebuildParity, Concern::LowerRuntimeRouting)
        | (Surface::PlanarLocalRebuildParity, Concern::ProjectionConsumption)
        | (Surface::PlanarLocalRebuildParity, Concern::SignalContinuation)
        | (Surface::PlanarLocalRebuildParity, Concern::HistoricalInspection)
        | (Surface::PlanarLocalRebuildParity, Concern::BranchLocalInspection)
        | (Surface::PlanarLocalRebuildParity, Concern::ReplayParity)
        | (Surface::PlanarLocalRebuildParity, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 19 certifies local planar rebuild and rebinding parity through grouped neighborhoods, rebinding continuation, retained/projection-consumed facts, and diagnostics without broad search",
        ),
        (Surface::PlanarLocalRebuildParity, Concern::MutationEvidence)
        | (Surface::PlanarLocalRebuildParity, Concern::RecoveryAction) => (
            Denied,
            "local rebuild parity consumes existing planar and rebinding evidence and must not mutate topology, repair input, or synthesize planar truth",
        ),
        (Surface::PlanarCleanFailBoundary, Concern::LowerRuntimeRouting)
        | (Surface::PlanarCleanFailBoundary, Concern::RecoveryAction)
        | (Surface::PlanarCleanFailBoundary, Concern::ProjectionConsumption)
        | (Surface::PlanarCleanFailBoundary, Concern::HistoricalInspection)
        | (Surface::PlanarCleanFailBoundary, Concern::BranchLocalInspection)
        | (Surface::PlanarCleanFailBoundary, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 20 freezes dirty and unbounded/open clean-fail posture through admission, recovery, diagnostics, and projection-consumption denial without repair or bounded conversion",
        ),
        (Surface::PlanarCleanFailBoundary, Concern::GroupedNeighborhoodWorkflow)
        | (Surface::PlanarCleanFailBoundary, Concern::ContributionComposition)
        | (Surface::PlanarCleanFailBoundary, Concern::MutationEvidence)
        | (Surface::PlanarCleanFailBoundary, Concern::SignalContinuation)
        | (Surface::PlanarCleanFailBoundary, Concern::ReplayParity) => (
            Denied,
            "clean-fail boundary is a non-mutating certification surface and must not run local rebuilds, mutate topology, or claim replay parity",
        ),
        (Surface::PlanarBooleanReadinessWorkload, Concern::LowerRuntimeRouting)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::ProjectionConsumption)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::HistoricalInspection)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::BranchLocalInspection)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::ReplayParity)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::BooleanReadinessCertification) => (
            Required,
            "phase 20 final-boss readiness consumes complete platform evidence, parity, diagnostics, and response blockers as the last admitted pre-M7 Query-backed workload",
        ),
        (Surface::PlanarBooleanReadinessWorkload, Concern::GroupedNeighborhoodWorkflow)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::ContributionComposition)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::RecoveryAction)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::MutationEvidence)
        | (Surface::PlanarBooleanReadinessWorkload, Concern::SignalContinuation) => (
            Denied,
            "boolean-readiness workload certifies pre-M7 readiness or typed blockers and must not mutate topology, repair input, synthesize recovery, or execute boolean work",
        ),
        _ => return None,
    };
    Some(classification)
}
