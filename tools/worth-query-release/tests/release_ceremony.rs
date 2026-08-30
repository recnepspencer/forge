#[path = "release_ceremony/denial.rs"]
mod denial;
#[cfg(target_os = "linux")]
#[path = "release_ceremony/ed25519.rs"]
mod ed25519;
#[path = "release_ceremony/preflight.rs"]
mod preflight;
#[path = "release_ceremony/success.rs"]
mod success;
#[path = "release_ceremony/support.rs"]
mod support;
#[path = "release_ceremony/workflow_contract.rs"]
mod workflow_contract;
