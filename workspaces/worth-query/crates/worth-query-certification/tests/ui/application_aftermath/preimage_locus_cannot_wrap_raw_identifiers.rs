use worth_query_decl::facade::application_schema::ApplicationFieldRef;
use worth_query_decl::facade::{worth_query_aspect, worth_query_entity, worth_query_field};

struct Schema;
worth_query_entity!(Account in Schema);
worth_query_aspect!(State in Schema, Account);
worth_query_field!(Status in Schema, Account, State: u64, read_only, no_equality);

fn main() {
    let _ = ApplicationFieldRef::<Schema, Account, State, Status, u64>::from_schema_identifiers(
        "OtherEntity",
        "OtherAspect",
        "OtherField",
    );
}
