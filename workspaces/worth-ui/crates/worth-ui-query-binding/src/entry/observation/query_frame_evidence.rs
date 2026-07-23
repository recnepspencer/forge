use crate::{WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceOrder};

/// Compact, non-authoritative evidence carried by an ordinary UI frame.
///
/// This receipt is copied from a borrowed retained settlement. It deliberately
/// owns neither the settled projection nor an `Arc` to its derived facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryFrameEvidence {
    evidence_identity_digest: u64,
    source_generation: WorthUiQueryAllocationSourceGeneration,
    source_order: WorthUiQueryAllocationSourceOrder,
    native_fact_count: usize,
    observation_count: usize,
    partial: bool,
}

impl WorthUiQueryFrameEvidence {
    pub(crate) fn from_settled_snapshot(
        reference: &crate::WorthUiInstalledQueryBindingReference,
        fact: &crate::WorthUiSettledSnapshotFact,
    ) -> Self {
        let source_generation = WorthUiQueryAllocationSourceGeneration::from_value(
            fact.source_generation()
                .expect("retained snapshot evidence carries a generation")
                .as_u64(),
        );
        let source_order = WorthUiQueryAllocationSourceOrder::from_value(
            fact.source_order()
                .expect("retained snapshot evidence carries an order")
                .as_u64(),
        );
        let definition_digest = reference.definition().digest().as_u64();
        Self {
            evidence_identity_digest: evidence_identity_digest(
                definition_digest,
                source_generation,
                source_order,
                definition_digest,
            ),
            source_generation,
            source_order,
            native_fact_count: fact.native_fact_count(),
            observation_count: fact
                .measurement_facts()
                .map_or(0, |batch| batch.observations().len()),
            partial: fact.is_partial(),
        }
    }

    pub fn evidence_identity_digest(self) -> u64 {
        self.evidence_identity_digest
    }

    pub fn source_generation(self) -> WorthUiQueryAllocationSourceGeneration {
        self.source_generation
    }

    pub fn source_order(self) -> WorthUiQueryAllocationSourceOrder {
        self.source_order
    }

    pub fn native_fact_count(self) -> usize {
        self.native_fact_count
    }

    pub fn observation_count(self) -> usize {
        self.observation_count
    }

    pub fn is_partial(self) -> bool {
        self.partial
    }
}

fn evidence_identity_digest(
    basis: u64,
    generation: WorthUiQueryAllocationSourceGeneration,
    order: WorthUiQueryAllocationSourceOrder,
    definition_digest: u64,
) -> u64 {
    basis
        ^ generation.as_u64().rotate_left(17)
        ^ order.as_u64().rotate_left(31)
        ^ definition_digest.rotate_left(47)
}
