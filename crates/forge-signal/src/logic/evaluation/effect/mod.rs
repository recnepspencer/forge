mod assembly;
mod diagnostics;
mod operational;
mod report;

pub(crate) use assembly::{EffectRuntimeMetadata, EvaluationEffect};
pub use diagnostics::DiagnosticEnvelope;
pub use operational::{DeferralReason, EvaluationVerdict, OperationalEffect, SuppressionReason};
pub(crate) use operational::{EffectDependencyInputs, PendingDependencySnapshot};
pub use report::AppliedEffectReport;
pub(crate) use report::{EffectComparison, PreparedApplyResult};
