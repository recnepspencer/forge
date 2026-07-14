use worth_store_budgets::{AllocationCounterSnapshot, AllocationScope};
use worth_store_buffer_pool::{
    AllocationAdmission, AllocationDenial, AllocationReceipt, FixedMetadataGrant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationEnvelopeEvidenceReport {
    row: AllocationEnvelopeEvidenceRow,
    counters: AllocationCounterSnapshot,
}

impl AllocationEnvelopeEvidenceReport {
    pub fn from_admission(
        row: AllocationEnvelopeEvidenceRow,
        admission: &AllocationAdmission,
    ) -> Result<Self, AllocationEnvelopeEvidenceDenial> {
        let counters = admission.counters();
        row.prove_counters(counters)?;
        Ok(Self { row, counters })
    }

    pub fn from_receipt(
        row: AllocationEnvelopeEvidenceRow,
        receipt: AllocationReceipt,
    ) -> Result<Self, AllocationEnvelopeEvidenceDenial> {
        let counters = receipt.counters();
        row.prove_counters(counters)?;
        Ok(Self { row, counters })
    }

    pub fn from_denial(
        row: AllocationEnvelopeEvidenceRow,
        denial: AllocationDenial,
        counters: AllocationCounterSnapshot,
    ) -> Result<Self, AllocationEnvelopeEvidenceDenial> {
        row.prove_denial(denial, counters)?;
        Ok(Self { row, counters })
    }

    pub fn from_fixed_metadata(
        admission: &AllocationAdmission,
        grant: &FixedMetadataGrant,
    ) -> Result<Self, AllocationEnvelopeEvidenceDenial> {
        let counters = admission.counters();
        if grant.bytes() == 0
            || !admission.owns_fixed_metadata_grant(grant)
            || grant.constant_size_at_scale(1, 1, 1, 1)
                != grant.constant_size_at_scale(u64::MAX, u64::MAX, u64::MAX, u64::MAX)
            || counters.fixed_metadata_bytes() == 0
            || counters.fixed_metadata_exemption_count() == 0
        {
            return Err(AllocationEnvelopeEvidenceDenial::UnprovenAllocationRow);
        }
        Ok(Self {
            row: AllocationEnvelopeEvidenceRow::FixedMetadataExemptionConstantAndCounted,
            counters,
        })
    }

    pub const fn row(self) -> AllocationEnvelopeEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> AllocationCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationEnvelopeEvidenceRow {
    SeparateScopesAdmittedAndCounted,
    AllocationDeniedBeforeMaterialization,
    ForegroundEnvelopeNotStolenByBackground,
    FixedMetadataExemptionConstantAndCounted,
    AllocationEvidenceDescriptiveOnly,
}

impl AllocationEnvelopeEvidenceRow {
    fn prove_counters(
        self,
        counters: AllocationCounterSnapshot,
    ) -> Result<(), AllocationEnvelopeEvidenceDenial> {
        match self {
            Self::SeparateScopesAdmittedAndCounted
                if AllocationScope::ALL
                    .iter()
                    .all(|scope| counters.scope(*scope).admitted_bytes() > 0)
                    && AllocationScope::ALL
                        .iter()
                        .all(|scope| counters.scope(*scope).allocated_bytes() > 0) =>
            {
                Ok(())
            }
            Self::AllocationEvidenceDescriptiveOnly
                if AllocationScope::ALL
                    .iter()
                    .any(|scope| counters.scope(*scope).requested_bytes() > 0)
                    || counters.fixed_metadata_bytes() > 0 =>
            {
                Ok(())
            }
            Self::SeparateScopesAdmittedAndCounted | Self::AllocationEvidenceDescriptiveOnly => {
                Err(AllocationEnvelopeEvidenceDenial::UnprovenAllocationRow)
            }
            Self::AllocationDeniedBeforeMaterialization
            | Self::ForegroundEnvelopeNotStolenByBackground
            | Self::FixedMetadataExemptionConstantAndCounted => {
                Err(AllocationEnvelopeEvidenceDenial::WrongRow)
            }
        }
    }

    fn prove_denial(
        self,
        denial: AllocationDenial,
        counters: AllocationCounterSnapshot,
    ) -> Result<(), AllocationEnvelopeEvidenceDenial> {
        match self {
            Self::AllocationDeniedBeforeMaterialization => {
                let scope = denial
                    .scope()
                    .ok_or(AllocationEnvelopeEvidenceDenial::DenialMismatch)?;
                let scope_counters = counters.scope(scope);
                if scope_counters.denial_count() > 0 && scope_counters.allocated_bytes() == 0 {
                    Ok(())
                } else {
                    Err(AllocationEnvelopeEvidenceDenial::UnprovenAllocationRow)
                }
            }
            Self::ForegroundEnvelopeNotStolenByBackground => match denial.scope() {
                Some(AllocationScope::Maintenance | AllocationScope::Scrub)
                    if counters.scope(AllocationScope::Foreground).admitted_bytes() == 0
                        && counters
                            .scope(AllocationScope::Foreground)
                            .allocated_bytes()
                            == 0 =>
                {
                    Ok(())
                }
                _ => Err(AllocationEnvelopeEvidenceDenial::DenialMismatch),
            },
            Self::SeparateScopesAdmittedAndCounted
            | Self::FixedMetadataExemptionConstantAndCounted
            | Self::AllocationEvidenceDescriptiveOnly => {
                Err(AllocationEnvelopeEvidenceDenial::WrongRow)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationEnvelopeEvidenceDenial {
    WrongRow,
    DenialMismatch,
    UnprovenAllocationRow,
}
