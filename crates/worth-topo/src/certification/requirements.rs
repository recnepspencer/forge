use crate::certification::core::{
    WorthCertificationBridgeExpectation, WorthCertificationCanonicalRow,
    WorthCertificationParityRow, WorthCertificationRejectionRow, WorthCertificationRequiredOutput,
    WorthCertificationSuiteDefinition, WorthCertificationSuiteRequirements,
    WorthCertificationValidatorExpectation,
};
use crate::certification::shared::{
    canonical_milestone_one_primitive_families, derived_validator_expectations_for_family,
    validator_expectations_for_family,
};

pub fn milestone_one_closeout_suite_definition() -> WorthCertificationSuiteDefinition {
    let canonical_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Smallest".to_string(),
                },
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Generic".to_string(),
                },
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "HostileAdmitted".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let rejection_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationRejectionRow {
            family: family.to_string(),
            role: "OutOfClass".to_string(),
            rejection_class: "OutOfClass".to_string(),
        })
        .collect::<Vec<_>>();
    let parity_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                WorthCertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "replay".to_string(),
                },
                WorthCertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "branch".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();

    WorthCertificationSuiteDefinition {
        suite_name: "worth.milestone_1.closeout".to_string(),
        canonical_rows,
        rejection_rows,
        parity_rows,
        required_outputs: milestone_one_closeout_requirements().required_outputs,
    }
}

pub fn milestone_one_closeout_requirements() -> WorthCertificationSuiteRequirements {
    let required_family_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let validator_expectations = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationValidatorExpectation {
            family: family.to_string(),
            validators: validator_expectations_for_family(family)
                .iter()
                .map(|validator| (*validator).to_string())
                .collect(),
        })
        .collect::<Vec<_>>();
    let required_bridge_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationBridgeExpectation {
            family: family.to_string(),
        })
        .collect::<Vec<_>>();

    WorthCertificationSuiteRequirements {
        suite_name: "worth.milestone_1.closeout".to_string(),
        required_family_rows: required_family_rows.clone(),
        required_rejection_rows: required_family_rows,
        validator_expectations,
        required_parity_rows: canonical_milestone_one_primitive_families()
            .into_iter()
            .map(str::to_string)
            .collect(),
        required_bridge_rows,
        required_outputs: vec![
            WorthCertificationRequiredOutput::TopologyTruthDigest,
            WorthCertificationRequiredOutput::NamingTruthDigest,
            WorthCertificationRequiredOutput::TopologyValidationDigest,
            WorthCertificationRequiredOutput::TopologyValidationReport,
            WorthCertificationRequiredOutput::TopologyLocalizationReport,
            WorthCertificationRequiredOutput::NamingAttachmentReport,
            WorthCertificationRequiredOutput::PrimitiveFamilyCoverageMatrix,
            WorthCertificationRequiredOutput::PrimitiveCorpusParityReport,
            WorthCertificationRequiredOutput::AdmittedRangeSweepReport,
            WorthCertificationRequiredOutput::ValidatorCoverageReport,
            WorthCertificationRequiredOutput::BranchLocalTopologyReport,
            WorthCertificationRequiredOutput::ReplayParityReport,
            WorthCertificationRequiredOutput::RejectionClassReport,
            WorthCertificationRequiredOutput::FailureLocalityReport,
            WorthCertificationRequiredOutput::BridgeFamilyCoverageReport,
            WorthCertificationRequiredOutput::BridgeProofReport,
            WorthCertificationRequiredOutput::CounterReport,
        ],
    }
}

pub fn milestone_two_closeout_suite_definition() -> WorthCertificationSuiteDefinition {
    let canonical_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Smallest".to_string(),
                },
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "Generic".to_string(),
                },
                WorthCertificationCanonicalRow {
                    family: family.to_string(),
                    role: "HostileAdmitted".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let rejection_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationRejectionRow {
            family: family.to_string(),
            role: "OutOfClass".to_string(),
            rejection_class: "OutOfClass".to_string(),
        })
        .collect::<Vec<_>>();
    let parity_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .flat_map(|family| {
            [
                WorthCertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "replay".to_string(),
                },
                WorthCertificationParityRow {
                    family: family.to_string(),
                    parity_kind: "branch".to_string(),
                },
            ]
        })
        .collect::<Vec<_>>();

    WorthCertificationSuiteDefinition {
        suite_name: "worth.milestone_2.closeout".to_string(),
        canonical_rows,
        rejection_rows,
        parity_rows,
        required_outputs: milestone_two_closeout_requirements().required_outputs,
    }
}

pub fn milestone_two_closeout_requirements() -> WorthCertificationSuiteRequirements {
    let required_family_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let validator_expectations = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationValidatorExpectation {
            family: family.to_string(),
            validators: derived_validator_expectations_for_family(family)
                .iter()
                .map(|validator| (*validator).to_string())
                .collect(),
        })
        .collect::<Vec<_>>();
    let required_bridge_rows = canonical_milestone_one_primitive_families()
        .into_iter()
        .map(|family| WorthCertificationBridgeExpectation {
            family: family.to_string(),
        })
        .collect::<Vec<_>>();

    WorthCertificationSuiteRequirements {
        suite_name: "worth.milestone_2.closeout".to_string(),
        required_family_rows: required_family_rows.clone(),
        required_rejection_rows: required_family_rows,
        validator_expectations,
        required_parity_rows: canonical_milestone_one_primitive_families()
            .into_iter()
            .map(str::to_string)
            .collect(),
        required_bridge_rows,
        required_outputs: vec![
            WorthCertificationRequiredOutput::MaterializedTopologyDigest,
            WorthCertificationRequiredOutput::InterpretedTopologyDigest,
            WorthCertificationRequiredOutput::DerivedValidationDigest,
            WorthCertificationRequiredOutput::DerivedTruthBasisDigest,
            WorthCertificationRequiredOutput::BridgeRoutingDigest,
            WorthCertificationRequiredOutput::BridgeHistoricalEvaluationDigest,
            WorthCertificationRequiredOutput::DerivedFamilyCoverageMatrix,
            WorthCertificationRequiredOutput::DerivedFamilyParityMatrix,
            WorthCertificationRequiredOutput::DerivedValidatorCoverageReport,
            WorthCertificationRequiredOutput::DerivedInvalidationReport,
            WorthCertificationRequiredOutput::DerivedRebuildReport,
            WorthCertificationRequiredOutput::DerivedEquivalenceContractReport,
            WorthCertificationRequiredOutput::DerivedFallbackReport,
            WorthCertificationRequiredOutput::DerivedFailureLocalityReport,
            WorthCertificationRequiredOutput::DerivedBranchLocalParityReport,
            WorthCertificationRequiredOutput::DerivedReplayParityReport,
            WorthCertificationRequiredOutput::DerivedBridgeFamilyCoverageReport,
            WorthCertificationRequiredOutput::MilestoneTwoCounterReport,
        ],
    }
}
