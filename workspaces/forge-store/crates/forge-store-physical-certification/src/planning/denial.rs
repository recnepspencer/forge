use forge_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisConstructionDenial;
use forge_foundational::canonicalization_api::lower_lane::digest::CanonicalDigestDerivationDenial;

use crate::{PhysicalScenarioExpectationKind, PhysicalSimulationScenarioFamily};

use super::{
    ForbiddenShortcutKind, ObserverKind, OracleFamilyKind, PhysicalDriverKind,
    PhysicalSimulationCapability, PhysicalSimulationProfile,
};
use crate::DriverAdmissionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationPlanDenial {
    MissingCapability(PhysicalSimulationCapability),
    MissingPhysicalDriver(PhysicalDriverKind),
    UnboundYieldpointSchedule(String),
    DriverAdmissionDenied(DriverAdmissionDenial),
    MissingObserver(ObserverKind),
    MissingOracleFamily(OracleFamilyKind),
    UnsupportedProfile(PhysicalSimulationProfile),
    ResourceEnvelopeProfileMismatch {
        expected: PhysicalSimulationProfile,
        actual: PhysicalSimulationProfile,
    },
    UnsupportedScenarioShape {
        family: PhysicalSimulationScenarioFamily,
        expectation: PhysicalScenarioExpectationKind,
    },
    MissingEvidencePolicy,
    MissingS5PhysicalIsolationLaneRegistration,
    AbsentForbiddenShortcutSet,
    MissingForbiddenShortcut(ForbiddenShortcutKind),
    AmbiguousFaultScope,
    PlanCanonicalBasisDenied(CanonicalBasisConstructionDenial),
    PlanDigestDerivationDenied(CanonicalDigestDerivationDenial),
    ProofProgressionSkipped,
}
