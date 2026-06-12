use topology::facade::NmtTopologyPosture;
use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailCase;
use worth_spatial::facade::nmt_radial_fan::{
    NmtRadialFanDenial, NmtRadialFanOutcomeKind, NmtRadialFanOutcomeMatrix, NmtRadialFanOutcomeRow,
    NmtRadialFanReceipt, NmtRadialFanWorkload,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateCoincidencePolicy,
};
use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportWorkload, UnsupportedSurfaceSupportReasonCode,
};
use worth_spatial::facade::transform_workload::{
    TransformSequence, TransformWorkload, UnsupportedTransformReasonCode,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::super::dirty_planar_clean_fail::subject::dirty_clean_fail_with_topology_seed;
use crate::public_api_planar_predicate::proof_fixture::{admitted_handle, orient_basis};

pub(crate) struct NmtRadialFanSubject {
    pub catalog: BuiltWorkloadCatalogRecipe,
    pub receipt: NmtRadialFanReceipt,
    pub user_outcome: WorthUserOutcome,
}

pub(crate) fn radial_fan_subject(stem: &str, incident_faces: usize) -> NmtRadialFanSubject {
    let catalog = WorkloadCatalog::open_shell_nmt_edge_fan(incident_faces)
        .declared(format!("{stem} open radial fan workload"))
        .build()
        .expect("open radial fan workload must build from catalog");
    let topology = catalog
        .topology_construction()
        .expect("open radial fan catalog must expose topology construction receipt");
    assert_eq!(
        topology.topology_posture().posture(),
        NmtTopologyPosture::OpenNonManifold
    );

    let receipt = NmtRadialFanWorkload::from_platform_evidence(
        topology,
        catalog.workload().evidence_ledger(),
        catalog.projected_workload(),
        catalog.transform_receipts(),
        catalog
            .replay_receipts()
            .expect("open radial fan catalog must expose retained replay receipts"),
    )
    .certify()
    .expect("open radial fan workload must certify from real catalog receipts");
    let user_outcome = respond(WorthUserResponseSource::from_nmt_radial_fan(&receipt), stem);

    NmtRadialFanSubject {
        catalog,
        receipt,
        user_outcome,
    }
}

pub(crate) fn radial_fan_outcome_matrix(stem: &str) -> NmtRadialFanOutcomeMatrix {
    let subject = radial_fan_subject(stem, 4);
    let unsupported = unsupported_non_plane_surface_denial(stem);
    let dirty = dirty_topology_boundary_outcome(stem);
    let predicate = predicate_uncertain_authority_outcome(stem);
    let rows = vec![
        NmtRadialFanOutcomeRow::admitted(&subject.receipt),
        NmtRadialFanOutcomeRow::from_denial(
            &NmtRadialFanDenial::closed_manifold_laundering_attempt(
                subject.receipt.retained_replay_identity(),
            ),
        ),
        NmtRadialFanOutcomeRow::from_denial(&NmtRadialFanDenial::LabelOnlyMotion),
        NmtRadialFanOutcomeRow::from_denial(&unsupported.0),
        NmtRadialFanOutcomeRow::from_user_outcome(NmtRadialFanOutcomeKind::DirtyInput, &dirty),
        NmtRadialFanOutcomeRow::from_user_outcome(
            NmtRadialFanOutcomeKind::PredicateUncertain,
            &predicate,
        ),
        NmtRadialFanOutcomeRow::from_denial(&NmtRadialFanDenial::MissingOpenBoundaryEvidence),
    ];
    NmtRadialFanOutcomeMatrix::from_rows(rows).expect("complete NMT fan matrix")
}

pub(crate) fn unsupported_non_plane_surface_denial(
    stem: &str,
) -> (NmtRadialFanDenial, UnsupportedSurfaceSupportReasonCode) {
    let fan = radial_fan_subject(stem, 4);
    let unsupported =
        SurfaceSupportWorkload::for_bound_geometry(fan.catalog.bound_geometry().clone())
            .declared("mb-m6-nmt-1 unsupported analytic non-planar fan support")
            .with_surface_family(SurfaceFamily::AnalyticNonPlanar)
            .certify()
            .expect_err("analytic non-planar support must deny before planar overlap");
    let family = unsupported
        .family()
        .map(|family| family.human_label())
        .unwrap_or("unknown surface family");
    (
        NmtRadialFanDenial::unsupported_surface_family(family),
        unsupported.reason_code(),
    )
}

pub(crate) fn storm_checkpoint_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-foreign-fan", 4);
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-1-foreign-storm")
        .build()
        .expect("storm workload must build for foreign checkpoint test");
    fan.receipt
        .require_matching_retained_replay(
            storm
                .replay_receipts()
                .expect("storm must expose retained replay receipts"),
        )
        .expect_err("storm retained checkpoint must not satisfy NMT fan authority")
}

pub(crate) fn cube_checkpoint_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-cube-fan", 4);
    let cube = WorkloadCatalog::cube()
        .with_retained_replay_artifacts()
        .declared("mb-m6-nmt-1-foreign-cube")
        .build()
        .expect("cube workload must build for foreign checkpoint test");
    fan.receipt
        .require_matching_retained_replay(
            cube.replay_receipts()
                .expect("cube must expose retained replay receipts for this attack"),
        )
        .expect_err("cube retained checkpoint must not satisfy NMT fan authority")
}

pub(crate) fn mismatched_topology_ledger_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-topology-fan", 4);
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-1-topology-storm")
        .build()
        .expect("storm workload must build for topology mismatch test");
    NmtRadialFanWorkload::from_platform_evidence(
        fan.catalog.topology_construction().expect("fan topology"),
        storm.workload().evidence_ledger(),
        fan.catalog.projected_workload(),
        fan.catalog.transform_receipts(),
        fan.catalog.replay_receipts().expect("fan replay"),
    )
    .certify()
    .expect_err("foreign ledger topology row must not satisfy fan construction")
}

