use std::num::NonZeroU32;

use super::{
    PhysicalWorkArtifactBinding, PhysicalWorkHostileArtifactEvidence,
    PhysicalWorkHostileProcessEvidence, PhysicalWorkHostileTruthCampaignEvidence,
    PhysicalWorkHostileTruthEvidenceDenial, PhysicalWorkHostileTruthFinding,
    PhysicalWorkHostileTruthScenario, PhysicalWorkProcessEvidence,
};

mod fixture;
use fixture::{
    accepted_case, accepted_case_with_binding, digest, environment, finish_case,
    finish_case_with_environment, finish_case_with_process_ordinal, killed_mutant,
};

#[test]
fn exact_five_case_campaign_is_accepted() {
    let cases = PhysicalWorkHostileTruthScenario::ALL
        .into_iter()
        .enumerate()
        .map(|(index, scenario)| accepted_case(scenario, index as u8 + 1))
        .collect::<Vec<_>>();
    assert!(cases.iter().all(|case| case.verdict().accepted()));
    let campaign = PhysicalWorkHostileTruthCampaignEvidence::new(cases, [killed_mutant()]);
    assert!(campaign.verdict().accepted());
}

#[test]
fn effect_started_crash_without_recovery_obligation_is_rejected() {
    let scenario = PhysicalWorkHostileTruthScenario::DuringShortWrite;
    let case = finish_case(scenario, 1, false, true, 61);
    assert_eq!(
        case.verdict().findings(),
        &[PhysicalWorkHostileTruthFinding::MissingRecoveryObligation]
    );
}

#[test]
fn pre_dispatch_death_accepts_clean_reopen_and_rejects_invented_recovery() {
    let scenario = PhysicalWorkHostileTruthScenario::BeforeBackendDispatch;
    assert!(accepted_case(scenario, 1).verdict().accepted());
    let invented = finish_case(scenario, 1, true, true, 61);
    for finding in [
        PhysicalWorkHostileTruthFinding::UnexpectedRecoveryObligation,
        PhysicalWorkHostileTruthFinding::ReopenRecoveryMismatch,
    ] {
        assert!(invented.verdict().findings().contains(&finding));
    }
}

#[test]
fn artifact_role_must_agree_with_its_durable_path() {
    let binding =
        PhysicalWorkArtifactBinding::new("families/physical-work/work.pending", 1, digest(51))
            .unwrap();
    assert_eq!(
        PhysicalWorkHostileArtifactEvidence::new(binding, [1], false),
        Err(PhysicalWorkHostileTruthEvidenceDenial::ArtifactRoleMismatch)
    );
}

#[test]
fn publication_must_advance_exact_current_truth() {
    let case = finish_case(
        PhysicalWorkHostileTruthScenario::DuringRootPublication,
        1,
        true,
        false,
        61,
    );
    assert!(case
        .verdict()
        .findings()
        .contains(&PhysicalWorkHostileTruthFinding::InvalidScenarioTransition));
}

#[test]
fn campaign_rejects_missing_scenario_and_duplicate_store() {
    let first = accepted_case(PhysicalWorkHostileTruthScenario::BeforeBackendDispatch, 1);
    let duplicate = accepted_case(PhysicalWorkHostileTruthScenario::DuringShortWrite, 1);
    let campaign =
        PhysicalWorkHostileTruthCampaignEvidence::new([first, duplicate], [killed_mutant()]);
    for finding in [
        PhysicalWorkHostileTruthFinding::MissingScenario,
        PhysicalWorkHostileTruthFinding::DuplicateStoreIdentity,
    ] {
        assert!(campaign.verdict().findings().contains(&finding));
    }
}

#[test]
fn campaign_accepts_recycled_process_numbers_across_isolated_cases() {
    let cases = PhysicalWorkHostileTruthScenario::ALL
        .into_iter()
        .enumerate()
        .map(|(index, scenario)| {
            finish_case_with_process_ordinal(
                scenario,
                index as u8 + 1,
                1,
                scenario.requires_recovery_obligation(),
                true,
                61,
            )
        })
        .collect::<Vec<_>>();
    let campaign = PhysicalWorkHostileTruthCampaignEvidence::new(cases, [killed_mutant()]);
    assert!(campaign.verdict().accepted());
}

#[test]
fn hostile_process_roles_reject_a_reused_pid_within_one_case() {
    let process = NonZeroU32::new(11).unwrap();
    assert_eq!(
        PhysicalWorkHostileProcessEvidence::new(
            PhysicalWorkProcessEvidence::exited_success("seed-writer", process).unwrap(),
            PhysicalWorkProcessEvidence::exited_success("baseline-observer", process).unwrap(),
            PhysicalWorkProcessEvidence::killed_at_yieldpoint(
                "faulting-writer",
                NonZeroU32::new(13).unwrap(),
                "test-checkpoint",
            )
            .unwrap(),
            PhysicalWorkProcessEvidence::exited_success(
                "post-kill-observer",
                NonZeroU32::new(14).unwrap(),
            )
            .unwrap(),
            PhysicalWorkProcessEvidence::exited_success(
                "fresh-reopener",
                NonZeroU32::new(15).unwrap(),
            )
            .unwrap(),
        ),
        Err(PhysicalWorkHostileTruthEvidenceDenial::DuplicateProcessIdentity),
    );
}

#[test]
fn campaign_rejects_mixed_source_and_binary_bindings() {
    let first = accepted_case_with_binding(
        PhysicalWorkHostileTruthScenario::BeforeBackendDispatch,
        1,
        61,
    );
    let mixed =
        accepted_case_with_binding(PhysicalWorkHostileTruthScenario::DuringShortWrite, 2, 71);
    let campaign = PhysicalWorkHostileTruthCampaignEvidence::new([first, mixed], [killed_mutant()]);
    for finding in [
        PhysicalWorkHostileTruthFinding::MixedSourceBinding,
        PhysicalWorkHostileTruthFinding::MixedBinaryBinding,
    ] {
        assert!(campaign.verdict().findings().contains(&finding));
    }
}

#[test]
fn campaign_rejects_cross_case_environment_drift_and_reused_filesystem_root() {
    let cases = PhysicalWorkHostileTruthScenario::ALL
        .into_iter()
        .enumerate()
        .map(|(index, scenario)| {
            let ordinal = index as u8 + 1;
            let (root, volume, feature) = if index == 4 {
                (1, 9, 9)
            } else {
                (ordinal, 2, 1)
            };
            finish_case_with_environment(
                scenario,
                ordinal,
                ordinal,
                scenario.requires_recovery_obligation(),
                true,
                61,
                environment(root, volume, feature),
            )
        })
        .collect::<Vec<_>>();
    let campaign = PhysicalWorkHostileTruthCampaignEvidence::new(cases, [killed_mutant()]);
    for finding in [
        PhysicalWorkHostileTruthFinding::MixedRunEnvironment,
        PhysicalWorkHostileTruthFinding::MixedFilesystemVolumeProfile,
        PhysicalWorkHostileTruthFinding::DuplicateFilesystemRootIdentity,
    ] {
        assert!(campaign.verdict().findings().contains(&finding));
    }
}
