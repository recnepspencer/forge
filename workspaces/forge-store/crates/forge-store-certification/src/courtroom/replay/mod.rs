//! Replay traces, layout observers, and runtime verifier comparison.

pub use crate::courtroom::cross_cutting::observed_trace::{
    FixtureAdversaryPosture, FixtureAdversaryReport, LargeStorePressureClass,
    ObservedPhysicalTrace, PhysicalCounterExpectationKind, RuntimeVerifierParityTrace,
    RuntimeVerifierRelationship, ScenarioCounterExpectation, ScenarioCounterObservation,
    ScenarioCounterTrace, ScenarioDenialBoundary, ScenarioDenialTrace, ShortcutRejectionTrace,
};
pub use crate::courtroom::cross_cutting::observer_trace::ScenarioObserverTrace;
pub use crate::courtroom::cross_cutting::runtime_verifier_comparison::{
    PhysicalRuntimeVerifierComparison, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
};
pub use crate::courtroom::cross_cutting::runtime_verifier_diagnostics::{
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind, RuntimeVerifierDiagnosticReport,
};
pub use crate::courtroom::cross_cutting::runtime_verifier_support::{
    RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport,
};
pub use crate::courtroom::cross_cutting::scale_property::PhysicalScalePropertyEvidence;
pub use crate::courtroom::layout::layout_observers::{
    OfflineVerifierObserver, PhysicalLayoutParity, PhysicalLayoutParityDenial,
    PhysicalLayoutParityReport, RuntimeLayoutObserver,
};
pub use crate::replay::layout::{assemble_layout_index_layout_replay_bundle, LayoutReplayBundle};
pub use crate::scenario::cross_cutting::scale_fixture::{
    PhysicalHostileScaleCondition, PhysicalHostileScaleFixtureDenial,
    PhysicalHostileScaleFixtureReport, PhysicalHostileScaleFixtureSource,
};
pub use crate::scenario::cross_cutting::story_transcript::PhysicalStoryTranscript;
