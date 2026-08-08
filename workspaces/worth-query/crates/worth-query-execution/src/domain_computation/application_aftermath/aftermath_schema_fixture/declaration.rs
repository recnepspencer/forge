//! Aftermath fixture schema declaration members.

use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::{
    ApplicationAbilityRef, ApplicationAspectRef, ApplicationAuthorizationPathBuilder,
    ApplicationEffectPayload, ApplicationEntityRef, ApplicationExternalEffectPayload,
    ApplicationExternalEffectProtocol, ApplicationFieldPresence, ApplicationFieldRef,
    ApplicationPolicyRef, ApplicationPrincipalBindingRef, ApplicationPrincipalBindingRequirements,
    ApplicationPrincipalIdentityRequirement, ApplicationPrincipalMappingIdentityRequirement,
    ApplicationPrincipalMappingStatusRequirement, ApplicationPrincipalTargetRequirement,
    ApplicationRelationRef, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, DeclaredApplicationFieldValue, EqualityPredicate,
    NoEqualityPredicate, OperationEmits, OperationReads, OperationRequiresAbility, ReadOnly,
    ReadWrite,
};
use worth_query_declaration::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};
use worth_query_declaration::worth_query_effect;

use super::operations::{bind_operations, FixtureReads};

pub(super) struct AftermathFixtureSchema;
pub(super) struct FixtureInput;

pub(super) struct ReleaseEstate;
pub(super) struct Transfer;
pub(super) struct TransferSmall;
pub(super) struct TransferLarge;
pub(super) struct FreezeAccount;
pub(super) struct NotifyDeath;
pub(super) struct LegalHold;
pub(super) struct AuditRetention;
pub(super) struct ApproveEmergencyAccess;
pub(super) struct WireTransferFinal;
pub(super) struct FreezeNote;
pub(super) struct FreezeBalance;
pub(super) struct Charge;

pub(super) struct FixtureEntity;
pub(super) struct FixtureAbility;
struct FixturePolicy;
pub(super) struct IdentityAspect;
struct ExternalIdentityField;
struct MappingStatusField;
pub(super) struct PrincipalIdentityField;
pub(super) struct FrozenField;
pub(super) struct NoteField;
pub(super) struct BalanceField;
struct MappingTarget;
struct PrincipalBinding;

macro_rules! required_field {
    ($field:ty, $value:ty) => {
        impl DeclaredApplicationFieldValue for $field {
            type Value = $value;
            const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
        }
    };
}

required_field!(ExternalIdentityField, WorthQueryExternalPrincipalIdentity);
required_field!(MappingStatusField, WorthQueryPrincipalMappingStatus);
required_field!(PrincipalIdentityField, u64);
required_field!(FrozenField, bool);
required_field!(NoteField, String);
required_field!(BalanceField, u64);

macro_rules! reads_principal {
    ($($op:ty),+ $(,)?) => { $(impl OperationReads<$op> for PrincipalIdentityField {})+ };
}
reads_principal!(
    ReleaseEstate,
    Transfer,
    TransferSmall,
    TransferLarge,
    NotifyDeath,
    LegalHold,
    AuditRetention,
    ApproveEmergencyAccess,
    WireTransferFinal,
    Charge,
);
impl OperationReads<FreezeAccount> for FrozenField {}
impl OperationReads<FreezeNote> for NoteField {}
impl OperationReads<FreezeBalance> for BalanceField {}

/// Payload the fixture's two escaping operations project onto the wire.
///
/// The fixture declares real external-effect slots in those operation definitions
/// operations. Before slice 9A it declared only an aftermath escaping posture,
/// with no lane behind it — the fixture was itself an instance of the defect,
/// asserting a reconcilable external owner for an operation that would have
/// co-committed no outbox record and dispatched nothing (Q8.25-C1).
#[derive(Clone, Copy)]
pub(super) struct FixtureExternalNotice(u64);

impl ApplicationEffectPayload for FixtureExternalNotice {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>()).unwrap_or(u64::MAX)
    }
}

