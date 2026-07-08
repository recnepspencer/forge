//! Partial-publication classification: evidence kind → outcome decision table → receipt.
mod ambiguity;
mod assembly;
mod crash_edge;
mod no_undo_hazard;
mod non_authoritative;
mod orchestration;
mod torn_publication;

pub use orchestration::PartialPublicationClassification;
