use crate::runtime::{WorthUiCompositionRootMountReceipt, WorthUiRuntimeFactId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionRootReconciliationOutcome {
    Preserved,
    Moved,
    Rebound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootReconciliationReceipt {
    prior_root_mount_digest: u64,
    next_root_mount_digest: u64,
    prior_composition_graph_digest: u64,
    next_composition_graph_digest: u64,
    outcome: WorthUiCompositionRootReconciliationOutcome,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiCompositionRootReconciliationReceipt {
    pub fn from_root_mounts(
        prior: &WorthUiCompositionRootMountReceipt,
        next: &WorthUiCompositionRootMountReceipt,
        prior_composition_graph_digest: u64,
        next_composition_graph_digest: u64,
    ) -> Self {
        let outcome = reconcile_outcome(
            prior,
            next,
            prior_composition_graph_digest,
            next_composition_graph_digest,
        );
        let mut consumed_facts = prior.consumed_facts().to_vec();
        consumed_facts.extend(next.consumed_facts().iter().cloned());
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = super::super::digest::digest_parts(
            [
                "composition_root_reconciliation".to_owned(),
                prior.receipt_digest().to_string(),
                next.receipt_digest().to_string(),
                prior_composition_graph_digest.to_string(),
                next_composition_graph_digest.to_string(),
                outcome.token().to_owned(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            prior_root_mount_digest: prior.receipt_digest(),
            next_root_mount_digest: next.receipt_digest(),
            prior_composition_graph_digest,
            next_composition_graph_digest,
            outcome,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn outcome(&self) -> WorthUiCompositionRootReconciliationOutcome {
        self.outcome
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn prior_root_mount_digest(&self) -> u64 {
        self.prior_root_mount_digest
    }

    pub fn next_root_mount_digest(&self) -> u64 {
        self.next_root_mount_digest
    }

    pub fn prior_composition_graph_digest(&self) -> u64 {
        self.prior_composition_graph_digest
    }

    pub fn next_composition_graph_digest(&self) -> u64 {
        self.next_composition_graph_digest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionRootReconciliationOutcome {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Moved => "moved",
            Self::Rebound => "rebound",
        }
    }
}

fn reconcile_outcome(
    prior: &WorthUiCompositionRootMountReceipt,
    next: &WorthUiCompositionRootMountReceipt,
    prior_composition_graph_digest: u64,
    next_composition_graph_digest: u64,
) -> WorthUiCompositionRootReconciliationOutcome {
    if prior.receipt_digest() == next.receipt_digest()
        && prior_composition_graph_digest == next_composition_graph_digest
    {
        return WorthUiCompositionRootReconciliationOutcome::Preserved;
    }
    if prior_composition_graph_digest == next_composition_graph_digest {
        return WorthUiCompositionRootReconciliationOutcome::Moved;
    }
    WorthUiCompositionRootReconciliationOutcome::Rebound
}
