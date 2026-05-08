use crate::certification::core::{
    CertificationBridgeExpectation, CertificationCanonicalRow, CertificationParityRow,
    CertificationRejectionRow, CertificationRequiredOutput, CertificationSuiteDefinition,
    CertificationSuiteRequirements, CertificationValidatorExpectation,
};
use crate::certification::milestone_three::{
    milestone_three_rejected_scenario_names, milestone_three_replay_scenario_names,
    milestone_three_required_scenario_names,
};
use crate::certification::shared::{
    canonical_milestone_one_primitive_families, derived_validator_expectations_for_family,
    validator_expectations_for_family,
};

pub fn milestone_one_closeout_suite_definition() -> CertificationSuiteDefinition {
    let canonical_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Smallest".to_string(),
                },
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Generic".to_string(),
                },
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "HostileAdmitted".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let rejection_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationRejectionRow {
            family: family.to_string(),
            role: "OutOfClass".to_string(),
            rejection_class: "OutOfClass".to_string(),
        })
        .collect::<Vec<_>>();
    let parity_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                CertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "replay".to_string(),
                },
                CertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "branch".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();

    CertificationSuiteDefinition {
        suite_name: ".milestone_1.closeout".to_string(),
        canonical_rows,
        rejection_rows,
        parity_rows,
        required_outputs: milestone_one_closeout_requirements().required_outputs,
    }
}

pub fn milestone_one_closeout_requirements() -> CertificationSuiteRequirements {
    let required_family_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let validator_expectations = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationValidatorExpectation {
            family: family.to_string(),
            validators: validator_expectations_for_family(family)
                .iter()
                .map(|validator| (*validator).to_string())
                .collect(),
        })
        .collect::<Vec<_>>();
    let required_bridge_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationBridgeExpectation {
            family: family.to_string(),
        })
        .collect::<Vec<_>>();

    CertificationSuiteRequirements {
        suite_name: ".milestone_1.closeout".to_string(),
        required_family_rows: required_family_rows.clone(),
        required_rejection_rows: required_family_rows,
        validator_expectations,
        required_parity_rows: canonical_milestone_one_primitive_families()
            .into_iter()
            .map(str::to_string)
            .collect(),
        required_bridge_rows,
        required_outputs: vec![
            CertificationRequiredOutput::TopologyTruthDigest,
            CertificationRequiredOutput::NamingTruthDigest,
            CertificationRequiredOutput::TopologyValidationDigest,
            CertificationRequiredOutput::TopologyValidationReport,
            CertificationRequiredOutput::TopologyLocalizationReport,
            CertificationRequiredOutput::NamingAttachmentReport,
            CertificationRequiredOutput::PrimitiveFamilyCoverageMatrix,
            CertificationRequiredOutput::PrimitiveCorpusParityReport,
            CertificationRequiredOutput::AdmittedRangeSweepReport,
            CertificationRequiredOutput::ValidatorCoverageReport,
            CertificationRequiredOutput::BranchLocalTopologyReport,
            CertificationRequiredOutput::ReplayParityReport,
            CertificationRequiredOutput::RejectionClassReport,
            CertificationRequiredOutput::FailureLocalityReport,
            CertificationRequiredOutput::BridgeFamilyCoverageReport,
            CertificationRequiredOutput::BridgeProofReport,
            CertificationRequiredOutput::CounterReport,
        ],
    }
}

pub fn milestone_two_closeout_suite_definition() -> CertificationSuiteDefinition {
    let canonical_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Smallest".to_string(),
                },
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Generic".to_string(),
                },
                CertificationCanonicalRow {
                    family: family.to_string(),
                    role: "HostileAdmitted".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let rejection_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationRejectionRow {
            family: family.to_string(),
            role: "OutOfClass".to_string(),
            rejection_class: "OutOfClass".to_string(),
        })
        .collect::<Vec<_>>();
    let parity_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                CertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "replay".to_string(),
                },
                CertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "branch".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();

    CertificationSuiteDefinition {
        suite_name: ".milestone_2.closeout".to_string(),
        canonical_rows,
        rejection_rows,
        parity_rows,
        required_outputs: milestone_two_closeout_requirements().required_outputs,
    }
}

