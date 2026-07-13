use worth_query::facade::runtime::TypedDetailQueryBuilder;

worth_query::worth_query_schema! {
    pub schema UserSchema("user") {
        fields {
            pub field UserId("identity", "id", String) => [projectable, equality(String)];
        }
        relations {}
    }
}

worth_query::worth_query_schema! {
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
