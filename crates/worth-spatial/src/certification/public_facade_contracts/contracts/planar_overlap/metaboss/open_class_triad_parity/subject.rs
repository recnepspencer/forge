use topology::facade::{
    NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyScopeKind, NmtTopologyScopeSet,
};
use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::nmt_certification_context::{
    NmtBossOutcomeMatrixEvidence, NmtCertifiedScopeContext, NmtCertifiedScopeSet,
};
use worth_spatial::facade::open_class_triad_parity::{
    OpenClassLaneAuthorityEvidence, OpenClassParityLaneSet, OpenClassStormExtractionEvidence,
    OpenClassTriadOutcomeMatrix, OpenClassTriadOutcomeRow, OpenClassTriadParityDenial,
    OpenClassTriadParityReceipt, OpenClassTriadParityWorkload, OpenTopologyClass,
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
    pub certified_scopes: NmtCertifiedScopeSet,
    pub receipt: OpenClassTriadParityReceipt,
    pub outcome_matrix: OpenClassTriadOutcomeMatrix,
    pub user_outcome: WorthUserOutcome,
}

pub(crate) struct OpenClassTriadCloseoutEvidence {
    pub certified_scopes: NmtCertifiedScopeSet,
    pub matrix: NmtBossOutcomeMatrixEvidence,
}

pub(crate) fn open_class_triad_subject(world: &'static str) -> OpenClassTriadSubject {
    let triad = WorkloadCatalog::open_class_triad(128)
        .declared(format!("{world} catalog triad"))
        .build()
        .expect("open class triad catalog must build");
    let wire_scopes = certified_scope_set(triad.wire());
    let sheet_scopes = certified_scope_set(triad.sheet());
    let fan_scopes = certified_scope_set(triad.fan());
    let certified_scopes = NmtCertifiedScopeSet::from_certified_open_class_members(&[
        &wire_scopes,
        &sheet_scopes,
        &fan_scopes,
    ])
    .expect("open class triad certified scope set must merge member scopes");
    let receipt = OpenClassTriadParityWorkload::new()
        .declared(format!("{world} open-class triad parity"))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.wire(),
            &wire_scopes,
            ProjectionParityCatalog::OpenWire,
        ))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.sheet(),
            &sheet_scopes,
            ProjectionParityCatalog::OpenSheet,
        ))
        .with_class_lane_set(certified_lane_set(
            world,
            triad.fan(),
            &fan_scopes,
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
        certified_scopes,
        receipt,
        outcome_matrix,
        user_outcome,
    }
}

pub(crate) fn cross_class_projection_denial(
    subject: &OpenClassTriadSubject,
) -> OpenClassTriadParityDenial {
    let fan_projection = OpenClassLaneAuthorityEvidence::projection_consumed_from_lane_set(
        subject
            .receipt
            .lane_set_for(OpenTopologyClass::NmtFan)
            .expect("fan lane set"),
    )
    .expect("fan projection-consumed evidence");
    subject
        .receipt
        .attempt_projection_consumed_as_retained_evidence(
            &fan_projection,
            OpenTopologyClass::NmtFan,
        )
        .expect_err("projection-consumed evidence cannot masquerade as retained replay")
}

pub(crate) fn storm_extraction_denial(
    subject: &OpenClassTriadSubject,
    evidence: &OpenClassStormExtractionEvidence,
) -> OpenClassTriadParityDenial {
    subject
        .receipt
        .attempt_storm_extraction_bundle_link_evidence(OpenTopologyClass::Sheet, evidence)
        .expect_err("storm extraction must not link into open sheet")
}

