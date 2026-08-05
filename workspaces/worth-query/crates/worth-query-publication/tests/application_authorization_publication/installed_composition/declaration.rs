use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequest,
    ApplicationCapabilityRequestContext, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityRequestProjectionDenial,
};
use worth_query_declaration::{
    worth_query_application_schema, worth_query_aspect, worth_query_capability,
    worth_query_capability_context, worth_query_capability_context_entity_slot,
    worth_query_capability_provenance, worth_query_entity, worth_query_field,
    worth_query_operation, worth_query_operation_reads, worth_query_operation_writes,
    worth_query_principal_binding, worth_query_relation,
};

worth_query_application_schema! {
    pub schema PublicationAuthorizationSchema {
        owner: publication_authorization_proof,
        version: (1, 0),
        members: |schema| {
            let schema = schema
                .entity(ExternalMapping::reference())
                .entity(Principal::reference())
                .entity(Resource::reference())
                .entity(CapabilityGrant::reference())
                .entity(ActionRecord::reference())
                .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
                .aspect(Principal::reference(), PrincipalFacts::reference())
                .aspect(Resource::reference(), ResourceFacts::reference())
                .aspect(CapabilityGrant::reference(), GrantFacts::reference())
                .aspect(ActionRecord::reference(), ActionRecordFacts::reference())
                .field(ExternalMapping::reference(), ExternalIdentityField::reference())
                .field(ExternalMapping::reference(), MappingStatusField::reference())
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .field(Resource::reference(), ResourceIdentityField::reference())
                .field(Resource::reference(), ResourceWorkflowField::reference())
                .field(Resource::reference(), ResourceLabelField::reference())
                .field(CapabilityGrant::reference(), GrantIdentityField::reference())
                .field(CapabilityGrant::reference(), GrantActionField::reference())
                .field(CapabilityGrant::reference(), GrantPurposeField::reference())
                .field(CapabilityGrant::reference(), GrantStatusField::reference())
                .field(CapabilityGrant::reference(), GrantWorkflowField::reference())
                .field(CapabilityGrant::reference(), GrantNotBeforeField::reference())
                .field(CapabilityGrant::reference(), GrantNotAfterField::reference())
                .field(CapabilityGrant::reference(), GrantDelegationLimitField::reference())
                .field(ActionRecord::reference(), ActionRecordIdentityField::reference())
                .relation(MappingTarget::reference(), ExternalMapping::reference(), Principal::reference())
                .relation(ResourceOwner::reference(), Principal::reference(), Resource::reference())
                .relation(GrantGrantee::reference(), Principal::reference(), CapabilityGrant::reference())
                .relation(GrantGrantor::reference(), Principal::reference(), CapabilityGrant::reference())
                .relation(GrantResource::reference(), CapabilityGrant::reference(), Resource::reference())
                .relation(GrantParent::reference(), CapabilityGrant::reference(), CapabilityGrant::reference())
                .relation(ExplicitDeny::reference(), Principal::reference(), Resource::reference())
                .relation(ConflictingActor::reference(), Principal::reference(), Resource::reference())
                .relation(RequestActor::reference(), Principal::reference(), ActionRecord::reference())
                .relation(PriorActor::reference(), Principal::reference(), ActionRecord::reference())
                .relation(ActionResource::reference(), ActionRecord::reference(), Resource::reference())
                .principal_binding(PublicationIdentityBinding::reference())
                .capability_context(PublicationRequestContext::reference())
                .capability_context_entity_slot(RequestActorSlot::reference())
                .capability_context_entity_slot(PriorActorSlot::reference())
                .capability_provenance(PublicationCapabilityProvenance::reference())
                .operation(PublicationOperation::reference())
                .operation_decision_fact_budget(PublicationOperation::reference(), 1)
                .operation_projection_work_budget(PublicationOperation::reference(), 16)
                .operation_read_field(PublicationOperation::reference(), ResourceLabelField::reference())
                .operation_write(PublicationOperation::reference(), ResourceLabelField::reference());
            super::contract::install(schema)
        }
    }
}

worth_query_entity!(pub ExternalMapping in PublicationAuthorizationSchema);
worth_query_entity!(pub Principal in PublicationAuthorizationSchema);
worth_query_entity!(pub Resource in PublicationAuthorizationSchema);
worth_query_entity!(pub CapabilityGrant in PublicationAuthorizationSchema);
worth_query_entity!(pub ActionRecord in PublicationAuthorizationSchema);

worth_query_aspect!(pub ExternalIdentity in PublicationAuthorizationSchema, ExternalMapping);
worth_query_aspect!(pub PrincipalFacts in PublicationAuthorizationSchema, Principal);
worth_query_aspect!(pub ResourceFacts in PublicationAuthorizationSchema, Resource);
worth_query_aspect!(pub GrantFacts in PublicationAuthorizationSchema, CapabilityGrant);
worth_query_aspect!(pub ActionRecordFacts in PublicationAuthorizationSchema, ActionRecord);

