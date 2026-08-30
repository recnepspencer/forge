//! Carried observation of bounded cold-path reconstruction work.

use crate::package::{
    WorthQueryPortablePackageReconstructionDenial as Denial,
    WorthQueryPortablePackageReconstructionLimits, WorthQueryPortablePackageRecord,
};

const RECORD_FRAMING_BYTES: u64 = 5;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryPortablePackageReconstructionWork {
    logical_bytes: u64,
    nested_entries: u64,
    canonical_work_bytes: u64,
}

impl WorthQueryPortablePackageReconstructionWork {
    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn nested_entries(self) -> u64 {
        self.nested_entries
    }

    pub const fn canonical_work_bytes(self) -> u64 {
        self.canonical_work_bytes
    }

    pub(super) fn observe_record(
        self,
        record: &WorthQueryPortablePackageRecord,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> Result<Self, Denial> {
        if let WorthQueryPortablePackageRecord::ApplicationSchema(schema) = record {
            return self.observe_application_schema(schema, limits);
        }
        let (logical_bytes, nested_entries) = record_work(record);
        let nested_observation = Self {
            logical_bytes: self.logical_bytes,
            nested_entries: checked_add(self.nested_entries, nested_entries)?,
            canonical_work_bytes: self.canonical_work_bytes,
        };
        nested_observation.validate(limits)?;
        let observed = Self {
            logical_bytes: checked_add(self.logical_bytes, RECORD_FRAMING_BYTES)?
                .checked_add(logical_bytes)
                .ok_or(Denial::WorkObservationOverflow)?,
            ..nested_observation
        };
        observed.validate(limits)?;
        Ok(observed)
    }

    fn observe_application_schema(
        self,
        schema: &worth_query_declaration::facade::application_schema::WorthQueryPortableApplicationSchemaRecord,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> Result<Self, Denial> {
        let maximum_source_bytes = self
            .remaining_logical_bytes(limits)
            .saturating_sub(RECORD_FRAMING_BYTES);
        let work = worth_query_declaration::facade::application_schema::observe_portable_application_schema_reconstruction_work(
            schema,
            maximum_source_bytes,
            self.remaining_nested_entries(limits),
        )
        .map_err(|denial| match denial {
            worth_query_declaration::facade::application_schema::WorthQueryPortableApplicationSchemaWorkObservationDenial::SourceByteBudgetExceeded { observed, .. } => {
                Denial::LogicalByteBudgetExceeded {
                    observed: self.logical_bytes
                        .saturating_add(RECORD_FRAMING_BYTES)
                        .saturating_add(observed),
                    maximum: limits.maximum_logical_bytes(),
                }
            }
            worth_query_declaration::facade::application_schema::WorthQueryPortableApplicationSchemaWorkObservationDenial::NestedEntryBudgetExceeded { observed, .. } => {
                Denial::NestedEntryBudgetExceeded {
                    observed: self.nested_entries.saturating_add(observed),
                    maximum: limits.maximum_nested_entries(),
                }
            }
        })?;
        let observed = Self {
            logical_bytes: checked_add(self.logical_bytes, RECORD_FRAMING_BYTES)?
                .checked_add(work.source_bytes())
                .ok_or(Denial::WorkObservationOverflow)?,
            nested_entries: checked_add(self.nested_entries, work.canonical_entries())?,
            canonical_work_bytes: self.canonical_work_bytes,
        };
        observed.validate(limits)?;
        Ok(observed)
    }

    pub(super) fn consume_canonical_work(
        self,
        bytes: u64,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> Result<Self, Denial> {
        let observed = Self {
            canonical_work_bytes: checked_add(self.canonical_work_bytes, bytes)?,
            ..self
        };
        observed.validate(limits)?;
        Ok(observed)
    }

    pub(super) const fn remaining_logical_bytes(
        self,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> u64 {
        limits
            .maximum_logical_bytes()
            .saturating_sub(self.logical_bytes)
    }

    pub(super) const fn remaining_nested_entries(
        self,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> u64 {
        limits
            .maximum_nested_entries()
            .saturating_sub(self.nested_entries)
    }

    pub(super) const fn remaining_canonical_work_bytes(
        self,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> u64 {
        limits
            .maximum_canonical_work_bytes()
            .saturating_sub(self.canonical_work_bytes)
    }

    fn validate(self, limits: WorthQueryPortablePackageReconstructionLimits) -> Result<(), Denial> {
        if self.logical_bytes > limits.maximum_logical_bytes() {
            return Err(Denial::LogicalByteBudgetExceeded {
                observed: self.logical_bytes,
                maximum: limits.maximum_logical_bytes(),
            });
        }
        if self.nested_entries > limits.maximum_nested_entries() {
            return Err(Denial::NestedEntryBudgetExceeded {
                observed: self.nested_entries,
                maximum: limits.maximum_nested_entries(),
            });
        }
        if self.canonical_work_bytes > limits.maximum_canonical_work_bytes() {
            return Err(Denial::CanonicalWorkBudgetExceeded {
                observed: self.canonical_work_bytes,
                maximum: limits.maximum_canonical_work_bytes(),
            });
        }
        Ok(())
    }
}

fn record_work(record: &WorthQueryPortablePackageRecord) -> (u64, u64) {
    use WorthQueryPortablePackageRecord as Record;
    let logical_only = |bytes| (bytes, 0);
    match record {
        Record::DomainIdentity(value) => logical_only(text(value.owner()).saturating_add(8)),
        Record::CapabilityRequirement(value) => logical_only(text(value.as_str())),
        Record::ConfigurationRequirement(value) => logical_only(text(value.as_str())),
        Record::OperatingRequirement(value) => logical_only(text(value.as_str())),
        Record::Definition(value) => logical_only(
            text(value.slot())
                .saturating_add(text(value.semantics()))
                .saturating_add(1),
        ),
        Record::DomainOperation(value) => value.reconstruction_work(),
        Record::ArtifactContract(value) => value.reconstruction_work(),
        Record::ApplicationSchema(_) => unreachable!("application schema has owner observation"),
        Record::ConditionalApplicationOperation(value) => [
            value.schema_owner(),
            value.schema_name(),
            value.application_operation(),
            value.input_type(),
            value.domain_operation_slot(),
            value.domain_operation_canonical_identity(),
        ]
        .into_iter()
        .map(text)
        .fold((0, 0), |(bytes, entries), value| {
            (bytes.saturating_add(value), entries)
        }),
        Record::ContributionPolicy(value) => logical_only(text(value.as_str())),
        Record::NativeAspectContract(value) => {
            let work = value.reconstruction_work();
            (work.logical_bytes, work.nested_entries)
        }
        Record::ApplicationOperationContract(value) => {
            let work = value.reconstruction_work();
            (work.logical_bytes, work.nested_entries)
        }
    }
}

pub(crate) fn text(value: &str) -> u64 {
    8_u64.saturating_add(u64::try_from(value.len()).unwrap_or(u64::MAX))
}

fn checked_add(left: u64, right: u64) -> Result<u64, Denial> {
    left.checked_add(right)
        .ok_or(Denial::WorkObservationOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::WorthQueryPortableDomainIdentity;

    #[test]
    fn record_framing_overflow_is_a_typed_fail_closed_denial() {
        let prior = WorthQueryPortablePackageReconstructionWork {
            logical_bytes: u64::MAX,
            nested_entries: 0,
            canonical_work_bytes: 0,
        };
        let record = WorthQueryPortablePackageRecord::DomainIdentity(
            WorthQueryPortableDomainIdentity::new("overflow", 1, 0),
        );
        let unbounded_test_limits = WorthQueryPortablePackageReconstructionLimits::DEFAULT
            .with_work_bounds(u64::MAX, u64::MAX, u64::MAX);

        assert_eq!(
            prior.observe_record(&record, unbounded_test_limits),
            Err(Denial::WorkObservationOverflow)
        );
    }
}
