mod counters;
mod report;
mod warnings;

pub use counters::CanonicalizationCounters;
pub use report::{CanonicalizationReport, CompatibilityEvidence, IdentityFreezeEvidence};
pub use warnings::{CanonicalizationWarning, NormalizationEvent};
