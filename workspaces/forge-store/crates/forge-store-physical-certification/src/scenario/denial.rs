use forge_foundational::canonicalization_api::lower_lane::{
    basis::CanonicalBasisConstructionDenial, digest::CanonicalDigestDerivationDenial,
};
use forge_store_aspect_native::StoreCanonicalBasisConstructionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalScenarioDefinitionDenial {
    MissingScenarioFamily,
    MissingScenarioIntent,
    MissingAspectNativeFixture,
    DuplicateAspectNativeFixture,
    MissingActor,
    UnnamedActorId,
    DuplicateActorId,
    MissingSchedule,
    UnnamedProductionBoundaryYieldpoint,
    MissingExpectation,
    JsonScenarioAuthorityDenied,
    TerminalProjectionScenarioDenied,
    RawStringScenarioAuthorityDenied,
    CopiedDigestScenarioAuthorityDenied,
    FixtureLabelScenarioAuthorityDenied,
    ProofProgressionSkipped,
    FixtureCanonicalBasisDenied(StoreCanonicalBasisConstructionDenial),
    ScenarioCanonicalBasisDenied(CanonicalBasisConstructionDenial),
    ScenarioDigestDerivationDenied(CanonicalDigestDerivationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonScenarioAuthorityDenied {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalProjectionScenarioDenied {
    _private: (),
}
