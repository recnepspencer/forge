use crate::validator_invariant_catalog::{
    WorthTopologyMilestoneNineCloseoutDenial, WorthTopologyOperatorCertificationCutoverDenial,
    WorthTopologyRelationalInvariantCatalogDenial,
    WorthTopologySelectedGraphObligationEnforcementDenial,
    WorthTopologySelectedValidatorEnforcementDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthTopologyLegalityCatalogError {
    MissingTouchedApplicability(String),
    MissingRequiredAccessPosture(String),
    MissingEnforcementPhase(String),
    MissingWitnessPosture(String),
    MissingDiagnosticProjection(String),
    MissingMilestoneEightReceiptContext,
    MilestoneEightSeedClaimsValidatorSelection(String),
    InvariantRegistration(String),
    UnknownValidatorApplicability(String),
    UnknownValidatorWitnessPosture(String),
    UnknownInvariantApplicability(String),
    UnknownInvariantExecutionPoint(String),
    UnknownInvariantWitnessPosture(String),
    QueryRegistration(String),
    MissingQueryProjectionRow(String),
    SourceFirewall(String),
    ConflictingFamilyIdentity(String),
    PhaseFourEnforcement(WorthTopologySelectedValidatorEnforcementDenial),
    RelationalInvariantCatalog(WorthTopologyRelationalInvariantCatalogDenial),
    PhaseSixGraphObligationEnforcement(WorthTopologySelectedGraphObligationEnforcementDenial),
    OperatorCertificationCutover(WorthTopologyOperatorCertificationCutoverDenial),
    MilestoneNineCloseout(WorthTopologyMilestoneNineCloseoutDenial),
}

impl std::fmt::Display for WorthTopologyLegalityCatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTouchedApplicability(family) => {
                write!(f, "family `{family}` is missing touched applicability")
            }
            Self::MissingRequiredAccessPosture(family) => {
                write!(f, "family `{family}` is missing required access posture")
            }
            Self::MissingEnforcementPhase(family) => {
                write!(f, "family `{family}` is missing enforcement phase")
            }
            Self::MissingWitnessPosture(family) => {
                write!(f, "family `{family}` is missing witness posture")
            }
            Self::MissingDiagnosticProjection(family) => {
                write!(f, "family `{family}` is missing diagnostic projection")
            }
            Self::MissingMilestoneEightReceiptContext => f.write_str(
                "Milestone 9 catalog requires Milestone 8 receipt context before graph obligation declaration",
            ),
            Self::MilestoneEightSeedClaimsValidatorSelection(seed) => write!(
                f,
                "Milestone 8 seed `{seed}` cannot claim validator selection authority"
            ),
            Self::InvariantRegistration(message) => {
                write!(f, "invariant registration source failed: {message}")
            }
            Self::UnknownValidatorApplicability(rule_name) => write!(
                f,
                "validator rule `{rule_name}` has no declared touched applicability"
            ),
            Self::UnknownValidatorWitnessPosture(rule_name) => write!(
                f,
                "validator rule `{rule_name}` has no declared witness posture"
            ),
            Self::UnknownInvariantApplicability(rule_id) => write!(
                f,
                "invariant rule `{rule_id}` has no declared touched applicability"
            ),
            Self::UnknownInvariantExecutionPoint(execution_point) => write!(
                f,
                "invariant execution point `{execution_point}` cannot enter the Phase 2 legality catalog"
            ),
            Self::UnknownInvariantWitnessPosture(rule_id) => write!(
                f,
                "invariant rule `{rule_id}` has no declared witness posture"
            ),
            Self::QueryRegistration(message) => write!(f, "Query registration failed: {message}"),
            Self::MissingQueryProjectionRow(registration_digest) => write!(
                f,
                "Query registration `{registration_digest}` has no Worth family projection row"
            ),
            Self::SourceFirewall(message) => write!(f, "source firewall failed: {message}"),
            Self::ConflictingFamilyIdentity(identity) => {
                write!(f, "duplicate legality family identity `{identity}`")
            }
            Self::PhaseFourEnforcement(denial) => {
                write!(f, "Phase 4 selected validator enforcement denied: {denial}")
            }
            Self::RelationalInvariantCatalog(denial) => {
                write!(f, "Phase 5 relational invariant catalog denied: {denial}")
            }
            Self::PhaseSixGraphObligationEnforcement(denial) => {
                write!(f, "Phase 6 graph obligation enforcement denied: {denial}")
            }
            Self::OperatorCertificationCutover(denial) => {
                write!(f, "operator certification cutover denied: {denial}")
            }
            Self::MilestoneNineCloseout(denial) => {
                write!(f, "Milestone 9 closeout denied: {denial}")
            }
        }
    }
}

impl std::error::Error for WorthTopologyLegalityCatalogError {}
