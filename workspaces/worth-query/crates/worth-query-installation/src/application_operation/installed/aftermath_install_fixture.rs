//! Shared install fixtures for aftermath classification tests.

use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolVersion, CanonicalDigestId,
};
use worth_query_declaration::facade::application_aftermath::DeclaredApplicationAftermathContract;
use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredCompensation, DeclaredCorrectionMechanism,
    DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand, DeclaredPreImageLocus,
    DeclaredRecordedInverse,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationExternalEffectProtocol, ApplicationFieldMarkerIdentity, ApplicationFieldRef,
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
    ApplicationOperationRef, ApplicationSchema, ApplicationSchemaBindingIdentity,
    ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder, ApplicationSchemaMember,
};

use crate::application_aftermath::{
    install_application_aftermath, WorthQueryAftermathInstallationDenial,
    WorthQueryInstalledAftermathContract,
};

use super::operation_compilation::WorthQueryApplicationOperationCompilation;

pub(crate) struct FixtureSchema;
struct FixtureOperation;
struct FixtureInput;
pub(crate) struct Account;
pub(crate) struct OtherAccount;
pub(crate) struct State;
pub(crate) struct OtherAccountState;
pub(crate) struct OtherState;
pub(crate) struct Balance;
pub(crate) struct OtherAccountBalance;
pub(crate) struct OtherStateBalance;
pub(crate) struct SecretField;
pub(crate) struct OtherBalance;

macro_rules! entity_identity {
    ($marker:ty, $identifier:literal) => {
        impl ApplicationEntityMarkerIdentity for $marker {
            type Schema = FixtureSchema;
            const IDENTIFIER: &'static str = $identifier;
        }
    };
}

macro_rules! aspect_identity {
    ($marker:ty, $entity:ty, $identifier:literal) => {
        impl ApplicationAspectMarkerIdentity for $marker {
            type Schema = FixtureSchema;
            type Entity = $entity;
            const IDENTIFIER: &'static str = $identifier;
        }
    };
}

macro_rules! field_identity {
    ($marker:ty, $entity:ty, $aspect:ty, $identifier:literal) => {
        impl ApplicationFieldMarkerIdentity for $marker {
            type Schema = FixtureSchema;
            type Entity = $entity;
            type Aspect = $aspect;
            const IDENTIFIER: &'static str = $identifier;
        }
    };
}

entity_identity!(Account, "Account");
entity_identity!(OtherAccount, "OtherAccount");
aspect_identity!(State, Account, "State");
aspect_identity!(OtherAccountState, OtherAccount, "State");
aspect_identity!(OtherState, Account, "OtherState");
field_identity!(Balance, Account, State, "balance");
field_identity!(
    OtherAccountBalance,
    OtherAccount,
    OtherAccountState,
    "balance"
);
field_identity!(OtherStateBalance, Account, OtherState, "balance");
field_identity!(SecretField, Account, State, "secret-field");
field_identity!(OtherBalance, Account, State, "other-balance");

impl ApplicationSchema for FixtureSchema {
    const OWNER: &'static str = "worth-query-installation-tests";
    const NAME: &'static str = "AftermathFixture";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema().build()
    }
}

pub(crate) fn digest(byte: u8) -> CanonicalDigestId {
    CanonicalDigestId::new([byte; 32])
}

/// One operation under aftermath installation, with every axis a test may vary.
///
/// The escaping lane defaults to `None` — the operation declared no
/// operation-definition external-effect slot — and `escaping()` is the twin where it
/// did. Making the lane a visible axis here is the point: it is the operation's
/// own contract, and no aftermath declaration can contradict it (Q8.25-C1).
pub(crate) struct AftermathInstall {
    binding: ApplicationSchemaBindingIdentity,
    operation_slot: &'static str,
    declared_reads: Vec<ApplicationOperationDecisionReadTarget>,
    external_effect: Option<FixtureExternalEffect>,
}

struct FixtureExternalEffect {
    rust_payload_type: String,
    protocol: ApplicationExternalEffectProtocol,
}

impl AftermathInstall {
    pub(crate) fn new(
        binding: ApplicationSchemaBindingIdentity,
        operation_slot: &'static str,
    ) -> Self {
        Self {
            binding,
            operation_slot,
            declared_reads: Vec::new(),
            external_effect: None,
        }
    }

    pub(crate) fn reads<Field>(self) -> Self
    where
        Field: ApplicationFieldMarkerIdentity<
            Schema = FixtureSchema,
            Entity = Account,
            Aspect = State,
        >,
    {
        self.reads_at::<Account, State, Field>()
    }

    pub(crate) fn reads_at<Entity, Aspect, Field>(mut self) -> Self
    where
        Entity: ApplicationEntityMarkerIdentity<Schema = FixtureSchema>,
        Aspect: ApplicationAspectMarkerIdentity<Schema = FixtureSchema, Entity = Entity>,
        Field: ApplicationFieldMarkerIdentity<
            Schema = FixtureSchema,
            Entity = Entity,
            Aspect = Aspect,
        >,
    {
        self.declared_reads = vec![decision_read_target::<Entity, Aspect, Field>()];
        self
    }

    /// The operation definition declares an external-effect slot on the schema.
    pub(crate) fn escaping(self) -> Self {
        self.escaping_with_protocol(protocol(1))
    }

    pub(crate) fn escaping_with_protocol(
        self,
        protocol: ApplicationExternalEffectProtocol,
    ) -> Self {
        self.escaping_with_contract("fixture::EscapingPayload", protocol)
    }

    pub(crate) fn escaping_with_contract(
        mut self,
        rust_payload_type: &str,
        protocol: ApplicationExternalEffectProtocol,
    ) -> Self {
        self.external_effect = Some(FixtureExternalEffect {
            rust_payload_type: rust_payload_type.to_owned(),
            protocol,
        });
        self
    }

