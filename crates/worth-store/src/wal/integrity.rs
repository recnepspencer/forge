use crate::failure::{StoreError, StoreErrorKind};
use crate::media::{
    barriers::validate_barrier_satisfies_requirement,
    frame_payload,
    framing::{scan_tail, TailValidationOutcome},
    validate_raw_record, BarrierClassifiedDurableRecord, DurabilityBarrierClass,
    DurableMediaFamily,
};

use super::{
    digest::{stable_digest, WalRecordDigestBasis},
    model::{WalRecord, CURRENT_WAL_VERSION},
};

impl WalRecord {
    pub fn validate_integrity(&self) -> Result<(), StoreError> {
        if self.wal_version != CURRENT_WAL_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::WalCanonicalizationVersionUnsupported,
                format!(
                    "wal record {} uses unsupported wal version {}",
                    self.wal_sequence, self.wal_version
                ),
            ));
        }
        let recomputed = stable_digest(&WalRecordDigestBasis {
            family: self.family,
            durable_mutation_id: self.durable_mutation_id,
            runtime_session_id: &self.runtime_session_id,
            wal_version: self.wal_version,
            payload: &self.payload,
        })?;
        if recomputed != self.record_digest {
            return Err(StoreError::new(
                StoreErrorKind::WalDigestMismatch,
                format!(
                    "wal record {} failed digest verification for durable mutation {}",
                    self.wal_sequence, self.durable_mutation_id.0
                ),
            ));
        }
        self.validate_media_frame_contract()?;
        Ok(())
    }

    pub(crate) fn classify_media_barrier(
        &self,
        barrier_class: DurabilityBarrierClass,
    ) -> Result<BarrierClassifiedDurableRecord, StoreError> {
        let framed = frame_payload(DurableMediaFamily::WalRecord, self)?;
        let validated = validate_raw_record(framed.to_raw_bytes())?;
        let decoded = Self::decode_from_media_bytes(framed.as_bytes().to_vec())?;
        if decoded != *self {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} failed framed media roundtrip validation",
                    self.wal_sequence
                ),
            ));
        }
        Ok(BarrierClassifiedDurableRecord::classify(
            validated,
            barrier_class,
        ))
    }

    fn validate_media_frame_contract(&self) -> Result<(), StoreError> {
        let classified =
            self.classify_media_barrier(DurabilityBarrierClass::TransactionalCommitDurable)?;
        let report = scan_tail(classified.record().framed_record().as_bytes())?;
        if report.outcome() != TailValidationOutcome::Clean || report.valid_record_count() != 1 {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} did not roundtrip to one clean durable frame",
                    self.wal_sequence
                ),
            ));
        }
        validate_barrier_satisfies_requirement(
            classified.barrier_class(),
            DurabilityBarrierClass::FileContentDurable,
        )?;
        if classified.record().version() != crate::media::CURRENT_DURABLE_MEDIA_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::DurableFamilyVersionUnsupported,
                format!(
                    "wal record {} uses unsupported durable media version {}",
                    self.wal_sequence,
                    classified.record().version()
                ),
            ));
        }
        if classified.record().family() != DurableMediaFamily::WalRecord {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} did not preserve WAL family classification",
                    self.wal_sequence
                ),
            ));
        }
        if classified.record().framed_record().payload_len() == 0 {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} encoded an empty durable payload",
                    self.wal_sequence
                ),
            ));
        }
        Ok(())
    }
}
