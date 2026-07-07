mod current;
mod projection;
mod selection;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use current::current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader;
#[cfg(test)]
pub(crate) use current::current_worth_touched_graph_conflict_derived_diagnostic_projection;
pub use current::current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy;
pub use projection::{
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictDerivedDiagnosticProjection,
};
pub(crate) use selection::select_rich_localization;
