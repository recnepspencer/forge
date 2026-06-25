#[path = "phase_chain/admitted_scaffold/mod.rs"]
#[cfg(test)]
mod admitted_scaffold;
#[path = "result_surface/artifact.rs"]
#[cfg(test)]
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
mod family;
pub(crate) mod graph_obligation_adoption;
#[path = "phase_chain/intent.rs"]
#[cfg(test)]
pub(crate) mod intent;
#[path = "result_surface/outcome.rs"]
#[cfg(test)]
pub(crate) mod outcome;
#[cfg(test)]
#[path = "proof/mod.rs"]
mod proof;
#[cfg(test)]
pub(crate) mod query_access_planning;
#[cfg(test)]
pub(crate) mod query_authority;
#[cfg(test)]
pub(crate) mod query_enforcement_adoption;
#[cfg(test)]
mod query_support_pins;
#[cfg(test)]
mod realization_snapshot;
#[path = "phase_chain/request.rs"]
#[cfg(test)]
pub(crate) mod request;
#[path = "result_surface/result.rs"]
#[cfg(test)]
pub(crate) mod result;
#[path = "phase_chain/specs.rs"]
#[cfg(test)]
pub(crate) mod specs;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
