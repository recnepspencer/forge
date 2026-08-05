use super::*;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationActivationDefinition;

struct ActivationOperation;

pub(crate) fn activated_contract() -> ErasedApplicationCapabilityContract {
    ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
        ApplicationOperationRef::<Schema, Operation, ()>::from_schema_identifier("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target(None))
    .constraints(constraints(
        None,
        ApplicationCapabilityContextRef::<Schema, Context>::from_schema_identifier("Context"),
    ))
    .delegation(
        delegation(
            None,
            ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
                "Provenance",
            ),
        )
        .with_activation(ApplicationCapabilityDelegationActivationDefinition::new(
            ApplicationOperationRef::<Schema, ActivationOperation, ()>::from_schema_identifier(
                "Activation",
            ),
            field_binding::<Action>("Action"),
        )),
    )
    .composition(composition(None))
    .elevation(ApplicationCapabilityElevationRule::not_applicable())
    .build()
    .erased()
    .clone()
}
