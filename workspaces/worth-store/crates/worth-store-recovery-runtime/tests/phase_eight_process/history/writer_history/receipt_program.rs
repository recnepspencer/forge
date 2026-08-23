use std::path::{Path, PathBuf};

use super::super::schedule::ParentWriterDurabilityProfileSelection;
use super::expected_history::ExpectedWriterHistory;

#[path = "receipt_program/completed_binding.rs"]
mod completed_binding;
#[path = "receipt_program/decoding.rs"]
mod decoding;
#[path = "receipt_program/non_durable_binding.rs"]
mod non_durable_binding;

impl ExpectedWriterHistory {
    pub(crate) fn bind_identity_receipt(&mut self, path: &Path) -> Result<(), String> {
        let decoded = decoding::decode(path, self.payloads.len() + 2)?;
        let completed_count = self.payloads.len();
        let durable_bindings = completed_binding::bind(&decoded.entries[..completed_count], self)?;
        let non_durable = non_durable_binding::bind(
            &decoded.entries[completed_count..],
            completed_count,
            self.no_effect_identity,
            self.in_flight_material(),
            self.in_flight_fate(),
        )?;
        let writer_fates = decoded
            .entries
            .iter()
            .map(|entry| (entry.idempotency, entry.fate))
            .collect();
        self.writer_fates = writer_fates;
        self.durable_bindings = durable_bindings;
        self.no_effect_idempotency = Some(non_durable.no_effect_idempotency);
        self.dirty_idempotency = Some(non_durable.dirty_idempotency);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SubmittedOperationProgram {
    pub(crate) path: PathBuf,
    pub(crate) identity_receipt: PathBuf,
    pub(crate) barrier_receipt: PathBuf,
    pub(crate) expected: ExpectedWriterHistory,
    pub(crate) writer_profile_selection: ParentWriterDurabilityProfileSelection,
}
