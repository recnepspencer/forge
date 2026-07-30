use worth_query::facade::{
    foundation::ObservationLaneWitness,
    installed::{self, operation},
};

use super::{
    WorthUiBoundCollectionTextProjection, WorthUiCollectionTextNativeRequest,
    WorthUiCollectionTextNativeRequestDenial, WorthUiCollectionTextOperatingWorldGateway,
    WorthUiInstalledCollectionTextOperationReference,
};

pub(crate) enum WorthUiCollectionTextConsumerPreparationDenial {
    Binding(Box<installed::WorthQueryOperationBindingDenial>),
    ConsumerContract(operation::WorthQueryConsumerProjectionContractDenial),
    ProjectionShapeMismatch,
    RowIdentityMismatch,
    NativeRequest(WorthUiCollectionTextNativeRequestDenial),
}

pub(crate) struct WorthUiPreparedCollectionTextConsumer {
    reference: WorthUiInstalledCollectionTextOperationReference,
    bound: WorthUiBoundCollectionTextProjection<ObservationLaneWitness>,
    native_request: WorthUiCollectionTextNativeRequest,
}

impl WorthUiCollectionTextOperatingWorldGateway<'_> {
    pub(crate) fn prepare_consumer(
        self,
        requirement: &crate::UiCollectionSchemaRequirement,
    ) -> Result<WorthUiPreparedCollectionTextConsumer, WorthUiCollectionTextConsumerPreparationDenial>
    {
        let (reference, bound) = self
            .bind()
            .map_err(WorthUiCollectionTextConsumerPreparationDenial::Binding)?;
        let consumer = bound
            .consumer_projection_contract()
            .map_err(WorthUiCollectionTextConsumerPreparationDenial::ConsumerContract)?;
        validate_collection_contract(&consumer, requirement)?;
        let native_request = WorthUiCollectionTextNativeRequest::from_consumer(
            consumer,
            requirement.selected_fields(),
        )
        .map_err(WorthUiCollectionTextConsumerPreparationDenial::NativeRequest)?;
        Ok(WorthUiPreparedCollectionTextConsumer {
            reference,
            bound,
            native_request,
        })
    }
}

fn validate_collection_contract(
    consumer: &operation::WorthQueryConsumerProjectionContract<
        crate::WorthUiDomainEntry,
        crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjection,
        crate::installed_domain::collection_text_projection::WorthUiCollectionTextProjectionFamily,
        ObservationLaneWitness,
    >,
    requirement: &crate::UiCollectionSchemaRequirement,
) -> Result<(), WorthUiCollectionTextConsumerPreparationDenial> {
    let worth_query::facade::domain::WorthQueryOperationCollectionContract::Collection {
        row_identity_field,
        ..
    } = consumer.collection()
    else {
        return Err(WorthUiCollectionTextConsumerPreparationDenial::ProjectionShapeMismatch);
    };
    let declared = worth_query::facade::domain::WorthQueryOperationCollectionField::from_dotted(
        requirement.row_identity_field().declared_name(),
    )
    .ok_or(WorthUiCollectionTextConsumerPreparationDenial::RowIdentityMismatch)?;
    if &declared != row_identity_field {
        return Err(WorthUiCollectionTextConsumerPreparationDenial::RowIdentityMismatch);
    }
    Ok(())
}

impl WorthUiPreparedCollectionTextConsumer {
    pub(crate) fn binding_identity_for_reporting(&self) -> &str {
        self.bound.binding_identity()
    }

    pub(crate) fn replacement_witness_for(
        &self,
        candidate: &Self,
    ) -> Result<
        worth_query::facade::domain::WorthQueryReplacementWitness,
        worth_query::facade::domain::WorthQueryReplacementDenial,
    > {
        self.bound.replacement_with(&candidate.bound)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiInstalledCollectionTextOperationReference,
        WorthUiBoundCollectionTextProjection<ObservationLaneWitness>,
        WorthUiCollectionTextNativeRequest,
    ) {
        (self.reference, self.bound, self.native_request)
    }
}
