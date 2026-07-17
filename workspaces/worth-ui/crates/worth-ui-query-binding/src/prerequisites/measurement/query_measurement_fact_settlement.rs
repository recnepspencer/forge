use std::sync::Arc;
use worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt;
use worth_query::facade::foundation::{
    ProjectionConsumptionWarningKind, ProjectionSourceBasisAuthority,
};
use worth_query::facade::read::{WorthQueryProjectionOutcome, WorthQueryProjectionViolation};

use super::{
    WorthUiQueryAuthorityHandle, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError,
};
use crate::WorthUiQueryViewDefinition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactSettlementDenial {
    Denied,
    Deferred,
    SourceMismatch,
    Unavailable,
    Receipt(WorthUiQueryMeasurementFactReceiptError),
    SourceOrderExhausted,
    SourceGenerationExhausted,
    InstalledAuthorityMismatch,
    UnregisteredView,
    QueryNotInstalled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryProjectionWarningKind {
    QueryContextRowBound,
    PreviewDerivedContext,
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
    definition: WorthUiQueryViewDefinition,
    installed_execution: WorthQueryInstalledDomainExecutionReceipt,
    basis_authority: super::WorthUiQueryBasisAuthority,
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
    basis_authority: Option<ProjectionSourceBasisAuthority>,
    canonical_identity: Option<WorthUiQueryAllocationSourceIdentity>,
}

impl Default for WorthUiQueryAllocationSourceAuthority {
    fn default() -> Self {
        Self {
            next_order: 1,
            generation: 0,
            basis_authority: None,
            canonical_identity: None,
        }
    }
}

impl WorthUiQueryAllocationSourceAuthority {
    pub(crate) fn admit(
        &mut self,
        definition: WorthUiQueryViewDefinition,
        outcome: WorthQueryProjectionOutcome,
        installed_execution: WorthQueryInstalledDomainExecutionReceipt,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        let (query_authority, warnings) =
            WorthUiQueryAuthorityHandle::from_outcome(outcome).map_err(map_authority_denial)?;
        let admitted_basis = query_authority.authority().basis_authority().clone();
        let generation = if self.basis_authority.as_ref() == Some(&admitted_basis) {
            self.generation
        } else {
            self.generation
                .checked_add(1)
                .ok_or(WorthUiQueryMeasurementFactSettlementDenial::SourceGenerationExhausted)?
        };
        let order = self.next_order;
        let identity = query_authority.authority().source_identity().as_str();
        let canonical_identity = self.canonical_identity(identity);
        let next_order = self
            .next_order
            .checked_add(1)
            .ok_or(WorthUiQueryMeasurementFactSettlementDenial::SourceOrderExhausted)?;
        let settlement = WorthUiQueryMeasurementFactSettlement::from_query_authority(
            definition,
            query_authority,
            warnings,
            installed_execution,
            WorthUiQueryAllocationSourceCoordinates {
                identity: canonical_identity.clone(),
                generation: WorthUiQueryAllocationSourceGeneration(generation),
                order: WorthUiQueryAllocationSourceOrder(order),
            },
        )?;
        if self.basis_authority.as_ref() != Some(&admitted_basis) {
            self.generation = generation;
            self.basis_authority = Some(admitted_basis);
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

fn map_authority_denial(
    outcome: WorthQueryProjectionOutcome,
) -> WorthUiQueryMeasurementFactSettlementDenial {
    match outcome {
        WorthQueryProjectionOutcome::Violation(WorthQueryProjectionViolation::SourceMismatch(
            _,
        )) => WorthUiQueryMeasurementFactSettlementDenial::SourceMismatch,
        WorthQueryProjectionOutcome::Violation(_) => {
            WorthUiQueryMeasurementFactSettlementDenial::Denied
        }
        WorthQueryProjectionOutcome::Deferred(_) => {
            WorthUiQueryMeasurementFactSettlementDenial::Deferred
        }
        WorthQueryProjectionOutcome::Unavailable(_) => {
            WorthUiQueryMeasurementFactSettlementDenial::Unavailable
        }
        WorthQueryProjectionOutcome::Completed(_) | WorthQueryProjectionOutcome::Advisory(_) => {
            unreachable!()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiQueryMeasurementFactSettlementRepresentation {
    Settled(WorthUiQueryMeasurementFactReceipt),
    Partial {
        receipt: WorthUiQueryMeasurementFactReceipt,
        warning_kinds: Box<[WorthUiQueryProjectionWarningKind]>,
        warning_digest: Box<str>,
    },
}

impl WorthUiQueryMeasurementFactSettlement {
    pub fn allocation_invalidation_basis(&self) -> super::WorthUiQueryAllocationInvalidationBasis {
        super::WorthUiQueryAllocationInvalidationBasis::from_settlement(self)
    }
    fn from_query_authority(
        definition: WorthUiQueryViewDefinition,
        query_authority: WorthUiQueryAuthorityHandle,
        warnings: Option<worth_query::facade::foundation::ProjectionConsumptionWarnings>,
        installed_execution: WorthQueryInstalledDomainExecutionReceipt,
        source_coordinates: WorthUiQueryAllocationSourceCoordinates,
    ) -> Result<Self, WorthUiQueryMeasurementFactSettlementDenial> {
        let basis_authority = super::WorthUiQueryBasisAuthority::from_execution(
            query_authority.clone(),
            installed_execution.basis_identity(),
        );
        match warnings {
            None => {
                let receipt = WorthUiQueryMeasurementFactReceipt::from_installed_query_authority(
                    query_authority,
                    false,
                )
                .map_err(WorthUiQueryMeasurementFactSettlementDenial::Receipt)?;
                Ok(Self {
                    representation: WorthUiQueryMeasurementFactSettlementRepresentation::Settled(
                        receipt,
                    ),
                    source_coordinates,
                    definition,
                    installed_execution,
                    basis_authority,
                })
            }
            Some(warnings) => {
                let warning_kinds = warnings
                    .warning_kinds()
                    .iter()
                    .map(|warning| match warning {
                        ProjectionConsumptionWarningKind::QueryContextRowBound => {
                            WorthUiQueryProjectionWarningKind::QueryContextRowBound
                        }
                        ProjectionConsumptionWarningKind::PreviewDerivedContext => {
                            WorthUiQueryProjectionWarningKind::PreviewDerivedContext
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let warning_digest = warnings.warning_digest().into();
                let receipt = WorthUiQueryMeasurementFactReceipt::from_installed_query_authority(
                    query_authority,
                    true,
                )
                .map_err(WorthUiQueryMeasurementFactSettlementDenial::Receipt)?;
                Ok(Self {
                    representation: WorthUiQueryMeasurementFactSettlementRepresentation::Partial {
                        receipt,
                        warning_kinds,
                        warning_digest,
                    },
                    source_coordinates,
                    definition,
                    installed_execution,
                    basis_authority,
                })
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

    pub fn warning_kinds(&self) -> &[WorthUiQueryProjectionWarningKind] {
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

    pub fn resolution_mode(&self) -> super::WorthUiQueryResolutionMode {
        if self
            .receipt()
            .query_authority()
            .authority()
            .basis_authority()
            .snapshot_identity()
            .is_some()
        {
            super::WorthUiQueryResolutionMode::RuntimeDirect
        } else {
            super::WorthUiQueryResolutionMode::StoreDirect
        }
    }

    pub fn basis_authority(&self) -> &super::WorthUiQueryBasisAuthority {
        &self.basis_authority
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn shares_installed_authority_with(&self, other: &Self) -> bool {
        self.installed_execution.installed_authority()
            == other.installed_execution.installed_authority()
    }

    pub fn allocation_ingress_identity(&self) -> u64 {
        self.definition.digest().as_u64()
            ^ self.source_coordinates.generation.as_u64().rotate_left(11)
            ^ self.source_coordinates.order.as_u64().rotate_left(23)
    }
}

impl WorthUiQueryAllocationSourceIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
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
