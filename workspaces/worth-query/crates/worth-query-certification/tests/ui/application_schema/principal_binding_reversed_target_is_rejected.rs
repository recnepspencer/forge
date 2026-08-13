use worth_query_decl::facade::application_schema::{
    ApplicationPrincipalTargetRequirement, ApplicationRelationRef,
};

struct Schema;
struct Mapping;
struct Principal;
struct Target;

fn reverse_target_relation(reversed: ApplicationRelationRef<Schema, Target, Principal, Mapping>) {
    let _ = ApplicationPrincipalTargetRequirement::<Schema, Mapping, Principal>::from_relation(
        reversed,
    );
}

fn main() {}