impl ApplicationExternalEffectPayload for FixtureExternalNotice {
    const PROTOCOL: ApplicationExternalEffectProtocol = ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.fixture-external-notice"),
        BoundaryProtocolVersion::new(1),
    );
    const MAX_EXTERNAL_BYTES: u64 = 8;

    fn external_effect_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

worth_query_effect!(
    pub(super) DeathNoticeEffect(FixtureExternalNotice) in AftermathFixtureSchema
);
worth_query_effect!(
    pub(super) WireInstructionEffect(FixtureExternalNotice) in AftermathFixtureSchema
);

impl OperationEmits<NotifyDeath> for DeathNoticeEffect {}
impl OperationEmits<WireTransferFinal> for WireInstructionEffect {}

macro_rules! requires_ability {
    ($($op:ty),+ $(,)?) => { $(impl OperationRequiresAbility<$op> for FixtureAbility {})+ };
}
requires_ability!(
    ReleaseEstate,
    Transfer,
    TransferSmall,
    TransferLarge,
    FreezeAccount,
    NotifyDeath,
    LegalHold,
    AuditRetention,
    ApproveEmergencyAccess,
    WireTransferFinal,
    FreezeNote,
    FreezeBalance,
    Charge,
);

pub(super) type PrincipalRead = ApplicationFieldRef<
    AftermathFixtureSchema,
    FixtureEntity,
    IdentityAspect,
    PrincipalIdentityField,
    u64,
    ReadOnly,
    EqualityPredicate,
>;
pub(super) type FrozenRead = ApplicationFieldRef<
    AftermathFixtureSchema,
    FixtureEntity,
    IdentityAspect,
    FrozenField,
    bool,
    ReadOnly,
    NoEqualityPredicate,
>;
pub(super) type NoteRead = ApplicationFieldRef<
    AftermathFixtureSchema,
    FixtureEntity,
    IdentityAspect,
    NoteField,
    String,
    ReadOnly,
    NoEqualityPredicate,
>;
pub(super) type BalanceRead = ApplicationFieldRef<
    AftermathFixtureSchema,
    FixtureEntity,
    IdentityAspect,
    BalanceField,
    u64,
    ReadOnly,
    NoEqualityPredicate,
>;

pub(super) fn principal_read() -> PrincipalRead {
    ApplicationFieldRef::from_schema_identifiers(
        "FixtureEntity",
        "IdentityAspect",
        "PrincipalIdentityField",
    )
}

impl ApplicationSchema for AftermathFixtureSchema {
    const OWNER: &'static str = "aftermath-fixture";
    const NAME: &'static str = "AftermathFixtureSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        let entity =
            ApplicationEntityRef::<Self, FixtureEntity>::from_schema_identifier("FixtureEntity");
        let ability =
            ApplicationAbilityRef::<Self, FixtureAbility, FixtureEntity>::from_schema_identifiers(
                "FixtureAbility",
                "FixtureEntity",
            );
        let reads = FixtureReads {
            frozen: ApplicationFieldRef::from_schema_identifiers(
                "FixtureEntity",
                "IdentityAspect",
                "frozen",
            ),
            note: ApplicationFieldRef::from_schema_identifiers(
                "FixtureEntity",
                "IdentityAspect",
                "note",
            ),
            balance: ApplicationFieldRef::from_schema_identifiers(
                "FixtureEntity",
                "IdentityAspect",
                "balance",
            ),
        };
        let schema = bind_entity_shape(entity, &reads);
        let schema = bind_authorization_shape(schema, entity, ability);
        bind_operations(schema, ability, reads)
    }
}

type FixtureEntityRef = ApplicationEntityRef<AftermathFixtureSchema, FixtureEntity>;
type FixtureBuilder = ApplicationSchemaDeclarationBuilder<AftermathFixtureSchema>;

fn fixture_policy() -> ApplicationPolicyRef<AftermathFixtureSchema, FixturePolicy> {
    ApplicationPolicyRef::from_schema_identifier("FixturePolicy")
}

