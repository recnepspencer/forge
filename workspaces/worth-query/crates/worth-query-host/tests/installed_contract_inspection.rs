use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolVersion, CanonicalFieldPath, FieldKey,
};
use worth_query_decl::facade::{
    application_aftermath::{
        DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract, DeclaredCompensation,
        DeclaredCorrectionMechanism, DeclaredReconciliationProcedure,
    },
    application_schema::{
        ApplicationEffectPayload, ApplicationExternalEffectPayload,
        ApplicationExternalEffectProtocol, AspectContractRevision, AspectIdentity,
        WorthQueryExternalEffectCorrelationFamily,
    },
    worth_query_application_schema, worth_query_aspect, worth_query_effect, worth_query_entity,
    worth_query_field, worth_query_operation, worth_query_operation_emits,
    worth_query_operation_links, worth_query_operation_reads, worth_query_operation_writes,
    worth_query_relation,
};
use worth_query_host::facade::domain::{
    InstalledCorrectionAuthority, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity,
    WorthQueryInstalledApplicationSchema, WorthQueryOperationGraphReadScope,
    WorthQueryOperationTouchScope, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
};

const CORRELATION_FAMILY: &str = "contract-inspection-rail";
const RECONCILIATION_SLOT: &str = "reconcile-contract-inspection";

worth_query_application_schema! {
    pub schema ContractInspectionSchema {
        owner: contract_inspection,
        version: (1, 0),
        members: |schema| {
            schema
                .entity(Account::reference())
                .aspect(Account::reference(), AccountState::reference())
                .aspect(Account::reference(), AccountAudit::reference())
                .field(Account::reference(), AccountStatus::reference())
                .field(Account::reference(), AccountBalance::reference())
                .field(Account::reference(), AccountLimit::reference())
                .field(Account::reference(), AuditSequence::reference())
                .relation(ObservedAccount::reference(), Account::reference(), Account::reference())
                .relation(ChangedAccount::reference(), Account::reference(), Account::reference())
                .effect(AccountNoticeEffect::reference())
                .operation(
                    UpdateAccount::reference()
                        .definition()
                        .no_external_effect()
                        .no_aftermath()
                        .finish(),
                )
                .operation_decision_fact_budget(UpdateAccount::reference(), 3)
                .operation_projection_work_budget(UpdateAccount::reference(), 32)
                .operation_read_entity(UpdateAccount::reference(), Account::reference())
                .operation_read_field(UpdateAccount::reference(), AccountStatus::reference())
                .operation_read_relation(UpdateAccount::reference(), ObservedAccount::reference())
                .operation_write(UpdateAccount::reference(), AccountBalance::reference())
                .operation_link(UpdateAccount::reference(), ChangedAccount::reference())
                .operation(
                    EmitAccountNotice::reference()
                        .definition()
                        .external_effect(
                            AccountNoticeEffect::reference(),
                            correlation_family(),
                        )
                        .aftermath(external_owner_aftermath())
                        .finish(),
                )
                .operation_decision_fact_budget(EmitAccountNotice::reference(), 1)
                .operation_projection_work_budget(EmitAccountNotice::reference(), 1)
                .operation_emit(
                    EmitAccountNotice::reference(),
                    AccountNoticeEffect::reference(),
                )
        }
    }
}

worth_query_entity!(pub Account in ContractInspectionSchema);
worth_query_aspect!(pub AccountState in ContractInspectionSchema, Account; identity = AspectIdentity(0x9161_1051), revision = AspectContractRevision(2),);
worth_query_aspect!(pub AccountAudit in ContractInspectionSchema, Account; identity = AspectIdentity(0x9161_1052), revision = AspectContractRevision(1),);
worth_query_field!(pub AccountStatus in ContractInspectionSchema, Account, AccountState: u64, read_only, equality);
worth_query_field!(pub AccountBalance in ContractInspectionSchema, Account, AccountState: u64, read_write, equality);
worth_query_field!(pub AccountLimit in ContractInspectionSchema, Account, AccountState: u64, read_only, equality);
worth_query_field!(pub AuditSequence in ContractInspectionSchema, Account, AccountAudit: u64, read_only, equality);
worth_query_relation!(pub ObservedAccount in ContractInspectionSchema, Account => Account);
worth_query_relation!(pub ChangedAccount in ContractInspectionSchema, Account => Account);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateAccountInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmitAccountNoticeInput;

