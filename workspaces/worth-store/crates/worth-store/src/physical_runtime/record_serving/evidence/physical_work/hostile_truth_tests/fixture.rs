use std::num::{NonZeroU32, NonZeroU64};

use super::super::{
    PhysicalWorkArtifactBinding, PhysicalWorkCourtroomRunBinding, PhysicalWorkEvidenceDigest,
    PhysicalWorkExecutionContext, PhysicalWorkFeatureGraphEvidence,
    PhysicalWorkFeatureNodeEvidence, PhysicalWorkFilesystemCapabilityEvidence,
    PhysicalWorkFilesystemCapabilityObservation, PhysicalWorkFilesystemLocationEvidence,
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkFilesystemProfileParts,
    PhysicalWorkFilesystemSupportEvidence, PhysicalWorkFreshReopenEvidence,
    PhysicalWorkFreshReopenIdentity, PhysicalWorkFreshReopenPosture,
    PhysicalWorkHostileArtifactEvidence, PhysicalWorkHostileCurrentTruth,
    PhysicalWorkHostileProcessEvidence, PhysicalWorkHostileTruthCaseBinding,
    PhysicalWorkHostileTruthCaseEvidence, PhysicalWorkHostileTruthComparison,
    PhysicalWorkHostileTruthScenario, PhysicalWorkMutantBinding,
    PhysicalWorkMutantExecutionContext, PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome,
    PhysicalWorkMutantSubject, PhysicalWorkOracleEvidence, PhysicalWorkPlatformEvidence,
    PhysicalWorkProcessEvidence, PhysicalWorkRerunEvidence, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkSourceBinding,
};

pub(super) fn accepted_case(
    scenario: PhysicalWorkHostileTruthScenario,
    ordinal: u8,
) -> PhysicalWorkHostileTruthCaseEvidence {
    finish_case(
        scenario,
        ordinal,
        scenario.requires_recovery_obligation(),
        true,
        61,
    )
}

pub(super) fn accepted_case_with_binding(
    scenario: PhysicalWorkHostileTruthScenario,
    ordinal: u8,
    binding_byte: u8,
) -> PhysicalWorkHostileTruthCaseEvidence {
    finish_case(
        scenario,
        ordinal,
        scenario.requires_recovery_obligation(),
        true,
        binding_byte,
    )
}

pub(super) fn finish_case(
    scenario: PhysicalWorkHostileTruthScenario,
    ordinal: u8,
    recovery_obligation: bool,
    valid_publication: bool,
    binding_byte: u8,
) -> PhysicalWorkHostileTruthCaseEvidence {
    finish_case_with_process_ordinal(
        scenario,
        ordinal,
        ordinal,
        recovery_obligation,
        valid_publication,
        binding_byte,
    )
}

pub(super) fn finish_case_with_process_ordinal(
    scenario: PhysicalWorkHostileTruthScenario,
    ordinal: u8,
    process_ordinal: u8,
    recovery_obligation: bool,
    valid_publication: bool,
    binding_byte: u8,
) -> PhysicalWorkHostileTruthCaseEvidence {
    finish_case_with_environment(
        scenario,
        ordinal,
        process_ordinal,
        recovery_obligation,
        valid_publication,
        binding_byte,
        environment(ordinal, 2, 1),
    )
}

pub(super) fn finish_case_with_environment(
    scenario: PhysicalWorkHostileTruthScenario,
    ordinal: u8,
    process_ordinal: u8,
    recovery_obligation: bool,
    valid_publication: bool,
    binding_byte: u8,
    environment: PhysicalWorkRunEnvironmentEvidence,
) -> PhysicalWorkHostileTruthCaseEvidence {
    let processes = processes(process_ordinal);
    let reopener_process = processes.ordered_ids()[4];
    let run = PhysicalWorkCourtroomRunBinding::new(
        source("source", binding_byte),
        source("writer", binding_byte.saturating_add(1)),
        PhysicalWorkExecutionContext::new(
            ordinal.into(),
            scenario.label(),
            processes.ordered().map(Clone::clone),
        )
        .unwrap(),
        environment,
    );
    let binding = PhysicalWorkHostileTruthCaseBinding::new(
        scenario,
        run,
        source("observer", binding_byte.saturating_add(2)),
        processes,
    );
    let baseline = truth(ordinal, 1, 1, 8, 21);
    let published = truth(ordinal, 2, 2, 16, 22);
    let expected = if scenario == PhysicalWorkHostileTruthScenario::DuringRootPublication
        && valid_publication
    {
        published
    } else {
        baseline
    };
    let artifacts = artifacts(ordinal, recovery_obligation);
    let recovery_count = u64::from(recovery_obligation);
    let inspection = recovery_obligation || scenario.requires_recovery_obligation();
    let residue = inspection && !recovery_obligation;
    let records = if inspection { 0 } else { expected.records() };
    let reopen = PhysicalWorkFreshReopenEvidence::new(
        PhysicalWorkFreshReopenIdentity::new(
            reopener_process,
            expected.store(),
            9,
            expected.generation(),
            records,
        )
        .unwrap(),
        PhysicalWorkFreshReopenPosture::new(residue, false, recovery_count, inspection),
    )
    .unwrap();
    binding.finish(
        PhysicalWorkHostileTruthComparison::new(baseline, expected, expected),
        artifacts,
        reopen,
        PhysicalWorkOracleEvidence::new("independent-payload-oracle", true, digest(31)).unwrap(),
    )
}