pub fn milestone_two_closeout_requirements() -> CertificationSuiteRequirements {
    let required_family_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let validator_expectations = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationValidatorExpectation {
            family: family.to_string(),
            validators: derived_validator_expectations_for_family(family)
                .iter()
                .map(|validator| (*validator).to_string())
                .collect(),
        })
        .collect::<Vec<_>>();
    let required_bridge_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| CertificationBridgeExpectation {
            family: family.to_string(),
        })
        .collect::<Vec<_>>();

    CertificationSuiteRequirements {
        suite_name: ".milestone_2.closeout".to_string(),
        required_family_rows: required_family_rows.clone(),
        required_rejection_rows: required_family_rows,
        validator_expectations,
        required_parity_rows: canonical_milestone_one_primitive_families()
            .into_iter()
            .map(str::to_string)
            .collect(),
        required_bridge_rows,
        required_outputs: vec![
            CertificationRequiredOutput::MaterializedTopologyDigest,
            CertificationRequiredOutput::InterpretedTopologyDigest,
            CertificationRequiredOutput::DerivedValidationDigest,
            CertificationRequiredOutput::DerivedTruthBasisDigest,
            CertificationRequiredOutput::BridgeRoutingDigest,
            CertificationRequiredOutput::BridgeHistoricalEvaluationDigest,
            CertificationRequiredOutput::DerivedFamilyCoverageMatrix,
            CertificationRequiredOutput::DerivedFamilyParityMatrix,
            CertificationRequiredOutput::DerivedValidatorCoverageReport,
            CertificationRequiredOutput::DerivedInvalidationReport,
            CertificationRequiredOutput::DerivedRebuildReport,
            CertificationRequiredOutput::DerivedEquivalenceContractReport,
            CertificationRequiredOutput::DerivedFallbackReport,
            CertificationRequiredOutput::DerivedFailureLocalityReport,
            CertificationRequiredOutput::DerivedBranchLocalParityReport,
            CertificationRequiredOutput::DerivedReplayParityReport,
            CertificationRequiredOutput::DerivedBridgeFamilyCoverageReport,
            CertificationRequiredOutput::MilestoneTwoCounterReport,
        ],
    }
}

pub fn milestone_three_closeout_suite_definition() -> CertificationSuiteDefinition {
    CertificationSuiteDefinition {
        suite_name: ".milestone_3.closeout".to_string(),
        canonical_rows: milestone_three_required_scenario_names()
            .into_iter()
            .map(|scenario| CertificationCanonicalRow {
                family: scenario,
                role: "HostileScenario".to_string(),
            })
            .collect(),
        rejection_rows: milestone_three_rejected_scenario_names()
            .into_iter()
            .map(|scenario| CertificationRejectionRow {
                family: scenario,
                role: "HostileRejection".to_string(),
                rejection_class: "InvariantBlocked".to_string(),
            })
            .collect(),
        parity_rows: milestone_three_replay_scenario_names()
            .into_iter()
            .map(|scenario| CertificationParityRow {
                family: scenario,
                parity_kind: "replay".to_string(),
            })
            .collect(),
        required_outputs: milestone_three_closeout_requirements().required_outputs,
    }
}

pub fn milestone_three_closeout_requirements() -> CertificationSuiteRequirements {
    CertificationSuiteRequirements {
        suite_name: ".milestone_3.closeout".to_string(),
        required_family_rows: milestone_three_required_scenario_names(),
        required_rejection_rows: milestone_three_rejected_scenario_names(),
        validator_expectations: Vec::new(),
        required_parity_rows: milestone_three_replay_scenario_names(),
        required_bridge_rows: Vec::new(),
        required_outputs: vec![
            CertificationRequiredOutput::MilestoneThreeHostileSuiteReport,
            CertificationRequiredOutput::MilestoneThreeHostileCoverageRows,
            CertificationRequiredOutput::MilestoneThreeHostileFamilyCoverageRows,
            CertificationRequiredOutput::MilestoneThreeRejectionDistributionRows,
            CertificationRequiredOutput::MilestoneThreeNamingDistributionRows,
            CertificationRequiredOutput::MilestoneThreeSideQuestCloseoutReport,
            CertificationRequiredOutput::MilestoneThreeReturnGateReport,
        ],
    }
}
