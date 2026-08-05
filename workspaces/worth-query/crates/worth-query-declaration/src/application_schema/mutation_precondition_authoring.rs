use super::capabilities::{
    ApplicationFieldCurrency, OperationExpectsFact, OperationExpectsVersion,
};
use super::references::{ApplicationFieldRef, ApplicationOperationRef};
use super::values::TypedApplicationValue;
use super::{
    ApplicationMutationPreconditionFamily, ApplicationMutationPreconditionTarget,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaMember,
};

impl<Schema> ApplicationSchemaDeclarationBuilder<Schema> {
    pub fn operation_expected_version<
        Operation,
        Input,
        Entity,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
    >(
        self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Field: OperationExpectsVersion<Operation>,
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
    {
        self.precondition(
            operation.name(),
            ApplicationMutationPreconditionFamily::ExpectedVersion,
            field,
        )
    }

    pub fn operation_expected_fact<
        Operation,
        Input,
        Entity,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
    >(
        self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Field: OperationExpectsFact<Operation>,
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
    {
        self.precondition(
            operation.name(),
            ApplicationMutationPreconditionFamily::ExpectedFact,
            field,
        )
    }

    fn precondition<Entity, Aspect, Field, Value, Write, Equality, Currency>(
        self,
        operation: &str,
        family: ApplicationMutationPreconditionFamily,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
    {
        self.push_member(ApplicationSchemaMember::OperationMutationPrecondition {
            operation: operation.to_owned(),
            target: ApplicationMutationPreconditionTarget::field(
                family,
                field.entity(),
                field.aspect(),
                field.field(),
            ),
        })
    }
}
