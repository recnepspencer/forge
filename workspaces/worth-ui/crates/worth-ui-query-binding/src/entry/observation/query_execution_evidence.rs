use std::sync::Arc;

use worth_query::facade::foundation::ConsumedNativeValueView;

use crate::{
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceOrder,
    WorthUiQueryMeasurementFactObservation,
};

/// Read-only sharing of one exact Query-owned settlement at the UI plan edge.
///
/// This reference deliberately exposes native values and compact coordinates,
/// but no method can recover the retained consumed-projection authority.
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
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => fact
                .measurement_facts()
                .map_or(&[], |facts| facts.observations()),
        }
    }

    pub fn native_fact_count(&self) -> usize {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.native_fact_count()
            }
        }
    }

    pub fn native_fact(&self, index: usize) -> Option<ConsumedNativeValueView<'_>> {
        match &self.representation {
            WorthUiQueryViewExecutionEvidenceRepresentation::SettledSnapshot { fact, .. } => {
                fact.native_fact(index)
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