worth_query_operation!(pub UpdateAccount(UpdateAccountInput) in ContractInspectionSchema);
worth_query_operation_reads!(UpdateAccount => [Account, AccountStatus, ObservedAccount]);
worth_query_operation_writes!(UpdateAccount => [AccountBalance]);
worth_query_operation_links!(UpdateAccount => [ChangedAccount]);

worth_query_operation!(pub EmitAccountNotice(EmitAccountNoticeInput) in ContractInspectionSchema);

#[derive(Clone, Copy)]
pub struct AccountNotice(u64);

impl ApplicationEffectPayload for AccountNotice {
    fn retained_bytes(&self) -> u64 {
        8
    }
}

impl ApplicationExternalEffectPayload for AccountNotice {
    const PROTOCOL: ApplicationExternalEffectProtocol = ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.contract-inspection"),
        BoundaryProtocolVersion::new(1),
    );
    const MAX_EXTERNAL_BYTES: u64 = 8;

    fn external_effect_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

worth_query_effect!(pub AccountNoticeEffect(AccountNotice) in ContractInspectionSchema);
worth_query_operation_emits!(EmitAccountNotice => [AccountNoticeEffect]);

fn correlation_family() -> WorthQueryExternalEffectCorrelationFamily {
    WorthQueryExternalEffectCorrelationFamily::new(CORRELATION_FAMILY)
        .expect("the test correlation family is an atomic identity")
}

fn external_owner_aftermath() -> DeclaredApplicationAftermathContract<ContractInspectionSchema> {
    DeclaredApplicationAftermathContract::runtime_with_external_owner(
        DeclaredCorrectionMechanism::Compensation(
            DeclaredCompensation::new(
                RECONCILIATION_SLOT,
                DeclaredAftermathPostcondition::BusinessPostcondition {
                    identity: "contract-inspected".into(),
                },
            )
            .expect("the compensation declaration is valid"),
        ),
        DeclaredReconciliationProcedure::new(RECONCILIATION_SLOT)
            .expect("the reconciliation procedure is valid"),
    )
}

#[test]
fn host_consumer_inspects_one_exact_typed_installed_contract() {
    let declaration = ContractInspectionSchema::declaration()
        .expect("the public declaration facade should build the schema");
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "contract_inspection",
        1,
        0,
    ))
    .application_schema(declaration.clone())
    .validate()
    .expect("the public package should validate");
    let admitted = WorthQueryInstallationAdmissionProfile::new("host", "contract-inspection")
        .admit(package)
        .expect("the public package should admit");
    let index = worth_query_host::facade::domain::WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .expect("the public package should install");
    let schema = index
        .bind_application_schema(declaration)
        .expect("the typed schema should bind");

    assert_native_catalog(&schema);
    assert_mutation_read_contract(&schema);
    assert_mutation_touch_contract(&schema);
    assert_emit_only_aftermath_contract(&schema);
}

fn assert_native_catalog(schema: &WorthQueryInstalledApplicationSchema<ContractInspectionSchema>) {
    let catalog = schema.native_contracts();
    assert_eq!(catalog.len(), 2);
    let state_contract = catalog
        .aspect("Account", "AccountState")
        .expect("the declared state aspect must be retained");
    assert_eq!(
        state_contract.contract().identity(),
        AspectIdentity(0x9161_1051)
    );
    assert_eq!(
        state_contract.contract().revision(),
        AspectContractRevision(2)
    );
    assert_eq!(state_contract.fields().len(), 3);
}

