use worth_foundational::facade::{AspectValuePosture, ScalarAspectType};
use worth_query::facade::{
    domain::WorthQueryNativeSelectionDenial, foundation::ObservationLaneWitness,
    installed::operation,
};

type BoundRequest = operation::WorthQueryBoundProjectionRequest<
    crate::WorthUiDomainEntry,
    crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjection,
    crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjectionFamily,
    ObservationLaneWitness,
>;

pub(crate) enum WorthUiCollectionTextNativeRequestDenial {
    ProjectionRequest(operation::WorthQueryNativeProjectionRequestDenial),
    SelectionMismatch(WorthQueryNativeSelectionDenial),
    NativeFamilyMismatch,
}

pub(crate) struct WorthUiCollectionTextNativeRequest {
    request: BoundRequest,
    accesses: Box<[WorthUiCollectionTextNativeAccess]>,
}

pub(crate) struct WorthUiCollectionTextNativeAccess {
    key: operation::WorthQueryNativeAccessKey,
    resolution_counters: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
}

impl WorthUiCollectionTextNativeRequest {
    pub(crate) fn from_consumer(
        consumer: operation::WorthQueryConsumerProjectionContract<
            crate::WorthUiDomainEntry,
            crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjection,
            crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjectionFamily,
            ObservationLaneWitness,
        >,
        selected_fields: &[crate::UiProjectionFieldRequirement],
    ) -> Result<Self, WorthUiCollectionTextNativeRequestDenial> {
        let mut builder = consumer.projection_request();
        let selections = selected_fields
            .iter()
            .map(|field| {
                builder
                    .select_display_native_field_name(field.native_key())
                    .map_err(WorthUiCollectionTextNativeRequestDenial::ProjectionRequest)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let request = builder
            .build()
            .map_err(WorthUiCollectionTextNativeRequestDenial::ProjectionRequest)?;
        let accesses = selections
            .iter()
            .map(|selection| {
                let resolution = request
                    .resolve_native_key(selection)
                    .map_err(WorthUiCollectionTextNativeRequestDenial::SelectionMismatch)?;
                if resolution.key().expected_shape()
                    != AspectValuePosture::Scalar(ScalarAspectType::String)
                {
                    return Err(WorthUiCollectionTextNativeRequestDenial::NativeFamilyMismatch);
                }
                Ok(WorthUiCollectionTextNativeAccess {
                    resolution_counters: resolution.counters(),
                    key: resolution.into_key(),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { request, accesses })
    }

    pub(crate) fn into_parts(self) -> (BoundRequest, Box<[WorthUiCollectionTextNativeAccess]>) {
        (self.request, self.accesses)
    }
}

impl WorthUiCollectionTextNativeAccess {
    pub(crate) fn key(&self) -> &operation::WorthQueryNativeAccessKey {
        &self.key
    }

    pub(crate) fn resolution_counters(
        &self,
    ) -> worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters {
        self.resolution_counters
    }
}
