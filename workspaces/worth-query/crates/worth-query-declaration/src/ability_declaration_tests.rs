use crate::facade::application_schema::{ApplicationSchemaMember, OperationRequiresAbility};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferInput;

worth_query_entity!(pub Account in AbilitySchema);
worth_query_operation!(pub TransferOperation(TransferInput) in AbilitySchema);
worth_query_ability!(pub SendMoney scoped_to Account, in AbilitySchema);
worth_query_operation_requires!(TransferOperation => [SendMoney]);

worth_query_application_schema! {
    pub schema AbilitySchema {
        owner: "WORTH.tests.ability",
        version: (1, 0),
        members: |schema| {
            schema
                .entity(Account::reference())
                .operation(TransferOperation::reference())
                .ability(SendMoney::reference())
                .operation_requires_ability(
                    TransferOperation::reference(),
                    SendMoney::reference(),
                )
        }
    }
}

#[test]
fn typed_ability_and_operation_requirement_enter_canonical_schema_meaning() {
    let declaration = AbilitySchema::declaration().expect("ability schema must declare");
    assert!(declaration.erased().members().iter().any(|member| matches!(
        member,
        ApplicationSchemaMember::Ability {
            ability,
            scope_entity,
        } if ability == "SendMoney" && scope_entity == "Account"
    )));
    assert!(declaration.erased().members().iter().any(|member| matches!(
        member,
        ApplicationSchemaMember::OperationAbility {
            operation,
            ability,
            scope_entity,
        } if operation == "TransferOperation"
            && ability == "SendMoney"
            && scope_entity == "Account"
    )));
}

#[test]
fn operation_requirement_is_a_compile_time_relationship() {
    fn requires<Operation, Ability: OperationRequiresAbility<Operation>>() {}
    requires::<TransferOperation, SendMoney>();
}
