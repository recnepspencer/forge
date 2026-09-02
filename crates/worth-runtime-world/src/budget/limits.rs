use std::num::NonZeroUsize;

use super::{RuntimeWorldBudgetDenial, RuntimeWorldBudgetResource};

/// A nonzero installed limit. It has no usage counter and cannot be used as a
/// proof that capacity is currently available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeWorldBudgetLimit(NonZeroUsize);

impl RuntimeWorldBudgetLimit {
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

/// All bounded Runtime World populations are installed together.
///
/// There is intentionally no `Default`: omitting a bound must be a compile- or
/// construction-time decision at the composition root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeWorldBudgets {
    live_product_branches: RuntimeWorldBudgetLimit,
    retained_composite_commits: RuntimeWorldBudgetLimit,
    history_metadata_bytes: RuntimeWorldBudgetLimit,
    active_observations: RuntimeWorldBudgetLimit,
    active_publication_attempts: RuntimeWorldBudgetLimit,
    retained_product_unpublished_records: RuntimeWorldBudgetLimit,
    retained_partial_metadata_bytes: RuntimeWorldBudgetLimit,
    unique_exact_component_pins: RuntimeWorldBudgetLimit,
    in_flight_pin_acquisition_reservations: RuntimeWorldBudgetLimit,
    owner_created_component_custody_records: RuntimeWorldBudgetLimit,
}

impl RuntimeWorldBudgets {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        live_product_branches: u64,
        retained_composite_commits: u64,
        history_metadata_bytes: u64,
        active_observations: u64,
        active_publication_attempts: u64,
        retained_product_unpublished_records: u64,
        retained_partial_metadata_bytes: u64,
        unique_exact_component_pins: u64,
        in_flight_pin_acquisition_reservations: u64,
        owner_created_component_custody_records: u64,
    ) -> Result<Self, RuntimeWorldBudgetDenial> {
        Ok(Self {
            live_product_branches: limit(
                RuntimeWorldBudgetResource::LiveProductBranches,
                live_product_branches,
            )?,
            retained_composite_commits: limit(
                RuntimeWorldBudgetResource::RetainedCompositeCommits,
                retained_composite_commits,
            )?,
            history_metadata_bytes: limit(
                RuntimeWorldBudgetResource::HistoryMetadataBytes,
                history_metadata_bytes,
            )?,
            active_observations: limit(
                RuntimeWorldBudgetResource::ActiveObservations,
                active_observations,
            )?,
            active_publication_attempts: limit(
                RuntimeWorldBudgetResource::ActivePublicationAttempts,
                active_publication_attempts,
            )?,
            retained_product_unpublished_records: limit(
                RuntimeWorldBudgetResource::RetainedProductUnpublishedRecords,
                retained_product_unpublished_records,
            )?,
            retained_partial_metadata_bytes: limit(
                RuntimeWorldBudgetResource::RetainedPartialMetadataBytes,
                retained_partial_metadata_bytes,
            )?,
            unique_exact_component_pins: limit(
                RuntimeWorldBudgetResource::UniqueExactComponentPins,
                unique_exact_component_pins,
            )?,
            in_flight_pin_acquisition_reservations: limit(
                RuntimeWorldBudgetResource::InFlightPinAcquisitionReservations,
                in_flight_pin_acquisition_reservations,
            )?,
            owner_created_component_custody_records: limit(
                RuntimeWorldBudgetResource::OwnerCreatedComponentCustodyRecords,
                owner_created_component_custody_records,
            )?,
        })
    }

    pub const fn live_product_branches(&self) -> RuntimeWorldBudgetLimit {
        self.live_product_branches
    }

    pub const fn retained_composite_commits(&self) -> RuntimeWorldBudgetLimit {
        self.retained_composite_commits
    }

    pub const fn history_metadata_bytes(&self) -> RuntimeWorldBudgetLimit {
        self.history_metadata_bytes
    }

    pub const fn active_observations(&self) -> RuntimeWorldBudgetLimit {
        self.active_observations
    }

    pub const fn active_publication_attempts(&self) -> RuntimeWorldBudgetLimit {
        self.active_publication_attempts
    }

    pub const fn retained_product_unpublished_records(&self) -> RuntimeWorldBudgetLimit {
        self.retained_product_unpublished_records
    }

    pub const fn retained_partial_metadata_bytes(&self) -> RuntimeWorldBudgetLimit {
        self.retained_partial_metadata_bytes
    }

    pub const fn unique_exact_component_pins(&self) -> RuntimeWorldBudgetLimit {
        self.unique_exact_component_pins
    }

    pub const fn in_flight_pin_acquisition_reservations(&self) -> RuntimeWorldBudgetLimit {
        self.in_flight_pin_acquisition_reservations
    }

    pub const fn owner_created_component_custody_records(&self) -> RuntimeWorldBudgetLimit {
        self.owner_created_component_custody_records
    }
}

fn limit(
    resource: RuntimeWorldBudgetResource,
    value: u64,
) -> Result<RuntimeWorldBudgetLimit, RuntimeWorldBudgetDenial> {
    let value = usize::try_from(value)
        .map_err(|_| RuntimeWorldBudgetDenial::SizeOverflow { resource, value })?;
    let value = NonZeroUsize::new(value).ok_or(RuntimeWorldBudgetDenial::ZeroLimit { resource })?;
    Ok(RuntimeWorldBudgetLimit(value))
}
