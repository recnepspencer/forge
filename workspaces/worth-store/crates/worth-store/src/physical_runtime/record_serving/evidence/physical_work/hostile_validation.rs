use std::collections::BTreeSet;

use super::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkHostileArtifactEvidence,
    PhysicalWorkHostileTruthCaseBinding, PhysicalWorkHostileTruthCaseEvidence,
    PhysicalWorkHostileTruthComparison, PhysicalWorkHostileTruthFinding,
    PhysicalWorkHostileTruthScenario, PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
};

pub(super) fn validate_case(
    binding: &PhysicalWorkHostileTruthCaseBinding,
    comparison: PhysicalWorkHostileTruthComparison,
    artifacts: &[PhysicalWorkHostileArtifactEvidence],
    reopener: PhysicalWorkFreshReopenEvidence,
    oracle: &PhysicalWorkOracleEvidence,
) -> Vec<PhysicalWorkHostileTruthFinding> {
    let mut findings = Vec::new();
    validate_processes(binding, reopener, &mut findings);
    validate_truth(binding.scenario(), comparison, &mut findings);
    validate_artifacts(binding.scenario(), artifacts, &mut findings);
    validate_reopen(
        binding.scenario(),
        comparison,
        artifacts,
        reopener,
        &mut findings,
    );
    if !oracle.accepted() {
        findings.push(PhysicalWorkHostileTruthFinding::OracleRejected);
    }
    findings
}

fn validate_processes(
    binding: &PhysicalWorkHostileTruthCaseBinding,
    reopener: PhysicalWorkFreshReopenEvidence,
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    let expected = binding.processes().ordered();
    if binding.run().execution().processes() != expected.map(Clone::clone)
        || reopener.identity().process() != expected[4].process()
    {
        findings.push(PhysicalWorkHostileTruthFinding::ProcessBindingMismatch);
    }
}

fn validate_truth(
    scenario: PhysicalWorkHostileTruthScenario,
    comparison: PhysicalWorkHostileTruthComparison,
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    let baseline = comparison.baseline();
    let expected = comparison.expected();
    let observed = comparison.observed();
    if baseline.store() != expected.store() || expected.store() != observed.store() {
        findings.push(PhysicalWorkHostileTruthFinding::StoreIdentityMismatch);
    }
    if observed != expected {
        findings.push(PhysicalWorkHostileTruthFinding::UnexpectedCurrentTruth);
    }
    let valid_transition = if scenario == PhysicalWorkHostileTruthScenario::DuringRootPublication {
        expected.generation() == baseline.generation().saturating_add(1)
            && expected.records() == baseline.records().saturating_add(1)
            && expected.payload_bytes() > baseline.payload_bytes()
    } else {
        expected == baseline
    };
    if !valid_transition {
        findings.push(PhysicalWorkHostileTruthFinding::InvalidScenarioTransition);
    }
}

fn validate_artifacts(
    scenario: PhysicalWorkHostileTruthScenario,
    artifacts: &[PhysicalWorkHostileArtifactEvidence],
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    if artifacts.is_empty() {
        findings.push(PhysicalWorkHostileTruthFinding::MissingArtifactManifest);
        return;
    }
    let mut paths = BTreeSet::new();
    if artifacts
        .iter()
        .any(|artifact| !paths.insert(artifact.binding().path()))
    {
        findings.push(PhysicalWorkHostileTruthFinding::DuplicateArtifactPath);
    }
    if !artifacts
        .iter()
        .any(PhysicalWorkHostileArtifactEvidence::is_mutation_coordination)
    {
        findings.push(PhysicalWorkHostileTruthFinding::MissingMutationCoordinationArtifact);
    }
    let recovery = artifacts
        .iter()
        .filter(|artifact| artifact.is_recovery_obligation())
        .count();
    if scenario.requires_recovery_obligation() && recovery == 0 {
        findings.push(PhysicalWorkHostileTruthFinding::MissingRecoveryObligation);
    }
    if !scenario.requires_recovery_obligation() && recovery != 0 {
        findings.push(PhysicalWorkHostileTruthFinding::UnexpectedRecoveryObligation);
    }
}

fn validate_reopen(
    scenario: PhysicalWorkHostileTruthScenario,
    comparison: PhysicalWorkHostileTruthComparison,
    artifacts: &[PhysicalWorkHostileArtifactEvidence],
    reopener: PhysicalWorkFreshReopenEvidence,
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    let identity = reopener.identity();
    let posture = reopener.posture();
    let observed = comparison.observed();
    if identity.store() != observed.store() || identity.generation() != observed.generation() {
        findings.push(PhysicalWorkHostileTruthFinding::ReopenTruthMismatch);
    }
    let recovery = artifacts
        .iter()
        .filter(|artifact| artifact.is_recovery_obligation())
        .count() as u64;
    let valid_posture = if scenario.requires_recovery_obligation() {
        posture.inspection_required()
            && posture.recovery_obligations() == recovery
            && identity.records() == 0
    } else {
        !posture.inspection_required()
            && posture.recovery_obligations() == 0
            && !posture.residue()
            && !posture.recovery_evidence_damaged()
            && identity.records() == observed.records()
    };
    if !valid_posture {
        findings.push(PhysicalWorkHostileTruthFinding::ReopenRecoveryMismatch);
    }
}

