use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityActorComposition, ApplicationCapabilityAllowRule,
        ApplicationCapabilityCardinalityDimension, ApplicationCapabilityComposition,
        ApplicationCapabilityConflictRule, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContextRef, ApplicationCapabilityCurrentnessDefinition,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDisclosureRule, ApplicationCapabilityDistinctActorRule,
        ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
        ApplicationCapabilityGraphClause, ApplicationCapabilityGraphRule,
        ApplicationCapabilityPropagationComposition, ApplicationCapabilityProvenanceRef,
        ApplicationCapabilityRef, ApplicationCapabilityRelationBinding,
        ApplicationCapabilityRelationDimension, ApplicationCapabilitySeparationOfDutyRule,
        ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
        ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
        ApplicationCapabilityWorkflowDefinition,
    },
    application_schema::{
        ApplicationAspectRef, ApplicationAuthorizationPathBuilder, ApplicationEntityRef,
        ApplicationFieldPresence, ApplicationFieldRef, ApplicationOperationRef,
        ApplicationPrincipalBindingRef, ApplicationRelationRef, ApplicationSchema,
        ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder,
        DeclaredApplicationFieldValue, EqualityPredicate, NoEqualityPredicate, OperationCreates,
        ReadOnly, ReadWrite,
    },
    authentication::{WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus},
};

use crate::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledGraphAuthorizationRequirement,
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationOwner,
    WorthQueryInstalledPackageIndex, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
};

struct CapabilitySchema;
struct ExternalMapping;
struct Grant;
struct ExternalMappingFacts;
struct GrantFacts;
struct ExternalIdentity;
struct MappingStatus;
struct GrantValue;
struct MappingTarget;
struct GrantRelation;
struct PrincipalBinding;
struct CapabilityOperation;
struct CapabilityInput;
struct Capability;
struct Context;
struct Provenance;

impl DeclaredApplicationFieldValue for GrantValue {
    type Value = u64;
    const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
}

impl DeclaredApplicationFieldValue for ExternalIdentity {
    type Value = WorthQueryExternalPrincipalIdentity;
    const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
}

impl DeclaredApplicationFieldValue for MappingStatus {
    type Value = WorthQueryPrincipalMappingStatus;
    const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
}

impl OperationCreates<CapabilityOperation> for Grant {}

impl ApplicationSchema for CapabilitySchema {
    const OWNER: &'static str = "graph-obligation-test";
    const NAME: &'static str = "CapabilitySchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        let grant = grant();
        let operation = operation();
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(mapping())
            .entity(grant)
            .aspect(mapping(), mapping_aspect())
            .aspect(grant, aspect())
            .field(mapping(), external_identity_field())
            .field(mapping(), mapping_status_field())
            .field(grant, field())
            .relation(mapping_target(), mapping(), grant)
            .relation(relation(), grant, grant)
            .principal_binding(principal_binding())
            .capability_context(context())
            .capability_provenance(provenance())
            .operation(operation)
            .operation_decision_fact_budget(operation, 1)
            .operation_projection_work_budget(operation, 8)
            .operation_create(operation, grant)
            .capability(capability_contract())
            .build()
    }
}

#[test]
fn installed_capability_operation_retains_exact_graph_authorization_contract() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(CapabilitySchema::declaration().unwrap())
        .unwrap();
    let operation = schema.installed_operation(operation()).unwrap();
    index.validate_application_operation(&operation).unwrap();

    let authorization = operation
        .contracts()
        .obligations()
        .inspect_kind(WorthQueryInstalledGraphObligationKind::AuthorizationObservation);
    let [authorization] = authorization.rows() else {
        panic!("the capability-authorized operation must install one authorization obligation");
    };
    let Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements)) =
        authorization.authorization_requirement()
    else {
        panic!("the authorization obligation must retain the typed capability contract");
    };
    let [requirement] = requirements else {
        panic!("the operation must retain exactly one capability requirement");
    };
    assert_eq!(requirement.contract().name(), "Capability");
    assert_eq!(requirement.contract().operation(), "CapabilityOperation");
    assert_ne!(requirement.identity().bytes(), &[0; 32]);
    assert_eq!(
        authorization.owner_progression(),
        [
            WorthQueryInstalledGraphObligationOwner::Relational,
            WorthQueryInstalledGraphObligationOwner::RuntimeBridge,
            WorthQueryInstalledGraphObligationOwner::Signal,
            WorthQueryInstalledGraphObligationOwner::QueryAdmission,
        ]
    );
}

fn capability_contract(
) -> worth_query_declaration::facade::application_capability::ApplicationCapabilityContract<
    CapabilitySchema,
    Capability,
    CapabilityOperation,
    CapabilityInput,
> {
    worth_query_declaration::facade::application_capability::ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::from_schema_identifier("Capability"),
        operation(),
        grant(),
    )
    .target(ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(field(), 1_u64),
        ApplicationCapabilityRelationBinding::from_reference(relation()),
        ApplicationCapabilityRelationDimension::not_applicable(),
        ApplicationCapabilityFieldDimension::not_applicable(),
        ApplicationCapabilityValueBinding::new(field(), 2_u64),
    ))
    .constraints(capability_constraints())
    .delegation(ApplicationCapabilityDelegationDefinition::new(
        ApplicationCapabilityRelationBinding::from_reference(relation()),
        ApplicationCapabilityRelationBinding::from_reference(relation()),
        ApplicationCapabilityRelationBinding::from_reference(relation()),
        ApplicationCapabilityFieldBinding::from_reference(field()),
        provenance(),
    ))
    .composition(capability_composition())
    .build()
}

