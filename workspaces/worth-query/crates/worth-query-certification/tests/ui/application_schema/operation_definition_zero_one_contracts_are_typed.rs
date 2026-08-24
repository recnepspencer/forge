use worth_query_decl::facade::application_aftermath::DeclaredApplicationAftermathContract;
use worth_query_decl::facade::application_schema::{
    ApplicationEffectPayload, ApplicationExternalEffectPayload,
    ApplicationExternalEffectProtocol, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaDeclarationDenial,
    WorthQueryExternalEffectCorrelationFamily,
};
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_decl::facade::{
    worth_query_effect, worth_query_operation, worth_query_operation_emits,
};

struct Schema;
struct Input;
struct Payload;

impl ApplicationEffectPayload for Payload {
    fn retained_bytes(&self) -> u64 {
        1
    }
}

impl ApplicationExternalEffectPayload for Payload {
    const PROTOCOL: ApplicationExternalEffectProtocol = ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.operation-effect"),
        BoundaryProtocolVersion::new(1),
    );
    const MAX_EXTERNAL_BYTES: u64 = 1;

    fn external_effect_bytes(&self) -> Vec<u8> {
        vec![1]
    }
}

worth_query_operation!(NoContracts(Input) in Schema);
worth_query_operation!(AftermathOnly(Input) in Schema);
worth_query_operation!(ExternalOnly(Input) in Schema);
worth_query_operation!(BothContracts(Input) in Schema);
worth_query_effect!(Effect(Payload) in Schema);
worth_query_operation_emits!(ExternalOnly => [Effect]);
worth_query_operation_emits!(BothContracts => [Effect]);

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "owner";
    const NAME: &'static str = "Schema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        let external_only = ExternalOnly::reference();
        let both = BothContracts::reference();
        ApplicationSchemaDeclarationBuilder::for_schema()
            .effect(Effect::reference())
            .operation(
                NoContracts::reference()
                    .definition()
                    .no_external_effect()
                    .no_aftermath()
                    .finish(),
            )
            .operation(
                AftermathOnly::reference()
                    .definition()
                    .no_external_effect()
                    .aftermath(DeclaredApplicationAftermathContract::not_correctable())
                    .finish(),
            )
            .operation(
                external_only
                    .definition()
                    .external_effect(
                        Effect::reference(),
                        WorthQueryExternalEffectCorrelationFamily::new("rail").unwrap(),
                    )
                    .no_aftermath()
                    .finish(),
            )
            .operation_emit(external_only, Effect::reference())
            .operation(
                both
                    .definition()
                    .external_effect(
                        Effect::reference(),
                        WorthQueryExternalEffectCorrelationFamily::new("rail").unwrap(),
                    )
                    .aftermath(DeclaredApplicationAftermathContract::not_correctable())
                    .finish(),
            )
            .operation_emit(both, Effect::reference())
            .build()
    }
}

fn main() {
    Schema::declaration().unwrap();
}
