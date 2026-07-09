use serde::{Deserialize, Serialize};

macro_rules! identity_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }
    };
}

identity_type!(ScenarioId);
identity_type!(FixtureId);
identity_type!(RunId);
identity_type!(SnapshotId);
identity_type!(DiagnosticsId);
identity_type!(ExplanationId);
identity_type!(ProvenanceId);
identity_type!(EventStreamId);
identity_type!(ReplayId);

pub fn scenario_id(name: &str) -> ScenarioId {
    ScenarioId::new(format!("scenario:{name}"))
}

pub fn fixture_id(scenario: &ScenarioId, profile_name: &str) -> FixtureId {
    FixtureId::new(format!("{}:fixture:{profile_name}", scenario.0))
}

pub fn run_id(scenario: &ScenarioId, profile_name: &str, request_name: &str) -> RunId {
    RunId::new(format!("{}:run:{profile_name}:{request_name}", scenario.0))
}

pub fn snapshot_id(run: &RunId, stage_name: &str) -> SnapshotId {
    SnapshotId::new(format!("{}:snapshot:{stage_name}", run.0))
}

pub fn diagnostics_id(run: &RunId) -> DiagnosticsId {
    DiagnosticsId::new(format!("{}:diagnostics", run.0))
}

pub fn explanation_id(run: &RunId, target_name: &str) -> ExplanationId {
    ExplanationId::new(format!("{}:explanation:{target_name}", run.0))
}

pub fn provenance_id(run: &RunId, target_name: &str) -> ProvenanceId {
    ProvenanceId::new(format!("{}:provenance:{target_name}", run.0))
}

pub fn event_stream_id(run: &RunId, stream_name: &str) -> EventStreamId {
    EventStreamId::new(format!("{}:events:{stream_name}", run.0))
}

pub fn replay_id(run: &RunId, replay_name: &str) -> ReplayId {
    ReplayId::new(format!("{}:replay:{replay_name}", run.0))
}