fn assert_mutation_read_contract(
    schema: &WorthQueryInstalledApplicationSchema<ContractInspectionSchema>,
) {
    let state_contract = schema
        .native_contracts()
        .aspect("Account", "AccountState")
        .expect("the declared state aspect must be retained");
    let mutation = schema
        .installed_operation(UpdateAccount::reference())
        .expect("the mutation operation should install");
    let reads = mutation.contracts().graph_reads().roles()[0].read_scopes();
    assert_eq!(reads.len(), 3);
    assert!(reads.iter().any(|scope| matches!(
        scope,
        WorthQueryOperationGraphReadScope::Entity(scope)
            if scope.schema() == mutation.binding_identity() && scope.semantic_key() == "Account"
    )));
    let projection = reads
        .iter()
        .find_map(|scope| match scope {
            WorthQueryOperationGraphReadScope::NativeProjection(scope) => Some(scope),
            _ => None,
        })
        .expect("the field decision read must retain a native projection");
    assert_eq!(
        projection.projection().contract(),
        state_contract.contract()
    );
    assert!(!projection.projection().mask().is_whole_aspect());
    assert_eq!(
        projection.projection().mask().paths(),
        &[CanonicalFieldPath::single(
            FieldKey::new("AccountStatus").expect("the field key is valid")
        )]
    );
    assert!(reads.iter().any(|scope| matches!(
        scope,
        WorthQueryOperationGraphReadScope::Relation(scope)
            if scope.relation() == "ObservedAccount"
                && scope.from() == "Account"
                && scope.to() == "Account"
    )));
}

fn assert_mutation_touch_contract(
    schema: &WorthQueryInstalledApplicationSchema<ContractInspectionSchema>,
) {
    let state_contract = schema
        .native_contracts()
        .aspect("Account", "AccountState")
        .expect("the declared state aspect must be retained");
    let mutation = schema
        .installed_operation(UpdateAccount::reference())
        .expect("the mutation operation should install");
    let touches = mutation.contracts().touches().scopes();
    assert_eq!(touches.len(), 2);
    assert!(touches.iter().any(|scope| matches!(
        scope,
        WorthQueryOperationTouchScope::WriteField(scope)
            if scope.contract() == state_contract.contract()
                && scope.field_path()
                    == &CanonicalFieldPath::single(
                        FieldKey::new("AccountBalance").expect("the field key is valid")
                    )
    )));
    assert!(touches.iter().any(|scope| matches!(
        scope,
        WorthQueryOperationTouchScope::LinkRelation(scope)
            if scope.relation() == "ChangedAccount"
                && scope.from() == "Account"
                && scope.to() == "Account"
    )));
}

fn assert_emit_only_aftermath_contract(
    schema: &WorthQueryInstalledApplicationSchema<ContractInspectionSchema>,
) {
    let emit_only = schema
        .installed_operation(EmitAccountNotice::reference())
        .expect("the emit-only operation should install");
    assert!(emit_only.contracts().touches().scopes().is_empty());
    assert_eq!(emit_only.contracts().emissions().emissions().len(), 1);
    assert_eq!(
        emit_only.contracts().emissions().emissions()[0].effect(),
        "AccountNoticeEffect"
    );
    assert_eq!(
        emit_only.contracts().external_effect().correlation_family(),
        Some(&correlation_family())
    );
    let aftermath = emit_only
        .contracts()
        .aftermath()
        .expect("the external-owner aftermath should install");
    assert_eq!(
        aftermath.authority(),
        InstalledCorrectionAuthority::RuntimeWithExternalOwner
    );
    assert_eq!(
        aftermath
            .reconciliation()
            .expect("the exact reconciliation procedure must be retained")
            .procedure_slot(),
        RECONCILIATION_SLOT
    );
    assert_eq!(
        aftermath.external_effect().correlation_family(),
        emit_only.contracts().external_effect().correlation_family()
    );
}