pub(crate) fn mismatched_projection_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-projection-fan", 4);
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-1-projection-storm")
        .build()
        .expect("storm workload must build for projection mismatch test");
    NmtRadialFanWorkload::from_platform_evidence(
        fan.catalog.topology_construction().expect("fan topology"),
        fan.catalog.workload().evidence_ledger(),
        storm.projected_workload(),
        fan.catalog.transform_receipts(),
        fan.catalog.replay_receipts().expect("fan replay"),
    )
    .certify()
    .expect_err("foreign projection receipt must not satisfy fan ledger")
}

pub(crate) fn mismatched_transform_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-transform-fan", 4);
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-1-transform-storm")
        .build()
        .expect("storm workload must build for transform mismatch test");
    NmtRadialFanWorkload::from_platform_evidence(
        fan.catalog.topology_construction().expect("fan topology"),
        fan.catalog.workload().evidence_ledger(),
        fan.catalog.projected_workload(),
        storm.transform_receipts(),
        fan.catalog.replay_receipts().expect("fan replay"),
    )
    .certify()
    .expect_err("foreign transform receipt must not satisfy fan ledger")
}

pub(crate) fn mismatched_replay_denial() -> NmtRadialFanDenial {
    let fan = radial_fan_subject("mb-m6-nmt-1-replay-fan", 4);
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-1-replay-storm")
        .build()
        .expect("storm workload must build for replay mismatch test");
    NmtRadialFanWorkload::from_platform_evidence(
        fan.catalog.topology_construction().expect("fan topology"),
        fan.catalog.workload().evidence_ledger(),
        fan.catalog.projected_workload(),
        fan.catalog.transform_receipts(),
        storm.replay_receipts().expect("storm replay"),
    )
    .certify()
    .expect_err("foreign replay receipt must not satisfy fan ledger")
}

pub(crate) fn manual_stage_substitution_errors() -> Vec<WorkloadEvidenceLedgerError> {
    let fan = radial_fan_subject("mb-m6-nmt-1-manual-authority", 4);
    WorkloadEvidenceStage::AUTHORITY_STAGES
        .into_iter()
        .map(|stage| {
            let rows = fan
                .catalog
                .workload()
                .evidence_ledger()
                .rows()
                .iter()
                .map(|row| {
                    if row.stage() == stage {
                        WorkloadEvidenceRow::new(stage, row.evidence_identity())
                    } else {
                        row.clone()
                    }
                })
                .collect();
            WorkloadEvidenceLedger::from_rows(rows)
                .expect("substituted ledger shape remains valid")
                .certify_complete()
                .expect_err("manual authority row must not certify complete")
        })
        .collect()
}

pub(crate) fn label_only_motion_denial() -> (UnsupportedTransformReasonCode, WorthUserOutcome) {
    let fan = radial_fan_subject("mb-m6-nmt-1-label-only", 3);
    let transform_error =
        TransformWorkload::for_projected_workload(fan.catalog.projected_workload().clone())
            .declared("mb-m6-nmt-1 label-only transform attack")
            .with_transform_sequence(TransformSequence::identity_label_only(
                "pretend radial fan moved",
            ))
            .transform()
            .expect_err("label-only transform must deny before NMT fan certification");
    let denial = NmtRadialFanWorkload::denied_transform_from_platform_evidence(
        fan.catalog.topology_construction().expect("fan topology"),
        fan.catalog.workload().evidence_ledger(),
        fan.catalog.projected_workload(),
        &transform_error,
    )
    .expect("label-only transform denial must pass through NMT fan authority");
    let outcome = respond(
        WorthUserResponseSource::from_nmt_radial_fan_denial(&denial),
        "mb-m6-nmt-1-label-only-response",
    );
    (transform_error.reason_code(), outcome)
}

pub(crate) fn missing_radial_evidence_outcome() -> WorthUserOutcome {
    respond(
        WorthUserResponseSource::from_nmt_radial_fan_denial(
            &NmtRadialFanDenial::MissingRadialAdjacencyEvidence,
        ),
        "mb-m6-nmt-1-missing-radial-response",
    )
}

pub(crate) fn missing_open_boundary_evidence_outcome() -> WorthUserOutcome {
    respond(
        WorthUserResponseSource::from_nmt_radial_fan_denial(
            &NmtRadialFanDenial::MissingOpenBoundaryEvidence,
        ),
        "mb-m6-nmt-1-missing-open-boundary-response",
    )
}

fn dirty_topology_boundary_outcome(_stem: &str) -> WorthUserOutcome {
    dirty_clean_fail_with_topology_seed(
        "mb-m6-nmt-1-dirty-non-manifold-wire",
        DirtyPlanarCleanFailCase::NonManifoldWire,
    )
    .user_outcome
}

fn predicate_uncertain_authority_outcome(_stem: &str) -> WorthUserOutcome {
    let handle = admitted_handle("mb-m6-nmt-1-predicate-authority");
    let basis = orient_basis(
        "movement:nmt-radial-fan-predicate-pressure",
        [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]],
    )
    .with_coincidence_policy(PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("certified zero must require policy or repair before NMT fan classification");
    respond(
        WorthUserResponseSource::from_predicate_authority_error(&error),
        "mb-m6-nmt-1-predicate-authority-response",
    )
}

fn respond(source: WorthUserResponseSource, declaration: &str) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(source)
        .declared(declaration)
        .respond()
        .expect("NMT fan response must produce a receipt")
        .outcome()
        .clone()
}