fn bind_entity_shape(entity: FixtureEntityRef, reads: &FixtureReads) -> FixtureBuilder {
    ApplicationSchemaDeclarationBuilder::<AftermathFixtureSchema>::for_schema()
        .entity(entity)
        .aspect(
            entity,
            ApplicationAspectRef::<AftermathFixtureSchema, FixtureEntity, IdentityAspect>::from_schema_identifier(
                "IdentityAspect",
            ),
        )
        .field(
            entity,
            ApplicationFieldRef::<
                AftermathFixtureSchema,
                FixtureEntity,
                IdentityAspect,
                ExternalIdentityField,
                WorthQueryExternalPrincipalIdentity,
                ReadOnly,
                EqualityPredicate,
            >::from_schema_identifiers(
                "FixtureEntity",
                "IdentityAspect",
                "ExternalIdentityField",
            ),
        )
        .field(
            entity,
            ApplicationFieldRef::<
                AftermathFixtureSchema,
                FixtureEntity,
                IdentityAspect,
                MappingStatusField,
                WorthQueryPrincipalMappingStatus,
                ReadWrite,
                NoEqualityPredicate,
            >::from_schema_identifiers(
                "FixtureEntity",
                "IdentityAspect",
                "MappingStatusField",
            ),
        )
        .field(entity, principal_read())
        .field(entity, reads.frozen)
        .field(entity, reads.note)
        .field(entity, reads.balance)
}

fn bind_authorization_shape(
    schema: FixtureBuilder,
    entity: FixtureEntityRef,
    ability: ApplicationAbilityRef<AftermathFixtureSchema, FixtureAbility, FixtureEntity>,
) -> FixtureBuilder {
    schema
        .relation(
            ApplicationRelationRef::<
                AftermathFixtureSchema,
                MappingTarget,
                FixtureEntity,
                FixtureEntity,
            >::from_schema_identifiers(
                "MappingTarget", "FixtureEntity", "FixtureEntity"
            ),
            entity,
            entity,
        )
        .principal_binding(fixture_principal_binding())
        .policy(fixture_policy())
        .ability(ability)
        .ability_policy(
            ability,
            fixture_policy(),
            [ApplicationAuthorizationPathBuilder::from_principal(entity).allow(entity)],
        )
}

fn fixture_principal_binding() -> ApplicationPrincipalBindingRef<
    AftermathFixtureSchema,
    PrincipalBinding,
    FixtureEntity,
    FixtureEntity,
    u64,
> {
    let identity = ApplicationFieldRef::<
        AftermathFixtureSchema,
        FixtureEntity,
        IdentityAspect,
        ExternalIdentityField,
        WorthQueryExternalPrincipalIdentity,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers(
        "FixtureEntity", "IdentityAspect", "ExternalIdentityField"
    );
    let status =
        ApplicationFieldRef::<
            AftermathFixtureSchema,
            FixtureEntity,
            IdentityAspect,
            MappingStatusField,
            WorthQueryPrincipalMappingStatus,
            ReadWrite,
            NoEqualityPredicate,
        >::from_schema_identifiers("FixtureEntity", "IdentityAspect", "MappingStatusField");
    let target = ApplicationRelationRef::<
        AftermathFixtureSchema,
        MappingTarget,
        FixtureEntity,
        FixtureEntity,
    >::from_schema_identifiers("MappingTarget", "FixtureEntity", "FixtureEntity");
    let principal_identity = ApplicationFieldRef::<
        AftermathFixtureSchema,
        FixtureEntity,
        IdentityAspect,
        PrincipalIdentityField,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers(
        "FixtureEntity", "IdentityAspect", "PrincipalIdentityField"
    );
    ApplicationPrincipalBindingRef::from_requirements(
        "PrincipalBinding",
        ApplicationPrincipalBindingRequirements {
            mapping_identity: ApplicationPrincipalMappingIdentityRequirement::from_field(identity),
            mapping_status: ApplicationPrincipalMappingStatusRequirement::from_field(status),
            target: ApplicationPrincipalTargetRequirement::from_relation(target),
            principal_identity: ApplicationPrincipalIdentityRequirement::from_field(
                principal_identity,
            ),
        },
    )
}
