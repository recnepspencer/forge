//! Courtroom-only certification harness test support modules.
//!
//! These builders and fixture shims exist to exercise certification evidence
//! paths inside the courtroom. They are not production authority surfaces.

#[cfg(test)]
pub(crate) mod authenticity_integrity_test_support;
#[cfg(test)]
pub(crate) mod bounded_memory_closeout_test_support;
#[cfg(test)]
pub(crate) mod dirty_publication_evidence_test_support;
#[cfg(test)]
pub(crate) mod integrity_handoff_test_support;
#[cfg(test)]
pub(crate) mod integrity_readiness_test_support;
#[cfg(test)]
pub(crate) mod physical_container_integrity_test_support;
#[cfg(test)]
pub(crate) mod physical_integrity_closeout_harness_test_support;
#[cfg(test)]
pub(crate) mod physical_integrity_closeout_line_cap_test_support;
#[cfg(test)]
pub(crate) mod physical_integrity_closeout_test_support;
#[cfg(test)]
pub(crate) mod physical_scope_admission_test_support;
#[cfg(test)]
pub(crate) mod pre_decode_physical_admission_test_support;
#[cfg(test)]
pub(crate) mod record_view_evidence_test_support;
#[cfg(test)]
pub(crate) mod recovery_blocking_damage_test_support;
