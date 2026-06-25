use schema::facade::platform::relations::TopologyRelationKind;

use super::family::{
    DerivedTopologyConsumedGraphFacts, DerivedTopologyDiagnosticPosture,
    DerivedTopologyInvalidationPredicate, DerivedTopologyLegalityReceiptPosture,
    DerivedTopologyProductFamilyIdentity, DerivedTopologyProductFamilyRecord,
    DerivedTopologyProductFamilyRecordInput, DerivedTopologyQueryReceiptPosture,
    DerivedTopologySpatialEvidencePosture, DerivedTopologySupportPosture,
    DerivedTopologyUpdatePosture,
};
use super::{DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogError};
use crate::derived_topology::invalidation_plan::inventory::DerivedInvalidationPhaseTwoSeed;
use crate::topology_operators::TopologyTouchedAspect;

pub fn current_derived_invalidation_family_catalog(
    phase_two_seed: DerivedInvalidationPhaseTwoSeed,
) -> Result<DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogError> {
    let families = vec![
        family(
            DerivedTopologyProductFamilyIdentity::MaterializedGraph,
            TopologyRelationKind::ALL.to_vec(),
            vec![
                TopologyTouchedAspect::TopologyStructure,
                TopologyTouchedAspect::TopologyOwnership,
                TopologyTouchedAspect::TopologyBoundary,
                TopologyTouchedAspect::TopologyRadial,
                TopologyTouchedAspect::NamingPersistentName,
            ],
            DerivedTopologyUpdatePosture::BoundedRebuildRequired,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::TraversalViews,
            vec![
                TopologyRelationKind::HalfEdgeNext,
                TopologyRelationKind::HalfEdgePrev,
                TopologyRelationKind::HalfEdgeRadialNext,
                TopologyRelationKind::LoopOwnsHalfEdge,
                TopologyRelationKind::WireOwnsHalfEdge,
                TopologyRelationKind::FaceOuterLoop,
                TopologyRelationKind::FaceInnerLoop,
            ],
            vec![
                TopologyTouchedAspect::TopologyStructure,
                TopologyTouchedAspect::TopologyBoundary,
                TopologyTouchedAspect::TopologyRadial,
            ],
            DerivedTopologyUpdatePosture::BoundedRebuildRequired,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::LoopCycles,
            vec![
                TopologyRelationKind::HalfEdgeNext,
                TopologyRelationKind::HalfEdgePrev,
                TopologyRelationKind::LoopOwnsHalfEdge,
                TopologyRelationKind::FaceOuterLoop,
                TopologyRelationKind::FaceInnerLoop,
            ],
            vec![TopologyTouchedAspect::TopologyBoundary],
            DerivedTopologyUpdatePosture::IncrementalEligible,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::RadialRings,
            vec![
                TopologyRelationKind::HalfEdgeRadialNext,
                TopologyRelationKind::HalfEdgeUsesEdge,
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                TopologyRelationKind::HalfEdgeEndsAtVertex,
            ],
            vec![TopologyTouchedAspect::TopologyRadial],
            DerivedTopologyUpdatePosture::IncrementalEligible,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::ShellViews,
            vec![
                TopologyRelationKind::ShellOwnsFace,
                TopologyRelationKind::FaceOuterLoop,
                TopologyRelationKind::FaceInnerLoop,
                TopologyRelationKind::LoopOwnsHalfEdge,
                TopologyRelationKind::HalfEdgeRadialNext,
            ],
            vec![
                TopologyTouchedAspect::TopologyOwnership,
                TopologyTouchedAspect::TopologyBoundary,
                TopologyTouchedAspect::TopologyRadial,
            ],
            DerivedTopologyUpdatePosture::BoundedRebuildRequired,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::VertexDisks,
            vec![
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                TopologyRelationKind::HalfEdgeRadialNext,
                TopologyRelationKind::HalfEdgeNext,
                TopologyRelationKind::HalfEdgePrev,
            ],
            vec![
                TopologyTouchedAspect::TopologyStructure,
                TopologyTouchedAspect::TopologyRadial,
            ],
            DerivedTopologyUpdatePosture::IncrementalEligible,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
        family(
            DerivedTopologyProductFamilyIdentity::WireViews,
            vec![
                TopologyRelationKind::WireOwnsHalfEdge,
                TopologyRelationKind::HalfEdgeNext,
                TopologyRelationKind::HalfEdgePrev,
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                TopologyRelationKind::HalfEdgeEndsAtVertex,
            ],
            vec![
                TopologyTouchedAspect::TopologyBoundary,
                TopologyTouchedAspect::TopologyStructure,
            ],
            DerivedTopologyUpdatePosture::IncrementalEligible,
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        )?,
    ];
    Ok(DerivedInvalidationFamilyCatalog::new(
        phase_two_seed,
        families,
    ))
}

fn family(
    identity: DerivedTopologyProductFamilyIdentity,
    relation_kinds: Vec<TopologyRelationKind>,
    aspects: Vec<TopologyTouchedAspect>,
    update_posture: DerivedTopologyUpdatePosture,
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
) -> Result<DerivedTopologyProductFamilyRecord, DerivedInvalidationFamilyCatalogError> {
    DerivedTopologyProductFamilyRecord::from_input(DerivedTopologyProductFamilyRecordInput {
        identity,
        consumed_graph_facts: Some(DerivedTopologyConsumedGraphFacts::new(
            relation_kinds,
            aspects,
        )),
        invalidation_predicate: Some(
            DerivedTopologyInvalidationPredicate::ConsumedGraphFactsIntersectTouchedClosure,
        ),
        update_posture: Some(update_posture),
        spatial_evidence_posture: Some(
            DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed,
        ),
        query_receipt_posture: Some(query_receipt_posture),
        legality_receipt_posture: Some(
            DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
        ),
        diagnostic_posture: Some(DerivedTopologyDiagnosticPosture::ProductFamilyWitnessRequired),
        support_posture: Some(DerivedTopologySupportPosture::QuerySupportRequired),
    })
}
