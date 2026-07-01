mod denial;
mod identity;
mod parts;
mod replay_bundle;
mod transcript;

pub(crate) use denial::require_plan_bound_oracle_verdicts_for_replay_basis;
pub use denial::{
    reject_copied_transcript_fields, reject_loose_log_transcript_attempt,
    reject_same_run_self_comparison_transcript_attempt, reject_terminal_json_transcript_attempt,
    TranscriptReplayDenial,
};
pub use identity::{
    PhysicalSimulationTranscriptIdentity, SimulationRunIdentity, TranscriptReplayEvidenceIdentity,
};
pub use parts::ExecutedTranscriptParts;
pub use replay_bundle::{DetachedSimulationReplayParts, SimulationReplayBundle};
pub use transcript::{PhysicalSimulationTranscript, PhysicalStoryTranscript};
