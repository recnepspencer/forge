#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthValidationAuthorityScanPattern {
    ValidatorExpectations,
    DerivedTopologyRuleSpecs,
    MilestoneOneInvariantRegistrations,
    DerivedValidationReport,
    MaterializedValidationReport,
    BuildMilestoneOneRuntime,
    ValidateInterpretedTopology,
    ValidateNamedTopologyTruth,
}

impl WorthValidationAuthorityScanPattern {
    pub const fn pattern(self) -> &'static str {
        match self {
            Self::ValidatorExpectations => "validator_expectations",
            Self::DerivedTopologyRuleSpecs => "DERIVED_TOPOLOGY_RULE_SPECS",
            Self::MilestoneOneInvariantRegistrations => "milestone_one_invariant_registrations",
            Self::DerivedValidationReport => "TopologyValidator::derived_validation_report",
            Self::MaterializedValidationReport => {
                "TopologyValidator::materialized_validation_report"
            }
            Self::BuildMilestoneOneRuntime => "build_milestone_one_runtime",
            Self::ValidateInterpretedTopology => "validate_interpreted_topology",
            Self::ValidateNamedTopologyTruth => "validate_named_topology_truth",
        }
    }

    pub const fn all() -> &'static [Self] {
        &[
            Self::ValidatorExpectations,
            Self::DerivedTopologyRuleSpecs,
            Self::MilestoneOneInvariantRegistrations,
            Self::DerivedValidationReport,
            Self::MaterializedValidationReport,
            Self::BuildMilestoneOneRuntime,
            Self::ValidateInterpretedTopology,
            Self::ValidateNamedTopologyTruth,
        ]
    }
}