    pub(crate) fn install(
        &self,
        declared: DeclaredApplicationAftermathContract<FixtureSchema>,
    ) -> Result<WorthQueryInstalledAftermathContract, WorthQueryAftermathInstallationDenial> {
        let mut members = associated_operation_members(self.operation_slot, declared);
        members.extend([
            ApplicationSchemaMember::OperationProgram {
                operation: self.operation_slot.to_owned(),
                target: ApplicationOperationProgramTarget::Create {
                    entity: "FixtureEntity".to_owned(),
                },
            },
            ApplicationSchemaMember::OperationDecisionFactBudget {
                operation: self.operation_slot.to_owned(),
                maximum_fact_count: 8,
            },
            ApplicationSchemaMember::OperationProjectionWorkBudget {
                operation: self.operation_slot.to_owned(),
                maximum_work_units: 16,
            },
        ]);
        members.extend(self.declared_reads.iter().cloned().map(|target| {
            ApplicationSchemaMember::OperationDecisionRead {
                operation: self.operation_slot.to_owned(),
                target,
            }
        }));
        if let Some(external_effect) = &self.external_effect {
            members.push(ApplicationSchemaMember::OperationExternalEffect {
                operation: self.operation_slot.to_owned(),
                effect: "EscapingEffect".to_owned(),
                rust_payload_type: external_effect.rust_payload_type.clone(),
                protocol: external_effect.protocol.clone(),
                maximum_payload_bytes: 64,
                correlation_family: "escaped-rail".to_owned(),
            });
        }
        let compilation = WorthQueryApplicationOperationCompilation::resolve(
            self.binding.clone(),
            &members,
            self.operation_slot,
            std::any::type_name::<FixtureInput>(),
        )
        .expect("the fixture must describe one compilable operation");
        install_application_aftermath(&compilation)
            .map(|installed| installed.expect("the fixture declares an aftermath"))
    }
}

fn associated_operation_members(
    operation_slot: &'static str,
    contract: DeclaredApplicationAftermathContract<FixtureSchema>,
) -> Vec<ApplicationSchemaMember> {
    let definition = ApplicationOperationRef::<FixtureSchema, FixtureOperation, FixtureInput>::
        from_schema_identifier(operation_slot)
        .definition()
        .no_external_effect()
        .aftermath(contract)
        .finish();
    let declaration = ApplicationSchemaDeclarationBuilder::<FixtureSchema>::for_schema()
        .operation(definition)
        .build()
        .expect("the matching operation builder associates the aftermath");
    declaration.erased().members().to_vec()
}

pub(crate) fn protocol(version: u32) -> ApplicationExternalEffectProtocol {
    ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.escaping-payload"),
        BoundaryProtocolVersion::new(version),
    )
}

pub(crate) fn binding(
    package: CanonicalDigestId,
    schema: CanonicalDigestId,
    generation: u64,
) -> ApplicationSchemaBindingIdentity {
    ApplicationSchemaBindingIdentity::from_installed_parts(1, generation, package, schema)
}

pub(crate) fn recorded_inverse<Field>() -> DeclaredCorrectionMechanism<FixtureSchema>
where
    Field: ApplicationFieldMarkerIdentity<Schema = FixtureSchema, Entity = Account, Aspect = State>,
{
    recorded_inverse_at::<Account, State, Field>(256)
}

pub(crate) fn recorded_inverse_at<Entity, Aspect, Field>(
    maximum_encoded_bytes: usize,
) -> DeclaredCorrectionMechanism<FixtureSchema>
where
    Entity: ApplicationEntityMarkerIdentity<Schema = FixtureSchema>,
    Aspect: ApplicationAspectMarkerIdentity<Schema = FixtureSchema, Entity = Entity>,
    Field: ApplicationFieldMarkerIdentity<Schema = FixtureSchema, Entity = Entity, Aspect = Aspect>,
{
    let field =
        ApplicationFieldRef::<FixtureSchema, Entity, Aspect, Field, u64>::from_schema_types();
    DeclaredCorrectionMechanism::RecordedInverse(
        DeclaredRecordedInverse::new(
            "unfreeze",
            DeclaredLoweringCorrespondenceRef::new("geometry-inverse").unwrap(),
            DeclaredAftermathPostcondition::ExactPriorTruth,
            DeclaredPreImageDemand::new(
                [DeclaredPreImageLocus::from_field(field)],
                maximum_encoded_bytes,
            )
            .unwrap(),
        )
        .unwrap(),
    )
}

fn decision_read_target<Entity, Aspect, Field>() -> ApplicationOperationDecisionReadTarget
where
    Entity: ApplicationEntityMarkerIdentity<Schema = FixtureSchema>,
    Aspect: ApplicationAspectMarkerIdentity<Schema = FixtureSchema, Entity = Entity>,
    Field: ApplicationFieldMarkerIdentity<Schema = FixtureSchema, Entity = Entity, Aspect = Aspect>,
{
    let field =
        ApplicationFieldRef::<FixtureSchema, Entity, Aspect, Field, u64>::from_schema_types();
    ApplicationOperationDecisionReadTarget::Field {
        entity: field.entity().to_owned(),
        aspect: field.aspect().to_owned(),
        field: field.field().to_owned(),
    }
}

pub(crate) fn compensation() -> DeclaredCorrectionMechanism<FixtureSchema> {
    DeclaredCorrectionMechanism::Compensation(
        DeclaredCompensation::new(
            "compensating-journal",
            DeclaredAftermathPostcondition::BusinessPostcondition {
                identity: "settled".into(),
            },
        )
        .unwrap(),
    )
}
