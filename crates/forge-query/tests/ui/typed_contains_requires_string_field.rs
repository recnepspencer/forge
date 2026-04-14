use forge_query::facade::TypedDetailQueryBuilder;

forge_query::forge_query_schema! {
    pub schema UserSchema("user") {
        fields {
            pub field Age("profile", "age", Integer) => [projectable, equality(i64), orderable];
        }
        relations {}
    }
}

fn main() {
    let _ = TypedDetailQueryBuilder::<UserSchema>::new()
        .project::<Age>()
        .where_contains::<Age>("est");
}
