use worth_kernel::workload_composition::{
    OperatorOutcome, OperatorOutcomeKind, TransformRecipe, WorkloadCatalog, WorkloadOperator,
    WorkloadOperatorFamily, WorkloadStageRequirement, WorkloadTopologyBreadth,
};
use worth_spatial::facade::coplanar_overlap_storm::{
    CoplanarOverlapStormReceipt, CoplanarOverlapStormWorkload, CoplanarOverlapStormWorkloadError,
};
use worth_spatial::facade::planar_overlap::CoplanarOverlapUserOutcome;
use worth_spatial::facade::workload_operators::{
    CoplanarOverlapOperatorReceipt, CoplanarOverlapWorkloadOperator,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::storm_extraction_subject::certify_projected_storm_extraction_bundle;

pub(crate) struct PlatformStormSubject {
    pub(crate) storm_receipt: CoplanarOverlapStormReceipt,
    pub(crate) operator_receipt: CoplanarOverlapOperatorReceipt,
    pub(crate) user_outcome: CoplanarOverlapUserOutcome,
}

pub(crate) fn certify_platform_storm(world: &'static str) -> PlatformStormSubject {
    certify_platform_storm_with_transform(
        world,
        WorkloadTopologyBreadth::Default,
        TransformRecipe::HostileCancellation,
    )
}

pub(crate) fn certify_platform_storm_with_transform(
    world: &'static str,
    topology_breadth: WorkloadTopologyBreadth,
    transform_recipe: TransformRecipe,
) -> PlatformStormSubject {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(topology_breadth)
        .with_transform(transform_recipe)
        .declared(format!("MB-M6-1 platform coplanar overlap storm {world}"))
        .build()
        .expect("platform storm workload should build");
    let workload = built.workload();
    let run = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
        .requiring(WorkloadStageRequirement::RetainedReplay)
        .declared_by_query(format!(
            "MB-M6-1 coplanar overlap operator consumes platform storm {world}"
        ))
        .admit_for(workload)
        .expect("platform storm workload should admit for coplanar overlap");
    let extraction_bundle =
        certify_projected_storm_extraction_bundle(world, built.projected_workload());
    let operator_receipt =
        CoplanarOverlapWorkloadOperator::from_consumed_evidence(run.consumed_evidence())
            .with_extraction_bundle(&extraction_bundle)
            .execute()
            .expect("platform storm operator should execute");
    let operator_outcome =
        OperatorOutcome::from_coplanar_overlap_receipt(run, operator_receipt.clone())
            .expect("kernel operator outcome should consume spatial receipt");
    assert_eq!(operator_outcome.kind(), OperatorOutcomeKind::Admitted);

    let storm_receipt = CoplanarOverlapStormWorkload::from_platform_evidence(
        workload.evidence_ledger(),
        &operator_receipt,
    )
    .certify()
    .expect("platform storm receipt should certify");
    let user_outcome = CoplanarOverlapUserOutcome::from_operator_receipt(&operator_receipt);

    PlatformStormSubject {
        storm_receipt,
        operator_receipt,
        user_outcome,
    }
}

pub(crate) fn manual_stage_substitution_error(
    stage: WorkloadEvidenceStage,
) -> Result<(), WorkloadEvidenceLedgerError> {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_transform(TransformRecipe::HostileCancellation)
        .declared(format!(
            "MB-M6-1 manual {} substitution",
            stage.human_name()
        ))
        .build()
        .expect("platform storm workload should build");
    let rows = built
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
    WorkloadEvidenceLedger::from_rows(rows)?.certify_complete()?;
    Ok(())
}

pub(crate) fn mismatched_operator_stage_link_error(
) -> Result<CoplanarOverlapStormReceipt, CoplanarOverlapStormWorkloadError> {
    let ledger_source = WorkloadCatalog::coplanar_overlap_storm()
        .with_transform(TransformRecipe::HostileCancellation)
        .declared("MB-M6-1 ledger side for mismatched operator proof")
        .build()
        .expect("ledger source workload should build");
    let operator_source = WorkloadCatalog::coplanar_overlap_storm()
        .with_transform(TransformRecipe::HostileCancellation)
        .declared("MB-M6-1 operator side for mismatched operator proof")
        .build()
        .expect("operator source workload should build");
    let operator_run = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
        .requiring(WorkloadStageRequirement::RetainedReplay)
        .declared_by_query("MB-M6-1 mismatched operator should not certify another ledger")
        .admit_for(operator_source.workload())
        .expect("operator source should admit");
    let operator_receipt =
        CoplanarOverlapWorkloadOperator::from_consumed_evidence(operator_run.consumed_evidence())
            .with_extraction_bundle(&certify_projected_storm_extraction_bundle(
                "mb-m6-1-mismatched-operator-extractions",
                operator_source.projected_workload(),
            ))
            .execute()
            .expect("operator source should execute");

    CoplanarOverlapStormWorkload::from_platform_evidence(
        ledger_source.workload().evidence_ledger(),
        &operator_receipt,
    )
    .certify()
}
