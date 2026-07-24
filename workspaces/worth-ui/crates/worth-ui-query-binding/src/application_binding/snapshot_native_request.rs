use worth_query::facade::{foundation::ObservationLaneWitness, installed::operation};

type BoundRequest = operation::WorthQueryBoundProjectionRequest<
    crate::WorthUiDomainEntry,
    crate::WorthUiSnapshotMeasurement,
    crate::WorthUiSnapshotMeasurementFamily,
    ObservationLaneWitness,
>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiNativeKeyResolutionCounters {
    declaration_checks: usize,
    indexed_slot_lookups: usize,
    path_matches: usize,
    key_scans: usize,
    path_parses: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiSnapshotNativeRequestDenial {
    ProjectionRequest(operation::WorthQueryNativeProjectionRequestDenial),
    SelectionMismatch(WorthUiNativeKeyResolutionCounters),
}

pub(crate) struct WorthUiSnapshotNativeRequest {
    request: BoundRequest,
    access: WorthUiSnapshotNativeAccess,
}

pub(crate) struct WorthUiSnapshotNativeAccess {
    key: operation::WorthQueryNativeAccessKey,
    resolution_counters: WorthUiNativeKeyResolutionCounters,
}

impl WorthUiSnapshotNativeRequest {
    pub(crate) fn from_consumer(
        consumer: operation::WorthQueryConsumerBoundary<
            crate::WorthUiDomainEntry,
            crate::WorthUiSnapshotMeasurement,
            crate::WorthUiSnapshotMeasurementFamily,
            ObservationLaneWitness,
        >,
    ) -> Result<Self, WorthUiSnapshotNativeRequestDenial> {
        let mut builder = consumer.into_query_contract().projection_request();
        let selection = builder
            .select_display_native_field_name("value")
            .map_err(WorthUiSnapshotNativeRequestDenial::ProjectionRequest)?;
        let request = builder
            .build()
            .map_err(WorthUiSnapshotNativeRequestDenial::ProjectionRequest)?;
        let resolution = request.resolve_native_key(&selection).map_err(|denial| {
            let counters = denial.counters();
            WorthUiSnapshotNativeRequestDenial::SelectionMismatch(
                WorthUiNativeKeyResolutionCounters::from_query(counters),
            )
        })?;
        let resolution_counters =
            WorthUiNativeKeyResolutionCounters::from_query(resolution.counters());
        Ok(Self {
            request,
            access: WorthUiSnapshotNativeAccess {
                key: resolution.into_key(),
                resolution_counters,
            },
        })
    }

    pub(crate) fn into_parts(self) -> (BoundRequest, WorthUiSnapshotNativeAccess) {
        (self.request, self.access)
    }
}

impl WorthUiSnapshotNativeAccess {
    pub(crate) fn key(&self) -> &operation::WorthQueryNativeAccessKey {
        &self.key
    }

    pub(crate) fn resolution_counters(&self) -> WorthUiNativeKeyResolutionCounters {
        self.resolution_counters
    }
}

impl WorthUiNativeKeyResolutionCounters {
    const fn from_query(
        counters: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
    ) -> Self {
        Self {
            declaration_checks: counters.declaration_checks,
            indexed_slot_lookups: counters.indexed_slot_lookups,
            path_matches: counters.path_matches,
            key_scans: counters.key_scans,
            path_parses: counters.path_parses,
        }
    }

    pub fn declaration_checks(self) -> usize {
        self.declaration_checks
    }

    pub fn indexed_slot_lookups(self) -> usize {
        self.indexed_slot_lookups
    }

    pub fn path_matches(self) -> usize {
        self.path_matches
    }

    pub fn key_scans(self) -> usize {
        self.key_scans
    }

    pub fn path_parses(self) -> usize {
        self.path_parses
    }
}
