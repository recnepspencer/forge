use worth_query_decl::facade::{worth_query_aspect, worth_query_entity};

struct StableSchema;

worth_query_entity!(StableEntity in StableSchema);
worth_query_aspect!(pub StableAspect in StableSchema, StableEntity);

fn main() {}
