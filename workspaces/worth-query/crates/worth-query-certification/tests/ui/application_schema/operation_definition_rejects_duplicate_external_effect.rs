use worth_query_decl::facade::{
    application_schema::{
        ApplicationEffectPayload, ApplicationExternalEffectPayload,
        ApplicationExternalEffectProtocol,
    },
    worth_query_effect, worth_query_operation, worth_query_operation_emits,
};
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

struct Schema;
struct Input;
struct Payload;
worth_query_operation!(Operation(Input) in Schema);
worth_query_effect!(Effect(Payload) in Schema);
worth_query_operation_emits!(Operation => [Effect]);

impl ApplicationEffectPayload for Payload {
    fn retained_bytes(&self) -> u64 {
        1
    }
}

impl ApplicationExternalEffectPayload for Payload {
    const PROTOCOL: ApplicationExternalEffectProtocol = ApplicationExternalEffectProtocol::new(
        BoundaryProtocolIdentity::new("test.duplicate-effect"),
        BoundaryProtocolVersion::new(1),
    );
    const MAX_EXTERNAL_BYTES: u64 = 1;

    fn external_effect_bytes(&self) -> Vec<u8> {
        vec![1]
    }
}

fn main() {
    let _ = Operation::reference()
        .definition()
        .no_external_effect()
        .external_effect(Effect::reference(), "rail");
}
