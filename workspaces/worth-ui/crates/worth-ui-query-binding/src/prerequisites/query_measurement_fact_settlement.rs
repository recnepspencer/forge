use std::sync::Arc;
use worth_query::facade::{ProjectionConsumptionWarningKind, ProjectionFactConsumptionAttempt};

use super::{
    WorthUiQueryMeasurementFactReceipt, WorthUiQueryMeasurementFactReceiptError,
    WorthUiQueryPrerequisiteEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactSettlementDenial {
    Denied,
    Deferred,
    SourceMismatch,
    Receipt(WorthUiQueryMeasurementFactReceiptError),
    SourceOrderExhausted,
    SourceGenerationExhausted,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationSourceIdentity(Arc<str>);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationSourceGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationSourceOrder(u64);

/// Query-owned projection facts after Worth UI binding has preserved settlement posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactSettlement {
    representation: WorthUiQueryMeasurementFactSettlementRepresentation,
    source_coordinates: WorthUiQueryAllocationSourceCoordinates,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthUiQueryAllocationSourceCoordinates {
    identity: WorthUiQueryAllocationSourceIdentity,
    generation: WorthUiQueryAllocationSourceGeneration,
    order: WorthUiQueryAllocationSourceOrder,
}

#[derive(Debug)]
pub(crate) struct WorthUiQueryAllocationSourceAuthority {
    next_order: u64,
    generation: u64,
    basis_digest: Option<Box<str>>,
    canonical_identity: Option<WorthUiQueryAllocationSourceIdentity>,
}

impl Default for WorthUiQueryAllocationSourceAuthority {
    fn default() -> Self {
        Self {
            next_order: 1,
            generation: 0,
            basis_digest: None,
            canonical_identity: None,
        }
    }
}

impl WorthUiQueryAllocationSourceAuthority {
    pub(crate) fn admit(
        &mut self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        attempt: &ProjectionFactConsumptionAttempt,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        let admitted_basis: Box<str> = prerequisites
            .resolution_report()
            .basis_digest()
            .as_str()
            .into();
        let generation = if self.basis_digest.as_deref() == Some(admitted_basis.as_ref()) {
            self.generation
        } else {
            self.generation
                .checked_add(1)
                .ok_or(WorthUiQueryMeasurementFactSettlementDenial::SourceGenerationExhausted)?
        };
        let order = self.next_order;
        let identity = settlement_source_identity(attempt)
            .ok_or(WorthUiQueryMeasurementFactSettlementDenial::SourceMismatch)?;
        let canonical_identity = self.canonical_identity(identity);
        let next_order = self
            .next_order
            .checked_add(1)
            .ok_or(WorthUiQueryMeasurementFactSettlementDenial::SourceOrderExhausted)?;
        let settlement =
            WorthUiQueryMeasurementFactSettlement::from_projection_consumption_attempt(
                prerequisites,
                attempt,
                WorthUiQueryAllocationSourceCoordinates {
                    identity: canonical_identity.clone(),
                    generation: WorthUiQueryAllocationSourceGeneration(generation),
                    order: WorthUiQueryAllocationSourceOrder(order),
                },
            )?;
        if self.basis_digest.as_deref() != Some(admitted_basis.as_ref()) {
            self.generation = generation;
            self.basis_digest = Some(admitted_basis);
        }
        self.next_order = next_order;
        if self.canonical_identity.as_ref() != Some(&canonical_identity) {
            self.canonical_identity = Some(canonical_identity);
        }
        Ok(settlement)
    }

    fn canonical_identity(&self, identity: &str) -> WorthUiQueryAllocationSourceIdentity {
        match self.canonical_identity.as_ref() {
            Some(canonical) if canonical.as_str() == identity => canonical.clone(),
            _ => WorthUiQueryAllocationSourceIdentity(Arc::from(identity)),
        }
    }
}

fn settlement_source_identity(attempt: &ProjectionFactConsumptionAttempt) -> Option<&str> {
    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed)
        | ProjectionFactConsumptionAttempt::AdmittedWithWarnings(completed, _) => {
            Some(completed.receipt().source_identity())
        }
        ProjectionFactConsumptionAttempt::Denied(_)
        | ProjectionFactConsumptionAttempt::Deferred(_)
        | ProjectionFactConsumptionAttempt::SourceMismatch(_) => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiQueryMeasurementFactSettlementRepresentation {
    Settled(WorthUiQueryMeasurementFactReceipt),
    Partial {
        receipt: WorthUiQueryMeasurementFactReceipt,
        warning_kinds: Box<[ProjectionConsumptionWarningKind]>,
        warning_digest: Box<str>,
    },
}

impl WorthUiQueryMeasurementFactSettlement {
    pub fn allocation_invalidation_basis(&self) -> super::WorthUiQueryAllocationInvalidationBasis {
        super::WorthUiQueryAllocationInvalidationBasis::from_settlement(self)
    }
    fn from_projection_consumption_attempt(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        attempt: &ProjectionFactConsumptionAttempt,
        source_coordinates: WorthUiQueryAllocationSourceCoordinates,
    ) -> Result<Self, WorthUiQueryMeasurementFactSettlementDenial> {
        match attempt {
            ProjectionFactConsumptionAttempt::Admitted(completed) => {
                let receipt =
                    WorthUiQueryMeasurementFactReceipt::from_completed_projection_consumption(
                        prerequisites,
                        completed,
                    )
                    .map_err(WorthUiQueryMeasurementFactSettlementDenial::Receipt)?;
                Ok(Self {
                    representation: WorthUiQueryMeasurementFactSettlementRepresentation::Settled(
                        receipt,
                    ),
                    source_coordinates,
                })
            }
            ProjectionFactConsumptionAttempt::AdmittedWithWarnings(completed, warnings) => {
                let receipt =
                    WorthUiQueryMeasurementFactReceipt::from_partial_projection_consumption(
                        prerequisites,
                        completed,
                    )
                    .map_err(WorthUiQueryMeasurementFactSettlementDenial::Receipt)?;
                Ok(Self {
                    representation: WorthUiQueryMeasurementFactSettlementRepresentation::Partial {
                        receipt,
                        warning_kinds: warnings.warning_kinds().to_vec().into_boxed_slice(),
                        warning_digest: warnings.warning_digest().into(),
                    },
                    source_coordinates,
                })
            }
            ProjectionFactConsumptionAttempt::Denied(_) => {
                Err(WorthUiQueryMeasurementFactSettlementDenial::Denied)
            }
            ProjectionFactConsumptionAttempt::Deferred(_) => {
                Err(WorthUiQueryMeasurementFactSettlementDenial::Deferred)
            }
            ProjectionFactConsumptionAttempt::SourceMismatch(_) => {
                Err(WorthUiQueryMeasurementFactSettlementDenial::SourceMismatch)
            }
        }
    }

    pub fn receipt(&self) -> &WorthUiQueryMeasurementFactReceipt {
        match &self.representation {
            WorthUiQueryMeasurementFactSettlementRepresentation::Settled(receipt)
            | WorthUiQueryMeasurementFactSettlementRepresentation::Partial { receipt, .. } => {
                receipt
            }
        }
    }

    pub fn is_partial(&self) -> bool {
        matches!(
            self.representation,
            WorthUiQueryMeasurementFactSettlementRepresentation::Partial { .. }
        )
    }

    pub fn warning_kinds(&self) -> &[ProjectionConsumptionWarningKind] {
        match &self.representation {
            WorthUiQueryMeasurementFactSettlementRepresentation::Settled(_) => &[],
            WorthUiQueryMeasurementFactSettlementRepresentation::Partial {
                warning_kinds, ..
            } => warning_kinds,
        }
    }

    pub fn warning_digest(&self) -> Option<&str> {
        match &self.representation {
            WorthUiQueryMeasurementFactSettlementRepresentation::Settled(_) => None,
            WorthUiQueryMeasurementFactSettlementRepresentation::Partial {
                warning_digest, ..
            } => Some(warning_digest),
        }
    }

    pub fn allocation_source_identity(&self) -> &WorthUiQueryAllocationSourceIdentity {
        &self.source_coordinates.identity
    }

    pub fn allocation_source_generation(&self) -> WorthUiQueryAllocationSourceGeneration {
        self.source_coordinates.generation
    }

    pub fn allocation_source_order(&self) -> WorthUiQueryAllocationSourceOrder {
        self.source_coordinates.order
    }

    pub fn allocation_ingress_identity(&self) -> u64 {
        let mut digest = stable_coordinate_digest(self.source_coordinates.identity.as_str());
        digest ^= self.source_coordinates.generation.as_u64().rotate_left(11);
        digest ^= self.source_coordinates.order.as_u64().rotate_left(23);
        digest ^= stable_coordinate_digest(self.receipt().projection_consumption_receipt_digest())
            .rotate_left(37);
        digest
    }
}

fn stable_coordinate_digest(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        })
}

impl WorthUiQueryAllocationSourceIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl WorthUiQueryAllocationSourceGeneration {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl WorthUiQueryAllocationSourceOrder {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}