fn capability_constraints() -> ApplicationCapabilityConstraintDefinition {
    let field_binding = || ApplicationCapabilityFieldBinding::from_reference(field());
    ApplicationCapabilityConstraintDefinition::new(
        ApplicationCapabilityFieldDimension::not_applicable(),
        ApplicationCapabilityCardinalityDimension::One,
        ApplicationCapabilityCurrentnessDefinition::new(
            ApplicationCapabilityValueBinding::new(field(), 1_u64),
            ApplicationCapabilityWorkflowDefinition::new(field_binding(), field_binding()),
            ApplicationCapabilityValidityDefinition::new(
                ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
                field_binding(),
                field_binding(),
            ),
        ),
        context(),
    )
}

fn capability_composition() -> ApplicationCapabilityComposition {
    let path = ApplicationAuthorizationPathBuilder::from_principal(grant())
        .forward(relation())
        .allow(grant());
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(path),
            ])),
            ApplicationCapabilityDenyRule::not_applicable(),
            ApplicationCapabilityConflictRule::not_applicable(),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::not_applicable(),
            ApplicationCapabilityDistinctActorRule::not_applicable(),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::forbidden(),
            ApplicationCapabilityDisclosureRule::not_applicable(),
        ),
    )
}

fn installed_index() -> WorthQueryInstalledPackageIndex {
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "graph-obligation-test",
        1,
        0,
    ))
    .application_schema(CapabilitySchema::declaration().unwrap())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
}

fn grant() -> ApplicationEntityRef<CapabilitySchema, Grant> {
    ApplicationEntityRef::from_schema_identifier("Grant")
}

fn mapping() -> ApplicationEntityRef<CapabilitySchema, ExternalMapping> {
    ApplicationEntityRef::from_schema_identifier("ExternalMapping")
}

fn mapping_aspect() -> ApplicationAspectRef<CapabilitySchema, ExternalMapping, ExternalMappingFacts>
{
    ApplicationAspectRef::from_schema_identifier("ExternalMappingFacts")
}

fn aspect() -> ApplicationAspectRef<CapabilitySchema, Grant, GrantFacts> {
    ApplicationAspectRef::from_schema_identifier("GrantFacts")
}

fn field() -> ApplicationFieldRef<
    CapabilitySchema,
    Grant,
    GrantFacts,
    GrantValue,
    u64,
    ReadOnly,
    EqualityPredicate,
> {
    ApplicationFieldRef::from_schema_identifiers("Grant", "GrantFacts", "GrantValue")
}

fn external_identity_field() -> ApplicationFieldRef<
    CapabilitySchema,
    ExternalMapping,
    ExternalMappingFacts,
    ExternalIdentity,
    WorthQueryExternalPrincipalIdentity,
    ReadOnly,
    EqualityPredicate,
> {
    ApplicationFieldRef::from_schema_identifiers(
        "ExternalMapping",
        "ExternalMappingFacts",
        "ExternalIdentity",
    )
}

fn mapping_status_field() -> ApplicationFieldRef<
    CapabilitySchema,
    ExternalMapping,
    ExternalMappingFacts,
    MappingStatus,
    WorthQueryPrincipalMappingStatus,
    ReadWrite,
    NoEqualityPredicate,
> {
    ApplicationFieldRef::from_schema_identifiers(
        "ExternalMapping",
        "ExternalMappingFacts",
        "MappingStatus",
    )
}

fn mapping_target(
) -> ApplicationRelationRef<CapabilitySchema, MappingTarget, ExternalMapping, Grant> {
    ApplicationRelationRef::from_schema_identifiers("MappingTarget", "ExternalMapping", "Grant")
}

fn principal_binding(
) -> ApplicationPrincipalBindingRef<CapabilitySchema, PrincipalBinding, ExternalMapping, Grant, u64>
{
    ApplicationPrincipalBindingRef::from_schema_identifiers(
        "PrincipalBinding",
        "ExternalMapping",
        "ExternalMappingFacts",
        "ExternalIdentity",
        "ExternalMappingFacts",
        "MappingStatus",
        "MappingTarget",
        "Grant",
        "GrantFacts",
        "GrantValue",
    )
}

fn relation() -> ApplicationRelationRef<CapabilitySchema, GrantRelation, Grant, Grant> {
    ApplicationRelationRef::from_schema_identifiers("GrantRelation", "Grant", "Grant")
}

fn operation() -> ApplicationOperationRef<CapabilitySchema, CapabilityOperation, CapabilityInput> {
    ApplicationOperationRef::from_schema_identifier("CapabilityOperation")
}

fn context() -> ApplicationCapabilityContextRef<CapabilitySchema, Context> {
    ApplicationCapabilityContextRef::from_schema_identifier("Context")
}

fn provenance() -> ApplicationCapabilityProvenanceRef<CapabilitySchema, Provenance> {
    ApplicationCapabilityProvenanceRef::from_schema_identifier("Provenance")
}