worth_query_field!(pub ExternalIdentityField in PublicationAuthorizationSchema, ExternalMapping, ExternalIdentity: worth_query_declaration::facade::authentication::WorthQueryExternalPrincipalIdentity, read_only, equality);
worth_query_field!(pub MappingStatusField in PublicationAuthorizationSchema, ExternalMapping, ExternalIdentity: worth_query_declaration::facade::authentication::WorthQueryPrincipalMappingStatus, read_write, equality);
worth_query_field!(pub PrincipalIdentityField in PublicationAuthorizationSchema, Principal, PrincipalFacts: u64, read_only, equality);
worth_query_field!(pub ResourceIdentityField in PublicationAuthorizationSchema, Resource, ResourceFacts: String, read_only, equality);
worth_query_field!(pub ResourceWorkflowField in PublicationAuthorizationSchema, Resource, ResourceFacts: String, read_write, equality);
worth_query_field!(pub ResourceLabelField in PublicationAuthorizationSchema, Resource, ResourceFacts: String, read_write, equality);
worth_query_field!(pub GrantIdentityField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: String, read_only, equality);
worth_query_field!(pub GrantActionField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: String, read_only, no_equality);
worth_query_field!(pub GrantPurposeField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: String, read_only, no_equality);
worth_query_field!(pub GrantStatusField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: String, read_write, no_equality);
worth_query_field!(pub GrantWorkflowField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: String, read_write, no_equality);
worth_query_field!(pub GrantNotBeforeField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: u64, read_write, no_equality);
worth_query_field!(pub GrantNotAfterField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: u64, read_write, no_equality);
worth_query_field!(pub GrantDelegationLimitField in PublicationAuthorizationSchema, CapabilityGrant, GrantFacts: u64, read_write, no_equality);
worth_query_field!(pub ActionRecordIdentityField in PublicationAuthorizationSchema, ActionRecord, ActionRecordFacts: String, read_only, equality);

worth_query_relation!(pub MappingTarget in PublicationAuthorizationSchema, ExternalMapping => Principal);
worth_query_relation!(pub ResourceOwner in PublicationAuthorizationSchema, Principal => Resource);
worth_query_relation!(pub GrantGrantee in PublicationAuthorizationSchema, Principal => CapabilityGrant);
worth_query_relation!(pub GrantGrantor in PublicationAuthorizationSchema, Principal => CapabilityGrant);
worth_query_relation!(pub GrantResource in PublicationAuthorizationSchema, CapabilityGrant => Resource);
worth_query_relation!(pub GrantParent in PublicationAuthorizationSchema, CapabilityGrant => CapabilityGrant);
worth_query_relation!(pub ExplicitDeny in PublicationAuthorizationSchema, Principal => Resource);
worth_query_relation!(pub ConflictingActor in PublicationAuthorizationSchema, Principal => Resource);
worth_query_relation!(pub RequestActor in PublicationAuthorizationSchema, Principal => ActionRecord);
worth_query_relation!(pub PriorActor in PublicationAuthorizationSchema, Principal => ActionRecord);
worth_query_relation!(pub ActionResource in PublicationAuthorizationSchema, ActionRecord => Resource);

worth_query_principal_binding!(pub PublicationIdentityBinding in PublicationAuthorizationSchema, mapping ExternalMapping { identity: ExternalIdentityField, status: MappingStatusField, target: MappingTarget => Principal, principal_identity: PrincipalIdentityField });
worth_query_capability_context!(pub PublicationRequestContext in PublicationAuthorizationSchema);
worth_query_capability_context_entity_slot!(pub RequestActorSlot in PublicationAuthorizationSchema, PublicationRequestContext => ActionRecord);
worth_query_capability_context_entity_slot!(pub PriorActorSlot in PublicationAuthorizationSchema, PublicationRequestContext => ActionRecord);
worth_query_capability_provenance!(pub PublicationCapabilityProvenance in PublicationAuthorizationSchema);
worth_query_capability!(pub PublicationCapability in PublicationAuthorizationSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PublicationInput;

worth_query_operation!(pub PublicationOperation(PublicationInput) in PublicationAuthorizationSchema);
worth_query_operation_reads!(PublicationOperation => [ResourceLabelField]);
worth_query_operation_writes!(PublicationOperation => [ResourceLabelField]);

impl ApplicationCapabilityRequest<PublicationAuthorizationSchema, PublicationCapability>
    for PublicationInput
{
    type Scope = Resource;
    type Context = PublicationRequestContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<
            PublicationAuthorizationSchema,
            Resource,
            PublicationRequestContext,
        >,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        Ok(ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(
                ResourceIdentityField::reference(),
                "resource-1".to_owned(),
            ),
            "inspect".to_owned(),
            "publication-proof".to_owned(),
            ApplicationCapabilityRequestContext::new(PublicationRequestContext::reference())
                .entity(
                    RequestActorSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        ActionRecordIdentityField::reference(),
                        "selected-request".to_owned(),
                    ),
                )
                .entity(
                    PriorActorSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        ActionRecordIdentityField::reference(),
                        "selected-prior".to_owned(),
                    ),
                ),
        ))
    }
}
