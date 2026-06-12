use topology::facade::{NmtTopologyConstructionReceipt, NmtTopologyPattern};
use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::open_class_triad_parity::{
    OpenClassParityLaneSet, OpenClassTriadOutcomeMatrix, OpenClassTriadOutcomeRow,
    OpenClassTriadParityDenial, OpenClassTriadParityReceipt, OpenClassTriadParityWorkload,
    OpenTopologyClass,
};
use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityLane, ProjectionFactParityWorkload,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

use super::super::projection_fact_parity::catalog::ProjectionParityCatalog;
use super::super::projection_fact_parity::subject::{admitted_basis, real_parity_parts};

pub(crate) struct OpenClassTriadSubject {
    pub receipt: OpenClassTriadParityReceipt,
    pub outcome_matrix: OpenClassTriadOutcomeMatrix,
    pub user_outcome: WorthUserOutcome,
}

pub(crate) fn open_class_triad_subject(world: &'static str) -> OpenClassTriadSubject {
    let triad = WorkloadCatalog::open_class_triad(128)
        .declared(format!("{world} catalog triad"))
        .build()
        .expect("open class triad catalog must build");
    let receipt = OpenClassTriadParityWorkload::new()
        .declared(format!("{world} open-class triad parity"))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.wire(),
            ProjectionParityCatalog::OpenWire,
        ))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.sheet(),
            ProjectionParityCatalog::OpenSheet,
        ))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.fan(),
            ProjectionParityCatalog::OpenRadialFan(128),
        ))
        .compare_required_lanes()
        .certify()
        .expect("open class triad must certify");
    let user_outcome = respond(WorthUserResponseSource::from_open_class_triad_parity(
        &receipt,
    ));
    let outcome_matrix = outcome_matrix(&receipt);
    let _ = triad;
    OpenClassTriadSubject {
        receipt,
        outcome_matrix,
        user_outcome,
    }
}

pub(crate) fn cross_class_projection_denial(
    subject: &OpenClassTriadSubject,
) -> OpenClassTriadParityDenial {
    subject
        .receipt
        .attempt_projection_consumed_as_retained(OpenTopologyClass::NmtFan)
        .expect_err("projection-consumed evidence cannot masquerade as retained replay")
}

pub(crate) fn storm_extraction_denial(
    subject: &OpenClassTriadSubject,
    digest: &str,
) -> OpenClassTriadParityDenial {
    subject
        .receipt
        .attempt_storm_extraction_bundle_link(OpenTopologyClass::Sheet, digest)
        .expect_err("storm extraction must not link into open sheet")
}

pub(crate) fn denied_upgrade_denial(
    subject: &OpenClassTriadSubject,
    lane: ProjectionFactParityLane,
) -> OpenClassTriadParityDenial {
    subject
        .receipt
        .attempt_denied_lane_upgrade(OpenTopologyClass::NmtFan, lane)
        .expect_err("denied parity cannot upgrade through another lane")
}

pub(crate) fn missing_lane_denial(
    subject: &OpenClassTriadSubject,
    lane: ProjectionFactParityLane,
) -> OpenClassTriadParityDenial {
    subject
        .receipt
        .attempt_missing_lane_evidence(OpenTopologyClass::Wire, lane)
        .expect_err("missing lane evidence must produce no-options")
}

pub(crate) fn closed_storm_digest() -> String {
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-3 closed storm foreign digest")
        .build()
        .expect("storm workload must build");
    storm
        .projected_workload()
        .receipts()
        .stage_identity()
        .receipt_identity()
}

pub(crate) fn topology_parity_mismatch_denial() -> OpenClassTriadParityDenial {
    let triad = WorkloadCatalog::open_class_triad(4)
        .declared("mb-m6-nmt-3 topology mismatch triad")
        .build()
        .expect("triad build");
    lane_set(
        "mb-m6-nmt-3-mismatched-topology",
        triad.wire(),
        ProjectionParityCatalog::OpenSheet,
    )
    .expect_err("sheet parity must not satisfy open wire topology authority")
}

fn certified_lane_set(
    world: &'static str,
    catalog: &BuiltWorkloadCatalogRecipe,
    parity_catalog: ProjectionParityCatalog,
) -> OpenClassParityLaneSet {
    lane_set(world, catalog, parity_catalog).expect("class-bound parity lane set")
}

fn lane_set(
    world: &'static str,
    catalog: &BuiltWorkloadCatalogRecipe,
    parity_catalog: ProjectionParityCatalog,
) -> Result<OpenClassParityLaneSet, OpenClassTriadParityDenial> {
    let topology = topology_construction(catalog);
    let mut parts = real_parity_parts(world, parity_catalog);
    if parity_catalog_matches_topology(topology, parity_catalog) {
        parts.ledger = catalog.workload().evidence_ledger().clone();
    }
    let parity = ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(&parts))
        .declared(format!(
            "MB-M6-NMT-3 {} parity {world}",
            topology.declaration()
        ))
        .compare_lanes()
        .certify()
        .expect("open class parity receipt");
    OpenClassParityLaneSet::from_topology_and_parity(topology, parity)
}

fn parity_catalog_matches_topology(
    topology: &NmtTopologyConstructionReceipt,
    parity_catalog: ProjectionParityCatalog,
) -> bool {
    matches!(
        (topology.pattern(), parity_catalog),
        (
            NmtTopologyPattern::OpenWireChain(_),
            ProjectionParityCatalog::OpenWire
        ) | (
            NmtTopologyPattern::OpenSheetPatch(_),
            ProjectionParityCatalog::OpenSheet
        ) | (
            NmtTopologyPattern::OpenRadialFan(_),
            ProjectionParityCatalog::OpenRadialFan(_)
        )
    )
}

fn topology_construction(catalog: &BuiltWorkloadCatalogRecipe) -> &NmtTopologyConstructionReceipt {
    catalog
        .topology_construction()
        .expect("open class catalog must expose topology construction")
}

fn outcome_matrix(receipt: &OpenClassTriadParityReceipt) -> OpenClassTriadOutcomeMatrix {
    let rows = vec![
        OpenClassTriadOutcomeRow::admitted(receipt),
        OpenClassTriadOutcomeRow::from_denial(
            &receipt
                .attempt_denied_lane_upgrade(
                    OpenTopologyClass::NmtFan,
                    ProjectionFactParityLane::Recovered,
                )
                .expect_err("upgrade denial"),
        ),
        OpenClassTriadOutcomeRow::from_denial(
            &receipt
                .attempt_cross_class_checkpoint_replay(
                    OpenTopologyClass::Wire,
                    OpenTopologyClass::Sheet,
                )
                .expect_err("cross-class denial"),
        ),
        OpenClassTriadOutcomeRow::from_denial(
            &receipt
                .attempt_storm_extraction_bundle_link(
                    OpenTopologyClass::Sheet,
                    &closed_storm_digest(),
                )
                .expect_err("storm unsupported"),
        ),
        OpenClassTriadOutcomeRow::from_denial(
            &receipt
                .attempt_missing_lane_evidence(
                    OpenTopologyClass::Wire,
                    ProjectionFactParityLane::Replayed,
                )
                .expect_err("missing lane"),
        ),
    ];
    OpenClassTriadOutcomeMatrix::from_rows(rows).expect("outcome matrix")
}

fn respond(source: WorthUserResponseSource) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(source)
        .declared("mb-m6-nmt-3 open triad response")
        .respond()
        .expect("open triad response")
        .outcome()
        .clone()
}
