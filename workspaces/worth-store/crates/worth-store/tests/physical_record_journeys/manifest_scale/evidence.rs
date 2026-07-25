use std::path::Path;

use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, ExternalPhysicalRecordLocator, PhysicalRecordId,
};
use worth_store_offline_verifier::OfflineDurableManifestWalk;

use super::super::scale_invalid_worlds::InvalidScaleWorlds;
use super::super::scenario_evidence::{
    ScenarioEvidence, ScenarioPredicate, ScenarioProcessEvidence,
};
use super::ScaleObservation;

pub(super) struct ScaleCourtroomEvidence<'world> {
    pub(super) root: &'world Path,
    pub(super) record_count: u16,
    pub(super) last: PhysicalRecordId,
    pub(super) locator: ExternalPhysicalRecordLocator,
    pub(super) walk: &'world OfflineDurableManifestWalk,
    pub(super) placement: AdmittedRecordPlacementPolicy,
    pub(super) publication_identity: u64,
    pub(super) processes: &'world [ScenarioProcessEvidence],
    pub(super) runtime_root_generation: u64,
    pub(super) observation: ScaleObservation,
    pub(super) invalid: &'world InvalidScaleWorlds,
}

pub(super) fn emit(input: ScaleCourtroomEvidence<'_>) {
    let world = format!("records-{}", input.record_count);
    let predicates = predicates(&input);
    super::super::scenario_evidence::emit(ScenarioEvidence {
        courtroom: "bounded_scale_identity_format_and_policy_courtroom",
        world: &world,
        root: input.root,
        seed: 0xC5C5_0000_0000_0001,
        action_trace: &[
            "initialize",
            "append",
            "close",
            "reopen",
            "locate",
            "scan",
            "offline-walk",
        ],
        authority_transitions: &[
            "absent-to-initialized",
            "batch-to-published-root",
            "fresh-process-readmission",
            "locator-readmission",
            "bounded-read-and-scan",
        ],
        walk: input.walk,
        placement: input.placement,
        publication_identity: Some(input.publication_identity),
        processes: input.processes,
        counters: serde_json::json!({
            "open_reads": input.observation.open_reads,
            "open_bytes": input.observation.open_bytes,
            "point_blocks": input.observation.point_blocks,
            "point_comparisons": input.observation.point_comparisons,
            "point_work": input.observation.point_work,
            "point_faults": input.observation.point_faults,
            "scan_work": input.observation.scan_work,
            "scan_blocks": input.observation.scan_blocks,
            "scan_frames": input.observation.scan_frames,
            "scan_faults": input.observation.scan_faults,
            "signal_clock_advance": input.observation.signal_clock_advance,
            "signal_invalidation_delta": input.observation.signal_invalidation_delta,
            "whole_blocks": input.observation.whole_blocks,
            "point_allocations": input.observation.point_allocations,
            "scan_allocations": input.observation.scan_allocations,
            "invalid_worlds": input.observation.invalid_worlds,
        }),
        runtime_result: serde_json::json!({
            "root_generation": input.runtime_root_generation,
            "records": input.observation.scan_records,
            "point_manifest_blocks": input.observation.point_blocks,
        }),
        oracle_result: serde_json::json!({
            "records": input.record_count,
            "payload_bytes": u64::from(input.record_count) * 100,
            "maximum_point_blocks": input.observation.whole_blocks,
            "invalid_worlds": {
                "missing_catalog_refused": true,
                "checksum_damage_refused": true,
                "stale_manifest_refused": true,
                "format_drift_refused": true,
                "unpublished_residue_excluded": true,
            },
        }),
        mutant_posture: "production-control",
        predicates: &predicates,
    });
}

fn predicates<'world>(
    input: &'world ScaleCourtroomEvidence<'world>,
) -> [ScenarioPredicate<'world>; 13] {
    [
        ScenarioPredicate::equality(
            "locator_readmitted",
            input.last.ordinal(),
            locator_record_ordinal(input.locator),
        ),
        ScenarioPredicate::equality(
            "runtime_offline_record_count",
            u64::from(input.record_count),
            input.walk.placements().len() as u64,
        ),
        ScenarioPredicate::equality(
            "scan_record_count",
            u64::from(input.record_count),
            input.observation.scan_records,
        ),
        ScenarioPredicate::equality(
            "bounded_point_path",
            true,
            input.observation.point_blocks <= input.observation.whole_blocks,
        ),
        ScenarioPredicate::equality(
            "point_allocation_contract",
            true,
            input.observation.point_allocations == 16_384,
        ),
        ScenarioPredicate::equality(
            "scan_allocation_contract",
            true,
            input.observation.scan_allocations >= input.observation.point_allocations
                && input.observation.scan_allocations < 65_536,
        ),
        ScenarioPredicate::equality(
            "successful_reads_do_not_invalidate_dependencies",
            0_u64,
            input.observation.signal_invalidation_delta,
        ),
        ScenarioPredicate::equality(
            "invalid_world_localization",
            5_u64,
            u64::from(input.observation.invalid_worlds),
        ),
        ScenarioPredicate::equality(
            "missing_catalog_refused",
            true,
            input.invalid.missing_catalog_refused,
        ),
        ScenarioPredicate::equality(
            "checksum_damage_refused",
            true,
            input.invalid.checksum_damage_refused,
        ),
        ScenarioPredicate::equality(
            "stale_manifest_refused",
            true,
            input.invalid.stale_manifest_refused,
        ),
        ScenarioPredicate::equality(
            "format_drift_refused",
            true,
            input.invalid.format_drift_refused,
        ),
        ScenarioPredicate::equality(
            "unpublished_residue_excluded",
            true,
            input.invalid.residue_excluded,
        ),
    ]
}

fn locator_record_ordinal(locator: ExternalPhysicalRecordLocator) -> u64 {
    u64::from_le_bytes(locator.encode()[32..40].try_into().unwrap())
}
