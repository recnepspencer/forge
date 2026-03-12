mod bridge;
mod capture;
mod assertions;
mod profiles;
mod runtime;
mod scenario;

pub use bridge::{signal_bench, signal_parity_suite, SignalHarnessBridge};
pub use assertions::SignalHarnessAssert;
pub use profiles::SignalProfileCatalog;
pub use runtime::{
    SignalEvaluationDriver, SignalFixtureFactory, SignalHarnessRuntime,
    SignalHarnessRuntimeBuilder, SignalHarnessSession, SignalMutationAction,
};
pub use scenario::{SignalMutationBatch, SignalScenario};
