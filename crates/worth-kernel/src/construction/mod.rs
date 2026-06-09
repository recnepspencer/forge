#[cfg(test)]
#[path = "phase_chain/admitted_scaffold/mod.rs"]
mod admitted_scaffold;
#[cfg(test)]
#[path = "result_surface/artifact.rs"]
mod artifact;
#[cfg(test)]
pub(crate) mod authoring;
#[cfg(test)]
#[path = "certification/mod.rs"]
pub(crate) mod certification;
#[cfg(test)]
mod digest;
#[cfg(test)]
mod digest_protocol;
#[cfg(test)]
mod family;
#[cfg(test)]
#[path = "phase_chain/intent.rs"]
pub(crate) mod intent;
#[cfg(test)]
#[path = "result_surface/outcome.rs"]
pub(crate) mod outcome;
#[cfg(test)]
#[path = "proof/mod.rs"]
mod proof;
#[cfg(test)]
mod realization_snapshot;
#[cfg(test)]
#[path = "phase_chain/request.rs"]
pub(crate) mod request;
#[cfg(test)]
#[path = "result_surface/result.rs"]
pub(crate) mod result;
#[cfg(test)]
#[path = "phase_chain/specs.rs"]
pub(crate) mod specs;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
