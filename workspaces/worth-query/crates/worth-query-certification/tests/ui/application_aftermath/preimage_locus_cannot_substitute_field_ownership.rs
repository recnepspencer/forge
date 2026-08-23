use worth_query_decl::facade::application_schema::ApplicationFieldRef;
use worth_query_decl::facade::{worth_query_aspect, worth_query_entity, worth_query_field};

struct Schema;
worth_query_entity!(Account in Schema);
worth_query_aspect!(State in Schema, Account; identity = AspectIdentity(0x91611028), revision = AspectContractRevision(1),);
worth_query_field!(Status in Schema, Account, State: u64, read_only, no_equality);

worth_query_entity!(OtherAccount in Schema);
worth_query_aspect!(OtherState in Schema, OtherAccount; identity = AspectIdentity(0x91611029), revision = AspectContractRevision(1),);

fn main() {
    let _ =
        ApplicationFieldRef::<Schema, OtherAccount, OtherState, Status, u64>::from_schema_types();
}
