#[path = "harness/adapter_capture.rs"]
mod adapter_capture;
#[path = "harness/adapter_core.rs"]
mod adapter_core;
#[path = "harness/assertions.rs"]
mod assertions;
#[path = "harness/profiles.rs"]
mod profiles;
#[path = "harness/runtime.rs"]
mod runtime;
#[path = "harness/scenario.rs"]
mod scenario;

pub use adapter_core::{signal_bench, signal_parity_suite, SignalHarnessAdapter};
pub use assertions::SignalHarnessAssert;
pub use profiles::SignalProfileCatalog;
pub use runtime::{
    SignalEvaluationDriver, SignalFixtureFactory, SignalHarnessRuntime,
    SignalHarnessRuntimeBuilder, SignalHarnessSession, SignalMutationAction,
};
pub use scenario::{SignalMutationBatch, SignalScenario};
