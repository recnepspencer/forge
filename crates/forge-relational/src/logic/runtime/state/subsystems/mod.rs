mod durability;
mod history;
mod indexing;
mod lineage;
mod publication;
mod services;
mod visibility;

pub(crate) trait RuntimeSubsystem: Sized {
    type Config;

    fn new(config: &Self::Config) -> Self;
    fn fork(&self) -> Self;
}

pub(crate) use durability::DurabilitySubsystem;
pub(crate) use history::HistorySubsystem;
pub(crate) use indexing::IndexingSubsystem;
pub(crate) use lineage::LineageSubsystem;
pub(crate) use publication::PublicationSubsystem;
pub(crate) use services::{RuntimeInstrumentation, RuntimeServices};
pub(crate) use visibility::{
    ReplayRetentionState, SnapshotHandleBinding, VisibilityResidency, VisibilitySubsystem,
};
