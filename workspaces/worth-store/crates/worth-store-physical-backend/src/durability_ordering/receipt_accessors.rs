use super::*;

macro_rules! impl_receipt_accessors {
    ($type:ident) => {
        impl<S> $type<S> {
            pub const fn profile(&self) -> BackendTargetProfile {
                self.core.profile
            }

            pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
                self.core.evidence_class
            }

            pub const fn requirement(&self) -> StoreDurabilityRequirement {
                self.core.requirement
            }

            pub const fn publication(&self) -> StoreDurabilityPublicationKind {
                self.core.requirement.publication()
            }

            pub const fn completed_barriers(&self) -> WalDurabilityBarrierSet {
                self.core.completed_barriers
            }

            pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
                self.core.counters
            }

            pub const fn scope(&self) -> &S {
                &self.core.scope
            }
        }
    };
}

impl_receipt_accessors!(StoreDurabilityWriteSubmitted);
impl_receipt_accessors!(StoreDurabilityWriteAccepted);
impl_receipt_accessors!(StoreDurabilityBoundaryReached);
impl_receipt_accessors!(StoreDurabilityParentNamespaceDurable);
impl_receipt_accessors!(StoreDurabilityRenameDurable);
impl_receipt_accessors!(StoreDurabilityOrderingBarrierDurable);
