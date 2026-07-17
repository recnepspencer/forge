#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiScrollBindingCatalogCounters {
    pub(crate) context_reads: u16,
    pub(crate) target_probes: u16,
    pub(crate) bindings_sealed: u16,
    pub(crate) duplicate_probes: u16,
    pub(crate) structural_comparisons: u16,
    pub(crate) source_visits: u16,
    pub(crate) receipt_validations: u16,
    pub(crate) owner_validations: u16,
    pub(crate) graph_target_validations: u16,
    pub(crate) index_writes: u16,
    pub(crate) projection_writes: u16,
    pub(crate) extent_rows_visited: u16,
    pub(crate) targets_emitted: u16,
    pub(crate) freeze_operations: u16,
    pub(crate) diagnostic_probes: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollCatalogIdentity {
    committed_binding_identity_digest: u64,
    predecessor_identity_digest: u64,
    successor_identity_digest: u64,
    activation_keys: Box<[crate::runtime::UiScrollReceiptActivationKey]>,
}

impl UiScrollCatalogIdentity {
    pub(super) fn new(
        committed_binding_identity_digest: u64,
        predecessor_identity_digest: u64,
        successor_identity_digest: u64,
        activation_keys: Box<[crate::runtime::UiScrollReceiptActivationKey]>,
    ) -> Self {
        Self {
            committed_binding_identity_digest,
            predecessor_identity_digest,
            successor_identity_digest,
            activation_keys,
        }
    }
    #[cfg(test)]
    pub(crate) fn activation_keys(&self) -> &[crate::runtime::UiScrollReceiptActivationKey] {
        &self.activation_keys
    }
    pub fn committed_binding_identity_digest(&self) -> u64 {
        self.committed_binding_identity_digest
    }
    pub fn predecessor_identity_digest(&self) -> u64 {
        self.predecessor_identity_digest
    }
    pub fn successor_identity_digest(&self) -> u64 {
        self.successor_identity_digest
    }
    pub fn identity_digest(&self) -> u64 {
        self.activation_keys.iter().fold(
            self.committed_binding_identity_digest
                ^ self.predecessor_identity_digest.rotate_left(11)
                ^ self.successor_identity_digest.rotate_left(29),
            |digest, key| digest.rotate_left(7) ^ key.identity_digest(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScrollOwnerCatalogReceipt {
    counters: UiScrollBindingCatalogCounters,
    owner_count: u16,
    identity: UiScrollCatalogIdentity,
    virtualization: crate::runtime::UiScrollVirtualizationPosture,
    offset_allocation: crate::runtime::UiScrollOffsetAllocationPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScrollOwnerCatalogDenialReport {
    reason: super::UiScrollInvalidationBindingDenial,
    counters: UiScrollBindingCatalogCounters,
    attempted_identity: UiScrollCatalogIdentity,
}

impl UiScrollOwnerCatalogDenialReport {
    pub(super) fn new(
        reason: super::UiScrollInvalidationBindingDenial,
        counters: UiScrollBindingCatalogCounters,
        attempted_identity: UiScrollCatalogIdentity,
    ) -> Self {
        Self {
            reason,
            counters,
            attempted_identity,
        }
    }
    pub fn reason(&self) -> super::UiScrollInvalidationBindingDenial {
        self.reason
    }
    pub fn counters(&self) -> UiScrollBindingCatalogCounters {
        self.counters
    }
    pub fn committed_binding_identity_digest(&self) -> u64 {
        self.attempted_identity.committed_binding_identity_digest()
    }
    pub fn attempted_catalog_identity_digest(&self) -> u64 {
        self.attempted_identity.identity_digest()
    }
    pub fn predecessor_identity_digest(&self) -> u64 {
        self.attempted_identity.predecessor_identity_digest()
    }
    pub fn successor_identity_digest(&self) -> u64 {
        self.attempted_identity.successor_identity_digest()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiScrollCatalogSwapEvidence {
    Prepared(UiScrollOwnerCatalogReceipt),
    Denied(UiScrollOwnerCatalogDenialReport),
}
impl UiScrollOwnerCatalogReceipt {
    pub(super) fn seal(
        counters: UiScrollBindingCatalogCounters,
        owner_count: usize,
        identity: UiScrollCatalogIdentity,
    ) -> Result<Self, super::UiScrollInvalidationBindingDenial> {
        Ok(Self {
            counters,
            owner_count: u16::try_from(owner_count)
                .map_err(|_| super::UiScrollInvalidationBindingDenial::AuthorityCounterExhausted)?,
            identity,
            virtualization: crate::runtime::UiScrollVirtualizationPosture::NonVirtualized,
            offset_allocation:
                crate::runtime::UiScrollOffsetAllocationPosture::ProjectedInteractionOnly,
        })
    }
    pub fn counters(&self) -> UiScrollBindingCatalogCounters {
        self.counters
    }
    pub fn owner_count(&self) -> u16 {
        self.owner_count
    }
    pub fn catalog_identity_digest(&self) -> u64 {
        self.identity.identity_digest()
    }
    pub fn committed_binding_identity_digest(&self) -> u64 {
        self.identity.committed_binding_identity_digest()
    }
    pub fn virtualization(&self) -> crate::runtime::UiScrollVirtualizationPosture {
        self.virtualization
    }
    pub fn offset_allocation(&self) -> crate::runtime::UiScrollOffsetAllocationPosture {
        self.offset_allocation
    }
    pub fn predecessor_identity_digest(&self) -> u64 {
        self.identity.predecessor_identity_digest()
    }
    pub fn successor_identity_digest(&self) -> u64 {
        self.identity.successor_identity_digest()
    }
    #[cfg(test)]
    pub(crate) fn identity(&self) -> &UiScrollCatalogIdentity {
        &self.identity
    }
}

impl UiScrollBindingCatalogCounters {
    pub fn context_reads(self) -> u16 {
        self.context_reads
    }
    pub fn target_probes(self) -> u16 {
        self.target_probes
    }
    pub fn bindings_sealed(self) -> u16 {
        self.bindings_sealed
    }
    pub fn duplicate_probes(self) -> u16 {
        self.duplicate_probes
    }
    pub fn structural_comparisons(self) -> u16 {
        self.structural_comparisons
    }
    pub fn source_visits(self) -> u16 {
        self.source_visits
    }
    pub fn receipt_validations(self) -> u16 {
        self.receipt_validations
    }
    pub fn owner_validations(self) -> u16 {
        self.owner_validations
    }
    pub fn graph_target_validations(self) -> u16 {
        self.graph_target_validations
    }
    pub fn index_writes(self) -> u16 {
        self.index_writes
    }
    pub fn projection_writes(self) -> u16 {
        self.projection_writes
    }
    pub fn extent_rows_visited(self) -> u16 {
        self.extent_rows_visited
    }
    pub fn targets_emitted(self) -> u16 {
        self.targets_emitted
    }
    pub fn freeze_operations(self) -> u16 {
        self.freeze_operations
    }
    pub fn diagnostic_probes(self) -> u16 {
        self.diagnostic_probes
    }
}

pub(super) fn bump(counter: &mut u16) -> Result<(), super::UiScrollInvalidationBindingDenial> {
    *counter = counter
        .checked_add(1)
        .ok_or(super::UiScrollInvalidationBindingDenial::AuthorityCounterExhausted)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn scroll_authority_counter_exhaustion_is_typed() {
        let mut counter = u16::MAX;
        assert_eq!(
            super::bump(&mut counter),
            Err(super::super::UiScrollInvalidationBindingDenial::AuthorityCounterExhausted)
        );
    }
}
