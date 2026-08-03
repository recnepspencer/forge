use worth_query::facade::{
    domain::WorthQueryNativeSelectionDenial, foundation::ObservationLaneWitness,
    installed::operation,
};

type BoundRequest = operation::WorthQueryBoundProjectionRequest<
    crate::WorthUiDomainEntry,
    crate::WorthUiScalarTextProjection,
    crate::WorthUiScalarTextProjectionFamily,
    ObservationLaneWitness,
>;

pub(crate) enum WorthUiScalarTextNativeRequestDenial {
    ProjectionRequest(operation::WorthQueryNativeProjectionRequestDenial),
    SelectionMismatch(WorthQueryNativeSelectionDenial),
}

pub(crate) struct WorthUiScalarTextNativeRequest {
    request: BoundRequest,
    access: WorthUiScalarTextNativeAccess,
}

pub(crate) struct WorthUiScalarTextNativeAccess {
    key: operation::WorthQueryNativeAccessKey,
    resolution_counters: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
}

impl WorthUiScalarTextNativeRequest {
    pub(crate) fn from_consumer(
        consumer: operation::WorthQueryConsumerProjectionContract<
            crate::WorthUiDomainEntry,
            crate::WorthUiScalarTextProjection,
            crate::WorthUiScalarTextProjectionFamily,
            ObservationLaneWitness,
        >,
        selected_field: &str,
    ) -> Result<Self, WorthUiScalarTextNativeRequestDenial> {
        let mut builder = consumer.projection_request();
        let selection = builder
            .select_display_native_field_name(selected_field)
            .map_err(WorthUiScalarTextNativeRequestDenial::ProjectionRequest)?;
        let request = builder
            .build()
            .map_err(WorthUiScalarTextNativeRequestDenial::ProjectionRequest)?;
        let resolution = request
            .resolve_native_key(&selection)
            .map_err(WorthUiScalarTextNativeRequestDenial::SelectionMismatch)?;
        let resolution_counters = resolution.counters();
        Ok(Self {
            request,
            access: WorthUiScalarTextNativeAccess {
                key: resolution.into_key(),
                resolution_counters,
            },
        })
    }

    pub(crate) fn into_parts(self) -> (BoundRequest, WorthUiScalarTextNativeAccess) {
        (self.request, self.access)
    }
}

impl WorthUiScalarTextNativeAccess {
    pub(crate) fn key(&self) -> &operation::WorthQueryNativeAccessKey {
        &self.key
    }

    pub(crate) fn resolution_counters(
        &self,
    ) -> worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters {
        self.resolution_counters
    }
}
