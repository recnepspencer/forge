use worth_query::facade::runtime::TypedDetailQueryBuilder;

worth_query::worth_query_schema! {
    pub schema UserSchema("user") {
        fields {
            pub field Age("profile", "age", Int64) => [projectable, equality(i64), orderable];
        }
        relations {}
    }
}

fn main() {
    let _ = TypedDetailQueryBuilder::<UserSchema>::new()
        .project::<Age>()
        .where_contains::<Age>("est");
}
