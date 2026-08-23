/// Query-owned work actually performed after granular impact admission.
///
/// Counts are minted from the performed scoped refresh and the deliveries that
/// publication actually emitted. They carry no admission authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGranularMaintenanceCounters {
    maintenance_operations: usize,
    coalesced_impacts: usize,
    projected_fields: usize,
    prior_field_comparisons: usize,
    membership_rows: usize,
    ordering_keys: usize,
    aggregate_groups: usize,
    window_rows: usize,
    bounded_reexecution_rows: usize,
    explicit_rebinds: usize,
    replacements: usize,
    retirements: usize,
    suppressions: usize,
    authorization_revalidations: usize,
    authorization_denials: usize,
    consumer_publications: usize,
    retained_backpressure_deliveries: usize,
    dropped_backpressure_deliveries: usize,
    terminated_backpressure_deliveries: usize,
    debt_backpressure_deliveries: usize,
}

impl WorthQueryGranularMaintenanceCounters {
    pub const fn maintenance_operations(self) -> usize {
        self.maintenance_operations
    }
    pub const fn coalesced_impacts(self) -> usize {
        self.coalesced_impacts
    }
    pub const fn projected_fields(self) -> usize {
        self.projected_fields
    }
    pub const fn prior_field_comparisons(self) -> usize {
        self.prior_field_comparisons
    }
    pub const fn membership_rows(self) -> usize {
        self.membership_rows
    }
    pub const fn ordering_keys(self) -> usize {
        self.ordering_keys
    }
    pub const fn aggregate_groups(self) -> usize {
        self.aggregate_groups
    }
    pub const fn window_rows(self) -> usize {
        self.window_rows
    }
    pub const fn bounded_reexecution_rows(self) -> usize {
        self.bounded_reexecution_rows
    }
    pub const fn explicit_rebinds(self) -> usize {
        self.explicit_rebinds
    }
    pub const fn replacements(self) -> usize {
        self.replacements
    }
    pub const fn retirements(self) -> usize {
        self.retirements
    }
    pub const fn suppressions(self) -> usize {
        self.suppressions
    }
    pub const fn authorization_revalidations(self) -> usize {
        self.authorization_revalidations
    }
    pub const fn authorization_denials(self) -> usize {
        self.authorization_denials
    }
    pub const fn consumer_publications(self) -> usize {
        self.consumer_publications
    }
    pub const fn retained_backpressure_deliveries(self) -> usize {
        self.retained_backpressure_deliveries
    }
    pub const fn dropped_backpressure_deliveries(self) -> usize {
        self.dropped_backpressure_deliveries
    }
    pub const fn terminated_backpressure_deliveries(self) -> usize {
        self.terminated_backpressure_deliveries
    }
    pub const fn debt_backpressure_deliveries(self) -> usize {
        self.debt_backpressure_deliveries
    }

    pub(super) fn primary(
        effect: &super::WorthQueryPerformedMaintenanceEffect,
        admitted_impacts: usize,
        publications: usize,
    ) -> Self {
        let mut counters = Self::performed(effect, admitted_impacts);
        counters.authorization_revalidations = publications;
        counters.consumer_publications = publications;
        counters
    }

    pub(super) fn shared(
        effect: &super::WorthQueryPerformedMaintenanceEffect,
        admitted_impacts: usize,
        policies: impl IntoIterator<Item = crate::subscription::DeliveryBackpressurePolicy>,
        authorization_denials: usize,
    ) -> Self {
        let mut counters = Self::performed(effect, admitted_impacts);
        for policy in policies {
            counters.authorization_revalidations += 1;
            counters.consumer_publications += 1;
            match policy {
                crate::subscription::DeliveryBackpressurePolicy::RetainWithinWindow => {
                    counters.retained_backpressure_deliveries += 1;
                }
                crate::subscription::DeliveryBackpressurePolicy::DropWithGapNotice => {
                    counters.dropped_backpressure_deliveries += 1;
                }
                crate::subscription::DeliveryBackpressurePolicy::TerminateConsumer => {
                    counters.terminated_backpressure_deliveries += 1;
                }
                crate::subscription::DeliveryBackpressurePolicy::DebtExplicit => {
                    counters.debt_backpressure_deliveries += 1;
                }
            }
        }
        counters.authorization_denials = authorization_denials;
        counters
    }

    fn performed(
        effect: &super::WorthQueryPerformedMaintenanceEffect,
        admitted_impacts: usize,
    ) -> Self {
        let mut counters = Self {
            maintenance_operations: 1,
            coalesced_impacts: admitted_impacts.saturating_sub(1),
            ..Self::default()
        };
        match effect {
            super::WorthQueryPerformedMaintenanceEffect::ProjectionPatch(patch) => {
                counters.projected_fields = patch.fields().len();
                counters.prior_field_comparisons = patch.prior_field_comparisons();
            }
            super::WorthQueryPerformedMaintenanceEffect::IndexedLivePatch(patch) => {
                let work = patch.work();
                let collection = patch.collection_work();
                counters.projected_fields = patch.fields().len();
                counters.prior_field_comparisons = work.prior_field_comparisons();
                counters.ordering_keys = collection.ordering_index_updates;
                counters.window_rows = collection.fresh_window_rows_visited;
                for operation in patch.operations() {
                    match operation {
                        crate::domain_installation::WorthQueryCollectionPatchOperation::Insert {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::Remove {
                            ..
                        } => {
                            counters.membership_rows += 1;
                        }
                        crate::domain_installation::WorthQueryCollectionPatchOperation::Regroup {
                            ..
                        } => {
                            counters.aggregate_groups += 1;
                        }
                        crate::domain_installation::WorthQueryCollectionPatchOperation::Move { .. }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::Update {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::WindowShift {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::ResultState {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::Warnings {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::Continuation {
                            ..
                        }
                        | crate::domain_installation::WorthQueryCollectionPatchOperation::ResetRequired {
                            ..
                        } => {}
                    }
                }
                counters.bounded_reexecution_rows = usize::from(
                    patch
                        .strategies()
                        .contains(&super::WorthQueryMaintenanceStrategy::BoundedReexecution),
                ) * work.affected_requirement_rows();
                counters.explicit_rebinds = usize::from(
                    patch
                        .strategies()
                        .contains(&super::WorthQueryMaintenanceStrategy::ExplicitRebind),
                );
                counters.replacements = usize::from(
                    patch
                        .strategies()
                        .contains(&super::WorthQueryMaintenanceStrategy::Replacement),
                );
                counters.retirements = usize::from(
                    patch
                        .strategies()
                        .contains(&super::WorthQueryMaintenanceStrategy::Retirement),
                );
                counters.suppressions = usize::from(
                    patch
                        .strategies()
                        .contains(&super::WorthQueryMaintenanceStrategy::Suppression),
                );
            }
        }
        counters
    }
}