pub(super) fn validate_campaign(
    cases: &[PhysicalWorkHostileTruthCaseEvidence],
    mutants: &[PhysicalWorkMutantLocalization],
) -> Vec<PhysicalWorkHostileTruthFinding> {
    let mut findings = Vec::new();
    let scenarios = cases
        .iter()
        .map(|case| case.binding().scenario())
        .collect::<BTreeSet<_>>();
    if scenarios.len() != cases.len() {
        findings.push(PhysicalWorkHostileTruthFinding::DuplicateScenario);
    }
    if PhysicalWorkHostileTruthScenario::ALL
        .iter()
        .any(|scenario| !scenarios.contains(scenario))
    {
        findings.push(PhysicalWorkHostileTruthFinding::MissingScenario);
    }
    validate_campaign_identity(cases, &mut findings);
    if cases.iter().any(|case| !case.verdict().accepted()) {
        findings.push(PhysicalWorkHostileTruthFinding::RejectedScenario);
    }
    if mutants.is_empty() {
        findings.push(PhysicalWorkHostileTruthFinding::MissingMutantLocalization);
    } else if mutants.iter().any(|mutant| !mutant.killed()) {
        findings.push(PhysicalWorkHostileTruthFinding::MutantSurvived);
    }
    findings
}

fn validate_campaign_identity(
    cases: &[PhysicalWorkHostileTruthCaseEvidence],
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    let stores = cases
        .iter()
        .map(|case| case.comparison().observed().store())
        .collect::<BTreeSet<_>>();
    if stores.len() != cases.len() {
        findings.push(PhysicalWorkHostileTruthFinding::DuplicateStoreIdentity);
    }
    let Some(first) = cases.first() else {
        return;
    };
    if cases
        .iter()
        .any(|case| case.binding().run().source() != first.binding().run().source())
    {
        findings.push(PhysicalWorkHostileTruthFinding::MixedSourceBinding);
    }
    if cases.iter().any(|case| {
        case.binding().run().binary() != first.binding().run().binary()
            || case.binding().observer_binary() != first.binding().observer_binary()
    }) {
        findings.push(PhysicalWorkHostileTruthFinding::MixedBinaryBinding);
    }
    validate_campaign_environment(cases, first, findings);
}

fn validate_campaign_environment(
    cases: &[PhysicalWorkHostileTruthCaseEvidence],
    first: &PhysicalWorkHostileTruthCaseEvidence,
    findings: &mut Vec<PhysicalWorkHostileTruthFinding>,
) {
    let expected = first.binding().run().environment();
    if cases.iter().any(|case| {
        let environment = case.binding().run().environment();
        environment.feature_graph() != expected.feature_graph()
            || environment.platform() != expected.platform()
            || environment.rerun() != expected.rerun()
    }) {
        findings.push(PhysicalWorkHostileTruthFinding::MixedRunEnvironment);
    }
    let expected_filesystem = expected.filesystem();
    if cases.iter().any(|case| {
        !same_volume_profile(
            case.binding().run().environment().filesystem(),
            expected_filesystem,
        )
    }) {
        findings.push(PhysicalWorkHostileTruthFinding::MixedFilesystemVolumeProfile);
    }
    let roots = cases
        .iter()
        .map(|case| {
            case.binding()
                .run()
                .environment()
                .filesystem()
                .root_identity()
        })
        .collect::<BTreeSet<_>>();
    if roots.len() != cases.len() {
        findings.push(PhysicalWorkHostileTruthFinding::DuplicateFilesystemRootIdentity);
    }
}

fn same_volume_profile(
    left: &super::PhysicalWorkFilesystemProfileEvidence,
    right: &super::PhysicalWorkFilesystemProfileEvidence,
) -> bool {
    left.volume_identity() == right.volume_identity()
        && left.filesystem_type() == right.filesystem_type()
        && left.allocation_granularity() == right.allocation_granularity()
        && left.location() == right.location()
        && left.is_removable() == right.is_removable()
        && left.is_read_only() == right.is_read_only()
        && left.capabilities() == right.capabilities()
}

#[cfg(test)]
mod tests {
    use super::super::PhysicalWorkHostileTruthCampaignEvidence;

    #[test]
    fn empty_campaign_is_rejected() {
        let evidence = PhysicalWorkHostileTruthCampaignEvidence::new([], []);
        assert!(!evidence.verdict().accepted());
    }
}
