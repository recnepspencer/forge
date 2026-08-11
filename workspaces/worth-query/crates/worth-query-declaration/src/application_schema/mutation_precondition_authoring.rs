use super::capabilities::{ApplicationFieldUnit, OperationExpectsFact, OperationExpectsVersion};
use super::field_reference::ApplicationFieldRef;
use super::references::ApplicationOperationRef;
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
        Unit,
    >(
        self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Self
    where
        Field: OperationExpectsVersion<Operation>,
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
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
        Unit,
    >(
        self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Self
    where
        Field: OperationExpectsFact<Operation>,
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
    {
        self.precondition(
            operation.name(),
            ApplicationMutationPreconditionFamily::ExpectedFact,
            field,
        )
    }

    fn precondition<Entity, Aspect, Field, Value, Write, Equality, Unit>(
        self,
        operation: &str,
        family: ApplicationMutationPreconditionFamily,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
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
