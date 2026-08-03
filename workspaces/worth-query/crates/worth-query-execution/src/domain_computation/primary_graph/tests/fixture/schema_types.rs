use super::*;

worth_query_entity!(pub ExternalMapping in IdentityExecutionSchema);
worth_query_entity!(pub Principal in IdentityExecutionSchema);
worth_query_entity!(pub Account in IdentityExecutionSchema);
worth_query_entity!(pub Activity in IdentityExecutionSchema);
worth_query_aspect!(pub ExternalIdentity in IdentityExecutionSchema, ExternalMapping);
worth_query_field!(
    pub ExternalIdentityField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryExternalPrincipalIdentity, read_only, equality
);
worth_query_aspect!(pub PrincipalIdentity in IdentityExecutionSchema, Principal);
worth_query_field!(
    pub PrincipalIdentityField in IdentityExecutionSchema, Principal, PrincipalIdentity:
    u64, read_only, equality
);
worth_query_field!(
    pub MappingStatusField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryPrincipalMappingStatus, read_write, equality
);
worth_query_relation!(
    pub MappingTarget in IdentityExecutionSchema,
    ExternalMapping => Principal
);
worth_query_principal_binding!(
    pub IdentityBinding in IdentityExecutionSchema,
    mapping ExternalMapping {
        identity: ExternalIdentityField,
        status: MappingStatusField,
        target: MappingTarget => Principal,
        principal_identity: PrincipalIdentityField
    }
);
worth_query_aspect!(pub AccountPolicy in IdentityExecutionSchema, Account);
worth_query_field!(
    pub AccountIdentity in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_only, equality
);
worth_query_field!(
    pub AccountStatus in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_write, equality
);
worth_query_field!(
    pub AccountLabel in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_write, equality
);
worth_query_aspect!(pub ActivityFacts in IdentityExecutionSchema, Activity);
worth_query_field!(
    pub ActivityIdentity in IdentityExecutionSchema, Activity, ActivityFacts:
    String, read_only, equality
);
worth_query_field!(
    pub ActivitySequence in IdentityExecutionSchema, Activity, ActivityFacts:
    u64, read_only, no_equality
);
worth_query_relation!(pub AccountOwner in IdentityExecutionSchema, Principal => Account);
worth_query_relation!(pub AccountBlocked in IdentityExecutionSchema, Principal => Account);
worth_query_relation!(pub AccountPrimaryActivity in IdentityExecutionSchema, Account => Activity);
worth_query_relation!(pub AccountSecondaryActivity in IdentityExecutionSchema, Account => Activity);
worth_query_relation!(pub AccountAllActivity in IdentityExecutionSchema, Account => Activity);
worth_query_relation!(pub ActivityAccount in IdentityExecutionSchema, Activity => Account);
worth_query_ability!(pub ViewAccount scoped_to Account, in IdentityExecutionSchema);
worth_query_ability!(pub EditAccount scoped_to Account, in IdentityExecutionSchema);
worth_query_ability!(pub ManageOwnership scoped_to Principal, in IdentityExecutionSchema);
worth_query_policy!(pub AccountAccessPolicy in IdentityExecutionSchema);
worth_query_effect!(pub AccountActivityEffect(String) in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchAccountInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiTouchInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeOwnershipInput;

worth_query_operation!(
    pub TouchAccountOperation(TouchAccountInput) in IdentityExecutionSchema
);
worth_query_operation!(
    pub MultiTouchOperation(MultiTouchInput) in IdentityExecutionSchema
);
worth_query_operation!(
    pub ChangeOwnershipOperation(ChangeOwnershipInput) in IdentityExecutionSchema
);
worth_query_operation_requires!(TouchAccountOperation => [ViewAccount]);
worth_query_operation_expects_fact!(TouchAccountOperation => [AccountStatus]);
worth_query_operation_requires!(MultiTouchOperation => [ViewAccount, EditAccount]);
worth_query_operation_requires!(ChangeOwnershipOperation => [ManageOwnership]);
worth_query_operation_writes!(TouchAccountOperation => [AccountStatus, AccountLabel]);
// Deliberately wider than the installed contract so authority-ceiling tests
// prove that compile-time capability cannot widen installed authority.
worth_query_operation_writes!(MultiTouchOperation => [AccountStatus, AccountLabel]);
worth_query_operation_emits!(
    TouchAccountOperation => [AccountActivityEffect, LiveActivityEffect]
);
worth_query_operation_reads!(TouchAccountOperation => [AccountStatus, AccountLabel, AccountOwner]);
worth_query_operation_reads!(MultiTouchOperation => [AccountStatus, AccountLabel]);
worth_query_operation_reads!(ChangeOwnershipOperation => [AccountOwner, AccountStatus]);
worth_query_operation_links!(ChangeOwnershipOperation => [AccountOwner]);
worth_query_operation_unlinks!(ChangeOwnershipOperation => [AccountOwner]);

pub(in crate::domain_computation::primary_graph) type InstalledIdentityBinding =
    WorthQueryInstalledPrincipalBinding<
        IdentityExecutionSchema,
        IdentityBinding,
        ExternalMapping,
        Principal,
        u64,
    >;

pub(in crate::domain_computation::primary_graph) struct IdentityWorld {
    pub(in crate::domain_computation::primary_graph) runtime: WorthQueryExecutionRuntime,
    pub(in crate::domain_computation::primary_graph) schema:
        WorthQueryInstalledApplicationSchema<IdentityExecutionSchema>,
    pub(in crate::domain_computation::primary_graph) binding: InstalledIdentityBinding,
    pub(in crate::domain_computation::primary_graph) publication: WorthQueryPrimaryGraphPublication,
}

pub(in crate::domain_computation::primary_graph) fn external_identity(
    subject: &str,
) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new("https://issuer.example", subject).unwrap()
}

pub(in crate::domain_computation::primary_graph) fn live_scope() -> WorthQueryRequestScope {
    let source = WorthQueryCancellationSource::new();
    WorthQueryRequestScope::new(Instant::now() + Duration::from_secs(60), source.token())
}
