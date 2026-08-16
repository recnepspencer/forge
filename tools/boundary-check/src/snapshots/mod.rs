mod baseline;
mod candidate;
mod commit;
mod committed_facade_snapshot;
mod crate_dag;
pub(crate) mod document;
mod facade_reexport_validation;
mod facade_surface_observation;
mod facade_visibility;
mod session;

pub(crate) use session::{FacadeVocabularyAuthority, SnapshotMode, SnapshotSession};

#[cfg(test)]
pub(crate) use committed_facade_snapshot::load_committed_facade_exports;