fn processes(ordinal: u8) -> PhysicalWorkHostileProcessEvidence {
    let first = u32::from(ordinal) * 10;
    PhysicalWorkHostileProcessEvidence::new(
        PhysicalWorkProcessEvidence::exited_success(
            "seed-writer",
            NonZeroU32::new(first + 1).unwrap(),
        )
        .unwrap(),
        PhysicalWorkProcessEvidence::exited_success(
            "baseline-observer",
            NonZeroU32::new(first + 2).unwrap(),
        )
        .unwrap(),
        PhysicalWorkProcessEvidence::killed_at_yieldpoint(
            "faulting-writer",
            NonZeroU32::new(first + 3).unwrap(),
            "test-checkpoint",
        )
        .unwrap(),
        PhysicalWorkProcessEvidence::exited_success(
            "post-kill-observer",
            NonZeroU32::new(first + 4).unwrap(),
        )
        .unwrap(),
        PhysicalWorkProcessEvidence::exited_success(
            "fresh-reopener",
            NonZeroU32::new(first + 5).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

pub(super) fn environment(root: u8, volume: u8, feature: u8) -> PhysicalWorkRunEnvironmentEvidence {
    let node = PhysicalWorkFeatureNodeEvidence::new(
        "worth-store",
        [format!("certification-{feature}")],
        Vec::<String>::new(),
    )
    .unwrap();
    let feature_graph = PhysicalWorkFeatureGraphEvidence::new(["worth-store"], [node]).unwrap();
    let capabilities = PhysicalWorkFilesystemCapabilityEvidence::ALL
        .map(|capability| {
            PhysicalWorkFilesystemCapabilityObservation::new(
                capability,
                PhysicalWorkFilesystemSupportEvidence::Supported,
            )
        })
        .into();
    let filesystem =
        PhysicalWorkFilesystemProfileEvidence::from_parts(PhysicalWorkFilesystemProfileParts {
            root_identity: [root; 32],
            volume_identity: [volume; 32],
            filesystem_type: "fixture-filesystem".into(),
            allocation_granularity: NonZeroU64::MIN,
            location: PhysicalWorkFilesystemLocationEvidence::Local,
            removable: false,
            read_only: false,
            capabilities,
        })
        .unwrap();
    PhysicalWorkRunEnvironmentEvidence::new(
        feature_graph,
        PhysicalWorkPlatformEvidence::current(),
        filesystem,
        PhysicalWorkRerunEvidence::new("cargo", ["test"]).unwrap(),
    )
}

fn truth(
    store_byte: u8,
    generation: u64,
    records: u64,
    payload_bytes: u64,
    digest_byte: u8,
) -> PhysicalWorkHostileCurrentTruth {
    PhysicalWorkHostileCurrentTruth::new(
        [store_byte; 16],
        generation,
        records,
        payload_bytes,
        digest(digest_byte),
    )
    .unwrap()
}

fn artifacts(ordinal: u8, recovery: bool) -> Vec<PhysicalWorkHostileArtifactEvidence> {
    let mut artifacts = vec![
        artifact("families/records/bootstrap.catalog", ordinal, false),
        artifact("namespace/mutation.lock", ordinal.saturating_add(1), false),
    ];
    if recovery {
        artifacts.push(artifact(
            "families/physical-work/obligation.pending",
            ordinal.saturating_add(2),
            true,
        ));
    }
    artifacts
}

fn artifact(path: &str, digest_byte: u8, recovery: bool) -> PhysicalWorkHostileArtifactEvidence {
    let binding = PhysicalWorkArtifactBinding::new(path, 4, digest(digest_byte)).unwrap();
    PhysicalWorkHostileArtifactEvidence::new(binding, [digest_byte], recovery).unwrap()
}

pub(super) fn killed_mutant() -> PhysicalWorkMutantLocalization {
    let subject = PhysicalWorkMutantSubject::new(15, "effect route", "effect.rs").unwrap();
    let execution = PhysicalWorkMutantExecutionContext::new("test", "courtroom-b").unwrap();
    let binding = PhysicalWorkMutantBinding::new(
        subject,
        digest(41),
        digest(42),
        source("mutant", 43),
        execution,
    );
    PhysicalWorkMutantLocalization::new(
        binding,
        PhysicalWorkMutantOutcome::new(true, "courtroom-b:1"),
    )
    .unwrap()
}

fn source(path: &str, digest_byte: u8) -> PhysicalWorkSourceBinding {
    PhysicalWorkSourceBinding::new(path, digest(digest_byte)).unwrap()
}

pub(super) fn digest(byte: u8) -> PhysicalWorkEvidenceDigest {
    PhysicalWorkEvidenceDigest::new([byte; 32]).unwrap()
}
