use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthValidationAuthoritySource {
    TopologyValidatorMaterializedReport,
    TopologyValidatorDerivedReport,
    ValidateInterpretedTopologyFacade,
    ValidateNamedTopologyTruthFacade,
    DerivedRuleRegistry(&'static str),
    MilestoneOneInvariantRegistration(&'static str),
    CertificationValidatorExpectations(&'static str),
    OperatorCloseoutValidatorFamilyCoverage,
    OperatorCloseoutValidationBreadth,
    OperatorCloseoutDerivedValidationInspection,
    BuildMilestoneOneRuntimeUsage(&'static str),
    OldAuthorityUseRegion(&'static str),
}

impl WorthValidationAuthoritySource {
    pub fn stable_key(self) -> String {
        match self {
            Self::TopologyValidatorMaterializedReport => {
                "topology-validator.materialized-validation-report".to_string()
            }
            Self::TopologyValidatorDerivedReport => {
                "topology-validator.derived-validation-report".to_string()
            }
            Self::ValidateInterpretedTopologyFacade => {
                "validation-facade.validate-interpreted-topology".to_string()
            }
            Self::ValidateNamedTopologyTruthFacade => {
                "validation-facade.validate-named-topology-truth".to_string()
            }
            Self::DerivedRuleRegistry(name) => format!("derived-rule-registry.{name}"),
            Self::MilestoneOneInvariantRegistration(name) => {
                format!("milestone-one-invariant-registration.{name}")
            }
            Self::CertificationValidatorExpectations(suite) => {
                format!("certification-validator-expectations.{suite}")
            }
            Self::OperatorCloseoutValidatorFamilyCoverage => {
                "operator-closeout.validator-family-coverage".to_string()
            }
            Self::OperatorCloseoutValidationBreadth => {
                "operator-closeout.validation-breadth".to_string()
            }
            Self::OperatorCloseoutDerivedValidationInspection => {
                "operator-closeout.derived-validation-inspection".to_string()
            }
            Self::BuildMilestoneOneRuntimeUsage(scope) => {
                format!("operator-closeout.build-milestone-one-runtime.{scope}")
            }
            Self::OldAuthorityUseRegion(region) => format!("old-authority-use-region.{region}"),
        }
    }
}

impl fmt::Display for WorthValidationAuthoritySource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.stable_key())
    }
}
