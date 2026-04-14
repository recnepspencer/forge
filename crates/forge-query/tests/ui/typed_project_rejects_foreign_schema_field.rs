use forge_query::facade::TypedDetailQueryBuilder;

forge_query::forge_query_schema! {
    pub schema UserSchema("user") {
        fields {
            pub field UserId("identity", "id", String) => [projectable, equality(String)];
        }
        relations {}
    }
}

forge_query::forge_query_schema! {
    pub schema TaskSchema("task") {
        fields {
            pub field TaskTitle("text", "title", String) => [projectable, equality(String)];
        }
        relations {}
    }
}

fn main() {
    let _ = TypedDetailQueryBuilder::<UserSchema>::new().project::<TaskTitle>();
}
