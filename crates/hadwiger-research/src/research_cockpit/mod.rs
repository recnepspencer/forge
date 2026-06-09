mod actions;
mod certification;
mod operations;
mod report;
mod session;
mod tile_equivalence;

use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::research_graph_invariants::ResearchGraphLegalityViolation;

pub use actions::{
    ResearchCockpitAction, ResearchCockpitActionBlocker, ResearchCockpitActionEligibility,
    ResearchCockpitActionKind, ResearchCockpitActionPacket, ResearchCockpitEquivalenceClass,
    ResearchCockpitEquivalenceScope,
};
pub use certification::{
    HadwigerCertificationBundle, HadwigerCertificationDigestInventory,
    HadwigerCertificationScenario,
};
pub use operations::{
    assemble_research_cockpit_session_checked, certify_hadwiger_milestone_one_bundle_checked,
    declare_tile_equivalence_witness_checked, derive_research_cockpit_action_packet_checked,
    replay_research_cockpit_session_checked,
};
pub use report::{ResearchCockpitCounters, ResearchCockpitReport};
pub use session::{
    ResearchCockpitInputDigest, ResearchCockpitSession, ResearchCockpitSessionBuilder,
};
pub use tile_equivalence::{
    PeriodicColorRuleSignature, TileConstraintSignature, TileContactGraphSignature,
    TileEquivalencePosture, TileEquivalenceScope, TileEquivalenceWitness,
    TileEquivalenceWitnessChecked, TileMetricThresholdSignature,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResearchCockpitError {
    Shape(HadwigerArtifactShapeError),
    MissingInput {
        field: &'static str,
    },
    ResearchGraphLegality {
        violations: Vec<ResearchGraphLegalityViolation>,
    },
}

impl From<HadwigerArtifactShapeError> for ResearchCockpitError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}
