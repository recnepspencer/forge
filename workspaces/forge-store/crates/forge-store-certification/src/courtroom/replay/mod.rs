//! Replay traces, layout observers, and runtime verifier comparison.

pub use crate::layout_observers::{
    OfflineVerifierObserver, PhysicalLayoutParity, PhysicalLayoutParityDenial,
    PhysicalLayoutParityReport, RuntimeLayoutObserver,
};
pub use crate::observed_trace::{
    FixtureAdversaryPosture, FixtureAdversaryReport, LargeStorePressureClass,
    ObservedPhysicalTrace, PhysicalCounterExpectationKind, RuntimeVerifierParityTrace,
    RuntimeVerifierRelationship, ScenarioCounterExpectation, ScenarioCounterObservation,
    ScenarioCounterTrace, ScenarioDenialBoundary, ScenarioDenialTrace, ShortcutRejectionTrace,
};
pub use crate::observer_trace::ScenarioObserverTrace;
pub use crate::runtime_verifier_comparison::{
    PhysicalRuntimeVerifierComparison, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
};
pub use crate::runtime_verifier_diagnostics::{
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind, RuntimeVerifierDiagnosticReport,
};
pub use crate::runtime_verifier_support::{
    RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport,
};
pub use crate::scale_fixture::{
    PhysicalHostileScaleCondition, PhysicalHostileScaleFixtureDenial,
    PhysicalHostileScaleFixtureReport, PhysicalHostileScaleFixtureSource,
};
pub use crate::scale_property::PhysicalScalePropertyEvidence;
pub use crate::story_transcript::PhysicalStoryTranscript;