pub(crate) fn cross_class_checkpoint_denial(
    subject: &OpenClassTriadSubject,
) -> OpenClassTriadParityDenial {
    let wire_retained = OpenClassLaneAuthorityEvidence::retained_checkpoint_from_lane_set(
        subject
            .receipt
            .lane_set_for(OpenTopologyClass::Wire)
            .expect("wire lane set"),
    )
    .expect("wire retained evidence");
    subject
        .receipt
        .attempt_cross_class_checkpoint_replay_evidence(&wire_retained, OpenTopologyClass::Sheet)
        .expect_err("wire retained checkpoint cannot satisfy sheet parity")
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

pub(crate) fn closed_storm_evidence() -> OpenClassStormExtractionEvidence {
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-3 closed storm foreign digest")
        .build()
        .expect("storm workload must build");
    OpenClassStormExtractionEvidence::from_projected_workload(storm.projected_workload())
}

pub(crate) fn topology_parity_mismatch_denial() -> OpenClassTriadParityDenial {
    let triad = WorkloadCatalog::open_class_triad(4)
        .declared("mb-m6-nmt-3 topology mismatch triad")
        .build()
        .expect("triad build");
    lane_set(
        "mb-m6-nmt-3-mismatched-topology",
        triad.wire(),
        &certified_scope_set(triad.wire()),
        ProjectionParityCatalog::OpenSheet,
    )
    .expect_err("sheet parity must not satisfy open wire topology authority")
}

pub(crate) fn open_class_triad_closeout_evidence(
    world: &'static str,
) -> OpenClassTriadCloseoutEvidence {
    let subject = open_class_triad_subject(world);
    let outcomes =
        vec![
            subject.user_outcome.clone(),
            respond(
                WorthUserResponseSource::from_open_class_triad_parity_denial(
                    &denied_upgrade_denial(&subject, ProjectionFactParityLane::Recovered),
                ),
            ),
            respond(
                WorthUserResponseSource::from_open_class_triad_parity_denial(
                    &cross_class_checkpoint_denial(&subject),
                ),
            ),
            respond(
                WorthUserResponseSource::from_open_class_triad_parity_denial(
                    &storm_extraction_denial(&subject, &closed_storm_evidence()),
                ),
            ),
            respond(
                WorthUserResponseSource::from_open_class_triad_parity_denial(&missing_lane_denial(
                    &subject,
                    ProjectionFactParityLane::Replayed,
                )),
            ),
        ];
    OpenClassTriadCloseoutEvidence {
        certified_scopes: subject.certified_scopes,
        matrix: NmtBossOutcomeMatrixEvidence::from_outcomes(outcomes),
    }
}

fn certified_lane_set(
    world: &'static str,
    catalog: &BuiltWorkloadCatalogRecipe,
    certified: &NmtCertifiedScopeSet,
    parity_catalog: ProjectionParityCatalog,
) -> OpenClassParityLaneSet {
    lane_set(world, catalog, certified, parity_catalog).expect("class-bound parity lane set")
}

fn lane_set(
    world: &'static str,
    catalog: &BuiltWorkloadCatalogRecipe,
    certified: &NmtCertifiedScopeSet,
    parity_catalog: ProjectionParityCatalog,
) -> Result<OpenClassParityLaneSet, OpenClassTriadParityDenial> {
    let topology = topology_construction(catalog);
    let scope = certified_scope_for_topology(&certified, topology);
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
    OpenClassParityLaneSet::from_certified_scope_and_parity(scope, topology, parity)
}

fn certified_scope_set(catalog: &BuiltWorkloadCatalogRecipe) -> NmtCertifiedScopeSet {
    let topology = topology_construction(catalog);
    let scopes = NmtTopologyScopeSet::from_construction(topology)
        .expect("open class topology scopes must compile");
    NmtCertifiedScopeSet::from_platform_evidence(
        topology,
        catalog.workload().evidence_ledger(),
        catalog.bound_geometry(),
        catalog.projected_workload(),
        catalog.transform_receipts(),
        catalog
            .replay_receipts()
            .expect("open class catalog must expose retained replay receipts"),
        scopes,
    )
    .compile()
    .expect("open class certified scopes must compile")
}

fn certified_scope_for_topology<'a>(
    certified: &'a NmtCertifiedScopeSet,
    topology: &NmtTopologyConstructionReceipt,
) -> &'a NmtCertifiedScopeContext {
    let kind = match topology.pattern() {
        NmtTopologyPattern::OpenWireChain(_) => NmtTopologyScopeKind::OpenWire,
        NmtTopologyPattern::OpenSheetPatch(_) => NmtTopologyScopeKind::OpenSheet,
        NmtTopologyPattern::OpenRadialFan(_) => NmtTopologyScopeKind::OpenRadialFan,
        NmtTopologyPattern::OpenLayerStack(_) => {
            panic!("open-class triad parity cannot consume open layer stack scopes");
        }
    };
    certified
        .single_scope(kind)
        .expect("open class certified scope")
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
                .attempt_cross_class_checkpoint_replay_evidence(
                    &OpenClassLaneAuthorityEvidence::retained_checkpoint_from_lane_set(
                        receipt
                            .lane_set_for(OpenTopologyClass::Wire)
                            .expect("wire lane set"),
                    )
                    .expect("wire retained evidence"),
                    OpenTopologyClass::Sheet,
                )
                .expect_err("cross-class denial"),
        ),
        OpenClassTriadOutcomeRow::from_denial(
            &receipt
                .attempt_storm_extraction_bundle_link_evidence(
                    OpenTopologyClass::Sheet,
                    &closed_storm_evidence(),
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
