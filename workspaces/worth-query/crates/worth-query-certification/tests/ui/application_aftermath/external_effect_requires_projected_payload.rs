use worth_query_decl::facade::application_schema::{
    ApplicationEffectPayload, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaDeclarationDenial,
    WorthQueryExternalEffectCorrelationFamily,
};
use worth_query_decl::facade::{
    worth_query_effect, worth_query_operation, worth_query_operation_emits,
};

struct Schema;
struct Input;
struct InternalOnlyPayload;

impl ApplicationEffectPayload for InternalOnlyPayload {
    fn retained_bytes(&self) -> u64 {
        0
    }
}

worth_query_operation!(Operation(Input) in Schema);
worth_query_effect!(ExternalEffect(InternalOnlyPayload) in Schema);
worth_query_operation_emits!(Operation => [ExternalEffect]);

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "owner";
    const NAME: &'static str = "Schema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::for_schema()
            .operation(
                Operation::reference()
                    .definition()
                    .external_effect(
                        ExternalEffect::reference(),
                        WorthQueryExternalEffectCorrelationFamily::new("external-family").unwrap(),
                    )
                    .no_aftermath()
                    .finish(),
            )
            .build()
    }
}

fn main() {
    let _ = Schema::declaration();
}
