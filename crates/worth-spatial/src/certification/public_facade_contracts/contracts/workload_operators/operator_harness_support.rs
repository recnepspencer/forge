use worth_kernel::workload_composition::WorkloadCatalog;
use worth_spatial::facade::projected_overlap_faces::CoplanarOverlapExtractionBundle;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    CoplanarOverlapOperatorDenial, CoplanarOverlapWorkloadOperator,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageLinkSet,
};

use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::certify_projected_storm_extraction_bundle;

pub(super) fn assert_operator_denial(
    stage_evidence_rows: Vec<WorkloadEvidenceRow>,
    extraction_bundle: &CoplanarOverlapExtractionBundle,
    expected: CoplanarOverlapOperatorDenial,
    human_reason_fragment: &str,
) {
    let stage_links = stage_links_from_rows(stage_evidence_rows);
    let denial = CoplanarOverlapWorkloadOperator::from_stage_links(&stage_links)
        .with_extraction_bundle(extraction_bundle)
        .execute()
        .expect_err("operator must deny invalid stage evidence");

    assert_eq!(denial, expected);
    assert!(denial.human_reason().contains(human_reason_fragment));
    assert!(!denial.human_reason().contains('_'));
    assert!(!denial.human_reason().contains(".operator."));
}

pub(super) fn assert_stage_link_denial(
    stage_evidence_rows: Vec<WorkloadEvidenceRow>,
    expected: WorkloadEvidenceLedgerError,
) {
    let denial = WorkloadEvidenceLedger::from_rows(stage_evidence_rows)
        .expect("operator denial rows should form an inspectable ledger")
        .stage_index()
        .link_required_stages(&required_operator_stages())
        .expect_err("invalid operator stage evidence must not link");

    assert_eq!(denial, expected);
    assert!(!denial.human_reason().contains('_'));
}

pub(super) fn stage_links_from_rows(
    stage_evidence_rows: Vec<WorkloadEvidenceRow>,
) -> WorkloadEvidenceStageLinkSet {
    WorkloadEvidenceLedger::from_rows(stage_evidence_rows)
        .expect("operator test rows should form an inspectable ledger")
        .stage_index()
        .link_required_stages(&required_operator_stages())
        .expect("operator test rows should link required stages")
}

pub(super) fn operator_extraction_bundle(world: &'static str) -> CoplanarOverlapExtractionBundle {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .declared(format!("operator harness projected overlap bundle {world}"))
        .build()
        .expect("operator harness projected overlap workload should build");
    certify_projected_storm_extraction_bundle(
        world,
        built.projected_workload(),
        built.transform_receipts(),
    )
}

fn required_operator_stages() -> [WorkloadEvidenceStage; 3] {
    [
        WorkloadEvidenceStage::Projection,
        WorkloadEvidenceStage::Transform,
        WorkloadEvidenceStage::RetainedReplay,
    ]
}
