use std::sync::Arc;

use crate::{
    WorthUiAdmittedQueryBindingReference, WorthUiAdmittedQuerySettlementReference,
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceOrder,
    WorthUiQueryMeasurementFactObservation,
};

/// Read-only sharing of one exact Query-owned settlement at the UI plan edge.
///
/// This reference exposes only UI-owned observations and compact coordinates;
/// no method can recover Query native values or retained projection authority.
#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiQueryViewExecutionEvidenceReference {
    representation: WorthUiQueryViewExecutionEvidenceRepresentation,
}

#[derive(Clone, Debug, PartialEq)]
enum WorthUiQueryViewExecutionEvidenceRepresentation {
    SettledSnapshot {
        reference: crate::WorthUiInstalledQueryBindingReference,
        fact: Arc<crate::WorthUiSettledSnapshotFact>,
    },
}

impl WorthUiQueryViewExecutionEvidenceReference {
    pub(crate) fn from_settled_snapshot(
        reference: crate::WorthUiInstalledQueryBindingReference,
        fact: Arc<crate::WorthUiSettledSnapshotFact>,
    ) -> Self {
        Self {
            representation: WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot {
                reference,
                fact,
            },
        }
    }

    pub fn definition(&self) -> &crate::WorthUiQueryViewDefinition {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot {
                reference, ..
            } => reference.definition(),
        }
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.measurement_facts().observations()
            }
        }
    }

    pub fn binding_reference(&self) -> &WorthUiAdmittedQueryBindingReference {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.binding_reference()
            }
        }
    }

    pub fn settlement_reference(&self) -> &WorthUiAdmittedQuerySettlementReference {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.settlement_reference()
            }
        }
    }

    pub fn source_generation(&self) -> WorthUiQueryAllocationSourceGeneration {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                WorthUiQueryAllocationSourceGeneration::from_value(
                    fact.source_generation()
                        .expect("retained snapshot evidence carries a generation")
                        .as_u64(),
                )
            }
        }
    }

    pub fn source_order(&self) -> WorthUiQueryAllocationSourceOrder {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                WorthUiQueryAllocationSourceOrder::from_value(
                    fact.source_order()
                        .expect("retained snapshot evidence carries an order")
                        .as_u64(),
                )
            }
        }
    }

    pub fn is_partial(&self) -> bool {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.is_partial()
            }
        }
    }

    pub fn evidence_identity_digest(&self) -> u64 {
        let basis = match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot {
                reference, ..
            } => reference.definition().digest().as_u64(),
        };
        basis
            ^ self.source_generation().as_u64().rotate_left(17)
            ^ self.source_order().as_u64().rotate_left(31)
            ^ self.definition().digest().as_u64().rotate_left(47)
    }
}
