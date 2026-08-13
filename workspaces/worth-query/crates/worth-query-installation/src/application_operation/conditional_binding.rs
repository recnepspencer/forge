use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchema,
};

use crate::domain_operation::WorthQueryDomainOperationRef;

type ConditionalOperationMarker<Schema, ApplicationOperation, Input, D, O, F> =
    fn(Input) -> (Schema, ApplicationOperation, D, O, F);

/// Portable package meaning joining one application operation to the exact
/// domain operation that owns its conditional declarations.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryPortableApplicationConditionalOperationBinding {
    schema_owner: String,
    schema_name: String,
    application_operation: String,
    input_type: String,
    domain_operation_slot: String,
    domain_operation_canonical_identity: String,
}

impl WorthQueryPortableApplicationConditionalOperationBinding {
    pub fn schema_owner(&self) -> &str {
        &self.schema_owner
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn application_operation(&self) -> &str {
        &self.application_operation
    }

    pub fn input_type(&self) -> &str {
        &self.input_type
    }

    pub fn domain_operation_slot(&self) -> &str {
        &self.domain_operation_slot
    }

    pub fn domain_operation_canonical_identity(&self) -> &str {
        &self.domain_operation_canonical_identity
    }
}

/// Typed authoring reference for one package-declared application-operation
/// to domain-operation binding.
pub struct WorthQueryApplicationConditionalOperationBinding<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
> {
    portable: WorthQueryPortableApplicationConditionalOperationBinding,
    marker: PhantomData<ConditionalOperationMarker<Schema, ApplicationOperation, Input, D, O, F>>,
}

impl<Schema, ApplicationOperation, Input, D, O, F>
    WorthQueryApplicationConditionalOperationBinding<Schema, ApplicationOperation, Input, D, O, F>
where
    Schema: ApplicationSchema,
{
    pub fn declare(
        application_operation: ApplicationOperationRef<Schema, ApplicationOperation, Input>,
        domain_operation: WorthQueryDomainOperationRef<D, O, F>,
    ) -> Self {
        Self {
            portable: WorthQueryPortableApplicationConditionalOperationBinding {
                schema_owner: Schema::OWNER.to_string(),
                schema_name: Schema::NAME.to_string(),
                application_operation: application_operation.name().to_string(),
                input_type: std::any::type_name::<Input>().to_string(),
                domain_operation_slot: domain_operation.identity().slot(),
                domain_operation_canonical_identity: domain_operation
                    .canonical_identity()
                    .to_string(),
            },
            marker: PhantomData,
        }
    }
}

impl<Schema, ApplicationOperation, Input, D, O, F>
    WorthQueryApplicationConditionalOperationBinding<Schema, ApplicationOperation, Input, D, O, F>
{
    pub fn portable(&self) -> &WorthQueryPortableApplicationConditionalOperationBinding {
        &self.portable
    }

    pub(crate) fn into_portable(self) -> WorthQueryPortableApplicationConditionalOperationBinding {
        self.portable
    }
}

impl<Schema, ApplicationOperation, Input, D, O, F> Clone
    for WorthQueryApplicationConditionalOperationBinding<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
    >
{
    fn clone(&self) -> Self {
        Self {
            portable: self.portable.clone(),
            marker: PhantomData,
        }
    }
}

impl<Schema, ApplicationOperation, Input, D, O, F> std::fmt::Debug
    for WorthQueryApplicationConditionalOperationBinding<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
    >
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryApplicationConditionalOperationBinding")
            .field("portable", &self.portable)
            .finish_non_exhaustive()
    }
}
